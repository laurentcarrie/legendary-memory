//! Storage module for S3 and local filesystem operations.
//!
//! This module provides abstractions for working with both local paths and S3 URLs,
//! allowing the build system to read source files from S3 and write output to S3.

use aws_config::BehaviorVersion;
use aws_credential_types::provider::ProvideCredentials;
use futures::TryStreamExt;
use object_store::aws::AmazonS3Builder;
use object_store::path::Path as ObjectPath;
use object_store::{ObjectStore, ObjectStoreExt, PutPayload};
use std::path::{Path, PathBuf};

/// Represents a storage path that can be either local or S3.
#[derive(Debug, Clone)]
pub enum StoragePath {
    Local(PathBuf),
    S3 { bucket: String, prefix: String },
}

impl StoragePath {
    /// Parse a string into a StoragePath.
    /// S3 URLs should be in the format `s3://bucket/prefix`.
    pub fn parse(s: &str) -> Result<Self, String> {
        if let Some(without_scheme) = s.strip_prefix("s3://") {
            let parts: Vec<&str> = without_scheme.splitn(2, '/').collect();
            if parts.is_empty() || parts[0].is_empty() {
                return Err("Invalid S3 URL: missing bucket name".to_string());
            }
            let bucket = parts[0].to_string();
            let prefix = if parts.len() > 1 { parts[1] } else { "" }.to_string();
            Ok(StoragePath::S3 { bucket, prefix })
        } else {
            Ok(StoragePath::Local(PathBuf::from(s)))
        }
    }

    /// Check if this is an S3 path.
    pub fn is_s3(&self) -> bool {
        matches!(self, StoragePath::S3 { .. })
    }

    /// Check if this is a local path.
    pub fn is_local(&self) -> bool {
        matches!(self, StoragePath::Local(_))
    }

    /// Get the local path if this is a local storage path.
    pub fn as_local(&self) -> Option<&PathBuf> {
        match self {
            StoragePath::Local(p) => Some(p),
            StoragePath::S3 { .. } => None,
        }
    }
}

impl std::fmt::Display for StoragePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoragePath::Local(p) => write!(f, "{}", p.display()),
            StoragePath::S3 { bucket, prefix } => {
                if prefix.is_empty() {
                    write!(f, "s3://{bucket}")
                } else {
                    write!(f, "s3://{bucket}/{prefix}")
                }
            }
        }
    }
}

/// Create an S3 client for the given bucket.
/// Uses the AWS SDK credential chain which supports:
/// - Environment variables (AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY)
/// - AWS_PROFILE environment variable
/// - Shared credentials file (~/.aws/credentials)
/// - Instance metadata (EC2, ECS, etc.)
pub async fn create_s3_client(bucket: &str) -> Result<impl ObjectStore, String> {
    // Load AWS config using the SDK's credential chain (supports profiles)
    let sdk_config = aws_config::defaults(BehaviorVersion::latest()).load().await;

    let region = sdk_config
        .region()
        .map(|r| r.to_string())
        .unwrap_or_else(|| "us-east-1".to_string());

    // Get credentials from the SDK
    let credentials_provider = sdk_config
        .credentials_provider()
        .ok_or_else(|| "No credentials provider available".to_string())?;

    let credentials = credentials_provider
        .provide_credentials()
        .await
        .map_err(|e| format!("Failed to load AWS credentials: {e}"))?;

    AmazonS3Builder::new()
        .with_bucket_name(bucket)
        .with_region(&region)
        .with_access_key_id(credentials.access_key_id())
        .with_secret_access_key(credentials.secret_access_key())
        .with_token(credentials.session_token().unwrap_or_default())
        .build()
        .map_err(|e| format!("Failed to create S3 client: {e}"))
}

/// Download all files from an S3 path to a local directory.
pub async fn download_to_local(src: &StoragePath, dest: &Path) -> Result<(), String> {
    let (bucket, prefix) = match src {
        StoragePath::S3 { bucket, prefix } => (bucket, prefix),
        StoragePath::Local(_) => return Err("Source is not an S3 path".to_string()),
    };

    let store = create_s3_client(bucket).await?;
    let prefix_path = if prefix.is_empty() {
        None
    } else {
        Some(ObjectPath::from(prefix.as_str()))
    };

    // List all objects with the given prefix
    let list_result = store.list(prefix_path.as_ref());

    let objects: Vec<_> = list_result
        .try_collect()
        .await
        .map_err(|e| format!("Failed to list S3 objects: {e}"))?;

    log::info!(
        "Downloading {} objects from s3://{}/{}",
        objects.len(),
        bucket,
        prefix
    );

    for meta in objects {
        let key = meta.location.to_string();

        // Calculate the relative path from the prefix
        let relative_path = if prefix.is_empty() {
            key.clone()
        } else {
            key.strip_prefix(prefix)
                .map(|s| s.trim_start_matches('/'))
                .unwrap_or(&key)
                .to_string()
        };

        if relative_path.is_empty() {
            continue;
        }

        let local_path = dest.join(&relative_path);

        // Create parent directories if needed
        if let Some(parent) = local_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory {}: {e}", parent.display()))?;
        }

        // Download the object
        let data = store
            .get(&meta.location)
            .await
            .map_err(|e| format!("Failed to get object {key}: {e}"))?;

        let bytes = data
            .bytes()
            .await
            .map_err(|e| format!("Failed to read object {key}: {e}"))?;

        std::fs::write(&local_path, &bytes)
            .map_err(|e| format!("Failed to write {}: {e}", local_path.display()))?;

        log::debug!("Downloaded {} -> {}", key, local_path.display());
    }

    Ok(())
}

/// Upload specific paths (files or directories) to an S3 path.
/// Each path is uploaded relative to the base_dir.
pub async fn upload_paths_to_s3(
    paths: &[PathBuf],
    base_dir: &Path,
    dest: &StoragePath,
) -> Result<(), String> {
    let (bucket, prefix) = match dest {
        StoragePath::S3 { bucket, prefix } => (bucket, prefix),
        StoragePath::Local(_) => return Err("Destination is not an S3 path".to_string()),
    };

    let store = create_s3_client(bucket).await?;

    // Collect all files to upload (expand directories)
    let mut files_to_upload = Vec::new();
    for path in paths {
        let full_path = base_dir.join(path);
        if full_path.is_dir() {
            files_to_upload.extend(walkdir(&full_path)?);
        } else if full_path.is_file() {
            files_to_upload.push(full_path);
        }
    }

    log::info!(
        "Uploading {} files to s3://{}/{}",
        files_to_upload.len(),
        bucket,
        prefix
    );

    for local_path in files_to_upload {
        let relative = local_path
            .strip_prefix(base_dir)
            .map_err(|e| format!("Failed to strip prefix: {e}"))?;

        let key = if prefix.is_empty() {
            relative.to_string_lossy().to_string()
        } else {
            format!("{}/{}", prefix.trim_end_matches('/'), relative.display())
        };

        let data = std::fs::read(&local_path)
            .map_err(|e| format!("Failed to read {}: {e}", local_path.display()))?;

        let object_path = ObjectPath::from(key.as_str());
        store
            .put(&object_path, PutPayload::from(data))
            .await
            .map_err(|e| format!("Failed to upload {key}: {e}"))?;

        log::debug!("Uploaded {} -> {}", local_path.display(), key);
    }

    Ok(())
}

/// Upload all files from a local directory to an S3 path.
pub async fn upload_to_s3(src: &Path, dest: &StoragePath) -> Result<(), String> {
    let (bucket, prefix) = match dest {
        StoragePath::S3 { bucket, prefix } => (bucket, prefix),
        StoragePath::Local(_) => return Err("Destination is not an S3 path".to_string()),
    };

    let store = create_s3_client(bucket).await?;

    // Walk the local directory
    let entries = walkdir(src)?;

    log::info!(
        "Uploading {} files to s3://{}/{}",
        entries.len(),
        bucket,
        prefix
    );

    for local_path in entries {
        // Calculate the S3 key
        let relative = local_path
            .strip_prefix(src)
            .map_err(|e| format!("Failed to strip prefix: {e}"))?;

        let key = if prefix.is_empty() {
            relative.to_string_lossy().to_string()
        } else {
            format!("{}/{}", prefix.trim_end_matches('/'), relative.display())
        };

        // Read the local file
        let data = std::fs::read(&local_path)
            .map_err(|e| format!("Failed to read {}: {e}", local_path.display()))?;

        // Upload to S3
        let object_path = ObjectPath::from(key.as_str());
        store
            .put(&object_path, PutPayload::from(data))
            .await
            .map_err(|e| format!("Failed to upload {key}: {e}"))?;

        log::debug!("Uploaded {} -> {}", local_path.display(), key);
    }

    Ok(())
}

/// Recursively walk a directory and return all file paths.
fn walkdir(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = Vec::new();

    if !dir.exists() {
        return Ok(files);
    }

    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("Failed to read directory {}: {e}", dir.display()))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {e}"))?;
        let path = entry.path();

        if path.is_dir() {
            files.extend(walkdir(&path)?);
        } else {
            files.push(path);
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_local_path() {
        let path = StoragePath::parse("/home/user/songs").unwrap();
        assert!(path.is_local());
        assert!(!path.is_s3());
        assert_eq!(path.as_local().unwrap(), &PathBuf::from("/home/user/songs"));
    }

    #[test]
    fn test_parse_s3_path() {
        let path = StoragePath::parse("s3://my-bucket/path/to/songs").unwrap();
        assert!(path.is_s3());
        assert!(!path.is_local());
        match path {
            StoragePath::S3 { bucket, prefix } => {
                assert_eq!(bucket, "my-bucket");
                assert_eq!(prefix, "path/to/songs");
            }
            _ => panic!("Expected S3 path"),
        }
    }

    #[test]
    fn test_parse_s3_bucket_only() {
        let path = StoragePath::parse("s3://my-bucket").unwrap();
        match path {
            StoragePath::S3 { bucket, prefix } => {
                assert_eq!(bucket, "my-bucket");
                assert_eq!(prefix, "");
            }
            _ => panic!("Expected S3 path"),
        }
    }

    #[test]
    fn test_parse_s3_bucket_with_slash() {
        let path = StoragePath::parse("s3://my-bucket/").unwrap();
        match path {
            StoragePath::S3 { bucket, prefix } => {
                assert_eq!(bucket, "my-bucket");
                assert_eq!(prefix, "");
            }
            _ => panic!("Expected S3 path"),
        }
    }

    #[test]
    fn test_parse_invalid_s3() {
        let result = StoragePath::parse("s3://");
        assert!(result.is_err());
    }

    #[test]
    fn test_display_local() {
        let path = StoragePath::Local(PathBuf::from("/home/user/songs"));
        assert_eq!(format!("{path}"), "/home/user/songs");
    }

    #[test]
    fn test_display_s3() {
        let path = StoragePath::S3 {
            bucket: "my-bucket".to_string(),
            prefix: "path/to/songs".to_string(),
        };
        assert_eq!(format!("{path}"), "s3://my-bucket/path/to/songs");
    }

    #[test]
    fn test_display_s3_no_prefix() {
        let path = StoragePath::S3 {
            bucket: "my-bucket".to_string(),
            prefix: "".to_string(),
        };
        assert_eq!(format!("{path}"), "s3://my-bucket");
    }
}

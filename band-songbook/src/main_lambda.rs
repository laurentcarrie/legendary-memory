use band_songbook::make_all_with_storage;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

/// S3 Event notification structure
#[derive(Deserialize, Debug)]
struct S3Event {
    #[serde(rename = "Records", default)]
    records: Vec<S3EventRecord>,
}

#[derive(Deserialize, Debug)]
struct S3EventRecord {
    #[serde(rename = "eventSource")]
    event_source: Option<String>,
    #[serde(rename = "eventName")]
    event_name: Option<String>,
    s3: Option<S3Data>,
}

#[derive(Deserialize, Debug)]
struct S3Data {
    bucket: S3Bucket,
    object: S3Object,
}

#[derive(Deserialize, Debug)]
struct S3Bucket {
    name: String,
}

#[derive(Deserialize, Debug)]
struct S3Object {
    key: String,
}

#[derive(Serialize)]
struct Response {
    request_id: String,
    success: bool,
    message: String,
    triggered_by: Option<String>,
}

/// Configuration from environment variables
struct Config {
    srcdir: String,
    sandbox: String,
    settings: Option<String>,
}

impl Config {
    fn from_env() -> Result<Self, String> {
        let srcdir = std::env::var("SRCDIR")
            .unwrap_or_else(|_| "s3://zik-laurent/songs".to_string());
        let sandbox = std::env::var("SANDBOX")
            .unwrap_or_else(|_| "s3://zik-laurent/sandbox".to_string());
        let settings = std::env::var("SETTINGS").ok()
            .or_else(|| Some("s3://zik-laurent/songs/settings.yml".to_string()));

        Ok(Config {
            srcdir,
            sandbox,
            settings,
        })
    }
}

async fn function_handler(event: LambdaEvent<S3Event>) -> Result<Response, Error> {
    let request_id = event.context.request_id.clone();
    let s3_event = event.payload;

    // Log the trigger
    let triggered_by = if let Some(record) = s3_event.records.first() {
        if let Some(s3_data) = &record.s3 {
            let key = &s3_data.object.key;
            log::info!(
                "Triggered by S3 event: {} on s3://{}/{}",
                record.event_name.as_deref().unwrap_or("unknown"),
                s3_data.bucket.name,
                key
            );
            Some(format!("s3://{}/{}", s3_data.bucket.name, key))
        } else {
            log::info!("Triggered manually or by unknown event");
            None
        }
    } else {
        log::info!("Triggered manually (no S3 records)");
        None
    };

    // Get configuration from environment
    let config = Config::from_env().map_err(|e| Error::from(e))?;

    log::info!("srcdir: {}", &config.srcdir);
    log::info!("sandbox: {}", &config.sandbox);
    if let Some(ref settings) = config.settings {
        log::info!("settings: {}", settings);
    }

    match make_all_with_storage(
        &config.srcdir,
        &config.sandbox,
        config.settings.as_deref(),
        None, // no pattern filter
    )
    .await
    {
        Ok((success, _graph)) => {
            let message = if success {
                "Build completed successfully".to_string()
            } else {
                "Build completed with errors".to_string()
            };
            log::info!("{}", message);
            Ok(Response {
                request_id,
                success,
                message,
                triggered_by,
            })
        }
        Err(e) => {
            log::error!("Build failed: {}", e);
            Ok(Response {
                request_id,
                success: false,
                message: format!("Build failed: {}", e),
                triggered_by,
            })
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    log::info!("Starting band-songbook lambda...");

    run(service_fn(function_handler)).await
}

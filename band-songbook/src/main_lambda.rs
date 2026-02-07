use band_songbook::make_all_with_storage;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde::{Deserialize, Serialize};

/// S3 Event notification structure
#[derive(Deserialize, Debug)]
struct S3Event {
    #[serde(rename = "Records", default)]
    records: Vec<S3EventRecord>,
}

#[derive(Deserialize, Debug)]
struct S3EventRecord {
    #[serde(rename = "eventName")]
    event_name: Option<String>,
    s3: Option<serde_json::Value>,
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
    sandbox_base: String,
    settings: Option<String>,
}

impl Config {
    fn from_env() -> Result<Self, String> {
        let srcdir =
            std::env::var("SRCDIR").unwrap_or_else(|_| "s3://zik-laurent/songs".to_string());
        let sandbox_base =
            std::env::var("SANDBOX").unwrap_or_else(|_| "s3://zik-laurent/sandbox".to_string());
        let settings = std::env::var("SETTINGS")
            .ok()
            .or_else(|| Some("s3://zik-laurent/songs/settings.yml".to_string()));

        Ok(Config {
            srcdir,
            sandbox_base,
            settings,
        })
    }

    /// Generate a unique sandbox path for this invocation
    fn sandbox_for_request(&self, request_id: &str) -> String {
        // Use a short hash of request_id to keep path manageable
        let short_id: String = request_id.chars().take(8).collect();
        let timestamp = chrono::Utc::now().format("%Y%m%d-%H%M%S");
        format!("{}/{}-{}", self.sandbox_base, timestamp, short_id)
    }
}

async fn function_handler(event: LambdaEvent<S3Event>) -> Result<Response, Error> {
    let request_id = event.context.request_id.clone();
    let s3_event = event.payload;

    // Check if triggered by S3 event - ignore S3 triggers, only respond to manual invocations
    if let Some(record) = s3_event.records.first() {
        if record.s3.is_some() {
            log::info!(
                "Ignoring S3 event trigger: {}",
                record.event_name.as_deref().unwrap_or("unknown")
            );
            return Ok(Response {
                request_id,
                success: true,
                message: "S3 trigger ignored - use manual invocation".to_string(),
                triggered_by: None,
            });
        }
    }

    log::info!("Triggered manually");
    let triggered_by: Option<String> = None;

    // Get configuration from environment
    let config = Config::from_env().map_err(Error::from)?;

    // Generate unique sandbox path for this invocation
    let sandbox = config.sandbox_for_request(&request_id);

    log::info!("srcdir: {}", &config.srcdir);
    log::info!("sandbox: {}", &sandbox);
    if let Some(ref settings) = config.settings {
        log::info!("settings: {settings}");
    }

    // Use a fixed path for local sandbox (Lambda only has /tmp writable)
    let local_sandbox = std::path::Path::new("/tmp/sandbox");

    match make_all_with_storage(
        &config.srcdir,
        &sandbox,
        local_sandbox,
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
            log::info!("{message}");
            Ok(Response {
                request_id,
                success,
                message,
                triggered_by,
            })
        }
        Err(e) => {
            log::error!("Build failed: {e}");
            Ok(Response {
                request_id,
                success: false,
                message: format!("Build failed: {e}"),
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

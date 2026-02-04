use band_songbook::make_all_with_storage;
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Request {
    /// Source directory containing song.yml files (s3://bucket/prefix)
    srcdir: String,
    /// Output directory for built files (s3://bucket/prefix)
    sandbox: String,
    /// Optional path to settings.yml file (s3://bucket/key)
    #[serde(default)]
    settings: Option<String>,
    /// Optional pattern to filter songs
    #[serde(default)]
    pattern: Option<String>,
}

#[derive(Serialize)]
struct Response {
    request_id: String,
    success: bool,
    message: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let request = event.payload;
    let request_id = event.context.request_id.clone();

    log::info!("Processing request {}", request_id);
    log::info!("srcdir: {}", &request.srcdir);
    log::info!("sandbox: {}", &request.sandbox);
    if let Some(ref settings) = request.settings {
        log::info!("settings: {}", settings);
    }
    if let Some(ref pattern) = request.pattern {
        log::info!("pattern: {}", pattern);
    }

    match make_all_with_storage(
        &request.srcdir,
        &request.sandbox,
        request.settings.as_deref(),
        request.pattern.as_deref(),
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
            })
        }
        Err(e) => {
            log::error!("Build failed: {}", e);
            Ok(Response {
                request_id,
                success: false,
                message: format!("Build failed: {}", e),
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

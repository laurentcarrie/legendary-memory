use band_songbook::make_all_with_storage;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Request {
    srcdir: String,
    settings: String,
    delivery: String,
}

#[derive(Serialize)]
struct Response {
    request_id: String,
    success: bool,
    message: String,
}

async fn function_handler(event: LambdaEvent<Request>) -> Result<Response, Error> {
    let request_id = event.context.request_id.clone();
    let req = event.payload;

    // Use a fixed path for local sandbox (Lambda only has /tmp writable)
    let sandbox = std::path::Path::new("/tmp/sandbox");

    log::info!("srcdir: {}", &req.srcdir);
    log::info!("settings: {}", &req.settings);
    log::info!("delivery: {}", &req.delivery);
    log::info!("sandbox: {}", sandbox.display());

    match make_all_with_storage(
        &req.srcdir,
        sandbox,
        Some(req.settings.as_str()),
        None, // no pattern filter
        &req.delivery,
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
            })
        }
        Err(e) => {
            log::error!("Build failed: {e}");
            Ok(Response {
                request_id,
                success: false,
                message: format!("Build failed: {e}"),
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

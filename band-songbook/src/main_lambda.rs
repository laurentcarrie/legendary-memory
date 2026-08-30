use band_songbook::make_all_with_storage;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct Request {
    #[serde(alias = "srcdir")]
    songs_srcdir: String,
    #[serde(default)]
    books_srcdir: Option<String>,
    settings: String,
    delivery: String,
    pattern: Option<String>,
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

    let sandbox = tempfile::tempdir()?;

    log::info!("songs srcdir: {}", &req.songs_srcdir);
    log::info!("books srcdir: {:?}", &req.books_srcdir);
    log::info!("settings: {}", &req.settings);
    log::info!("delivery: {}", &req.delivery);
    log::info!("sandbox: {}", sandbox.path().display());

    match make_all_with_storage(
        &req.songs_srcdir,
        req.books_srcdir.as_deref(),
        sandbox.path(),
        Some(req.settings.as_str()),
        req.pattern.as_deref(),
        &req.delivery,
        &[], // no drum patterns dirs
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

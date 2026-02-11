use band_songbook::make_all_with_storage;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::prelude::*;
use tracing_subscriber::reload;

#[derive(Deserialize, Debug)]
struct Request {
    srcdir: String,
    settings: String,
    delivery: String,
    pattern: Option<String>,
    log_level: Option<String>,
}

#[derive(Serialize)]
struct Response {
    request_id: String,
    success: bool,
    message: String,
}

fn parse_log_level(level: Option<&str>) -> LevelFilter {
    match level.map(|s| s.to_lowercase()).as_deref() {
        Some("trace") => LevelFilter::TRACE,
        Some("debug") => LevelFilter::DEBUG,
        Some("info") => LevelFilter::INFO,
        Some("warn") => LevelFilter::WARN,
        Some("error") => LevelFilter::ERROR,
        _ => LevelFilter::INFO,
    }
}

async fn function_handler(
    reload_handle: Arc<reload::Handle<LevelFilter, tracing_subscriber::Registry>>,
    event: LambdaEvent<Request>,
) -> Result<Response, Error> {
    let request_id = event.context.request_id.clone();
    let req = event.payload;

    // Update log level if specified in the request
    let level = parse_log_level(req.log_level.as_deref());
    reload_handle.reload(level)?;

    // Use a fixed path for local sandbox (Lambda only has /tmp writable)
    let sandbox = std::path::Path::new("/tmp/sandbox");

    log::info!("srcdir: {}", &req.srcdir);
    log::info!("settings: {}", &req.settings);
    log::info!("delivery: {}", &req.delivery);
    log::info!("sandbox: {}", sandbox.display());
    log::info!("log_level: {level}");

    match make_all_with_storage(
        &req.srcdir,
        sandbox,
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
    // Initialize logging with a reloadable filter so each invocation can set its own level
    let (filter, reload_handle) = reload::Layer::new(LevelFilter::INFO);
    let reload_handle = Arc::new(reload_handle);

    tracing_subscriber::registry()
        .with(filter)
        .with(
            tracing_subscriber::fmt::layer()
                .with_target(false)
                .without_time(),
        )
        .init();

    log::info!("Starting band-songbook lambda...");

    let handle = Arc::clone(&reload_handle);
    run(service_fn(move |event| {
        let handle = Arc::clone(&handle);
        async move { function_handler(handle, event).await }
    }))
    .await
}

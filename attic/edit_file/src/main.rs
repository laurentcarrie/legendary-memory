// use leptos::html::Title;
use edit_file::App;
use leptos::prelude::*;
pub fn main() {
    use tracing_subscriber::fmt;
    use tracing_subscriber_wasm::MakeConsoleWriter;

    fmt()
        .with_writer(
            // To avoide trace events in the browser from showing their
            // JS backtrace, which is very annoying, in my opinion
            MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG),
        )
        // For some reason, if we don't do this in the browser, we get
        // a runtime error.
        .without_time()
        .init();
    console_error_panic_hook::set_once();

    // view!{<Title text="songbook" />} ;
    // mount_to_body(fetch_example)
    mount_to_body(|| view! { <App/> })
}

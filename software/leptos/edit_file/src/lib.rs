use human_sort::compare;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::tachys::html::style::style;
use leptos_meta::*;
use std::cmp::Ordering;
use std::fs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen]
    fn my_edit(s: &str, data: &str) -> JsValue;
    fn my_set_data(e: JsValue, s: &str);
}

async fn fetch_file(path: String) -> Result<String> {
    // gloo_timers::future::TimeoutFuture::new(5000).await;
    log!("{}",path) ;
    // make the request
    let response  = reqwasm::http::Request::get(path.as_str())
        .mode(reqwasm::http::RequestMode::Cors)
        .send()
        .await;
    match response {
        Ok(x) => {
            log!("status {}",x.status()) ;
            let x=x.text().await? ;
            Ok(x)
        }
        Err(e) => {
            log!("error : {:?}",e) ;
            Ok(format!("{:?}",e))
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // we use new_unsync here because the reqwasm request type isn't Send
    // if we were doing SSR, then
    // 1) we'd want to use a Resource, so the data would be serialized to the client
    // 2) we'd need to make sure there was a thread-local spawner set up
    // let file_data = AsyncDerived::new_unsync(move || fetch_file("https://www.google.com/search?client=ubuntu-sn&channel=fs&q=curl+get+parameters".
    // to_string())) ;
    let file_data = AsyncDerived::new_unsync(move || fetch_file("http://185.247.117.231/input-songs/amy_winehouse/you_know_i_m_no_good/song.json".to_string()));

    let fallback = move |errors: ArcRwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect::<Vec<_>>()
            })
        };

        view! {
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    let spreadable = style(("foreground-color", "red"));

    view! {
            <main>
                <Script src="/src-noconflict/ace.js"></Script>
                <Script src="/my-ace.js"> </Script>
            </main>

                <Title text="songbook" />

            <div>

                <pre id="editor">r#"

xxx

edit me...

yyy

            "#</pre>

    </div>


                {view! {
                }}
                <div>
                       <Transition fallback=|| view! { <div>"Loading..."</div> } {..spreadable}>
                        <ErrorBoundary fallback>
                                {move || Suspend::new(async move {
                                    let text = match file_data.await {
                                        Ok(text) => {
                                             log!("found text : {} ",text.len()) ;
                                            log!("{}",text) ;
                                             my_edit("editor",text.as_str());
                                             text
                                        } ,
                                        Err(e) => {
                                             log!("{:?}",e) ;
                                            "???".to_string()
                                        }
                                    };

                                    view!{
                                    <h1> text </h1>
                                    };
                                    })}

                        </ErrorBoundary>
                    </Transition>
            </div>
            }
}

use human_sort::compare;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::tachys::html::style::style;
use leptos_meta::*;
use std::cmp::Ordering;
use std::fs;
use wasm_bindgen::prelude::*;
use leptos_router::hooks::query_signal;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen]
    fn my_edit(s: &str, data: &str,nblines:usize) -> JsValue;
    fn my_set_data(editor:JsValue, data: &str,nblines:usize) -> JsValue;
    fn my_get_data(e: JsValue) -> JsValue ;
}

async fn fetch_file(path: String) -> Result<String> {
    gloo_timers::future::TimeoutFuture::new(1000).await;
    log!("{}", path);
    // make the request
    let response = reqwasm::http::Request::get(path.as_str())
        // .mode(reqwasm::http::RequestMode::Cors)
        .send()
        .await;
    match response {
        Ok(x) => {
            log!("status {}", x.status());
            let x = x.text().await?;
            Ok(x)
        }
        Err(e) => {
            log!("error : {:?}", e);
            Ok(format!("{:?}", e))
        }
    }
}

#[component]
pub fn EditFile() -> impl IntoView {
    let file_data = AsyncDerived::new_unsync(move || {
        fetch_file(
            "http://185.247.117.231/input-songs/amy_winehouse/you_know_i_m_no_good/song.json"
                .to_string(),
        )
    });

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
    // let (gfile_data,sfile_data) = query_signal::<String>(Some("".to_string())) ;
    // let savecb = move |ev|  { log!("save data ") ; sfile_data.set(Some("xxx".to_string())) ; } ;
    let (count, set_count) = query_signal::<i32>(0);
    let clear = move |_| set_count.set(None);
    let decrement = move |_| set_count.set(Some(count.get().unwrap_or(0) - 1));
    let increment = move |_| set_count.set(Some(count.get().unwrap_or(0) + 1));

    view! {
                <Script src="/src-noconflict/ace.js"></Script>
                <Script src="/my-ace.js"> </Script>
        <div>
            <button on:click=clear>"Clear"</button>
            <button on:click=decrement>"-1"</button>
            <span>"Value: " {move || count.get().unwrap_or(0)} "!"</span>
            <button on:click=increment>"+1"</button>
        </div>

        <div>
                       <Transition fallback=|| view! { <div>"Loading..."</div> } {..spreadable}>
                        <ErrorBoundary fallback>
                                {move || Suspend::new(async move {
                                    match file_data.await {
                                        Ok(text) => {
                                             log!("found text, len is : {} ",text.len()) ;
                                            let nblines=text.chars().filter(|c| *c == '\n').count() ;
                                             log!("nblines : {} ",nblines) ;
                                             let editor=my_edit("editor",text.as_str(),nblines);
                                            my_set_data(editor,text.as_str(),nblines) ;

                                            view! {
                                                <button on:click=
                                                move |_| {
                                                log!("save") ;
                                                // let mjf = mjf.as_str().c
                                                    }
                                                >"save"</button>
                                            };
                                            ()
                                        } ,
                                        Err(e) => {
                                             log!("{:?}",e) ;
                                            view!{

                                            };
                                            ()
                                        }
                                    }

                                    })}

                        </ErrorBoundary>
                    </Transition>
            </div>
                    <div>

                <pre id="editor">r#"

xxx

edit me...

yyy

            "#</pre>

    </div>

            }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
            <Title text="songbook" />
        <div>
    <EditFile/>
        </div>
        }
}

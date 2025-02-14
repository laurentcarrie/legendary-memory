use human_sort::compare;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::tachys::html::style::style;
use leptos_meta::*;
use leptos_router::hooks::query_signal;
use std::cmp::Ordering;
use std::fs;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen]
    fn my_edit(s: &str, data: &str, nblines: usize) -> JsValue;
    fn my_set_data(editor: &JsValue, data: &str, nblines: usize) -> JsValue;
    fn my_get_data(e: &JsValue) -> String;
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
    // let editor=my_edit("editor","hello world",10) ;
    // my_set_data(editor,"hello world",10) ;
    // let xxx = move |a:&str,b:usize| {
    //     my_set_data(editor,a,b)
    // };
    let (g_editor, s_editor) = signal::<String>("".to_string());
    let load = move |_| {
        let text = g_editor.get();
        let nblines = text.chars().filter(|c| *c == '\n').count();
        // xxx(text.as_str(), nblines);
    };

    view! {
                <Script src="/src-noconflict/ace.js"></Script>
                <Script src="/my-ace.js"> </Script>
        <div>
            <button on:click=load>"load"</button>
        </div>

        <div>
                       <Transition fallback=|| view! { <div>"Loading..."</div> } {..spreadable}>
                        <ErrorBoundary fallback>
                                {move || Suspend::new(async move {
                                    let text = match file_data.await {
                                        Ok(text) => {
                                             log!("found text, len is : {} ",text.len()) ;
                                            text
                                        } ,
                                        Err(e) => {
                                             log!("{:?}",e) ;
                                            e.to_string()
                                        }
                                     } ;
                                     let nblines = text.chars().filter(|c| *c == '\n').count();
                                     let editor=my_edit("editor","hello world",10) ;
                                     my_set_data(&editor,&text,nblines) ;
                                    view! {
                                                <button on:click=move |_| {my_get_data(&editor);}>"save"</button>
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

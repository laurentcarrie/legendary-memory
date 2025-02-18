use base64::prelude::BASE64_STANDARD;
use base64::prelude::*;
use human_sort::compare;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::tachys::html::style::style;
use leptos_meta::*;
use std::cmp::Ordering;
use std::fs;
use wasm_bindgen::prelude::*;

pub mod input_model;

pub mod protocol;
use protocol::model::answer::{Choice, EChoice, SourceTree};
use protocol::model::request as request ;

fn default_world() -> SourceTree {
    SourceTree { items: vec![] }
}

async fn fetch_world() -> Result<SourceTree> {
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let world = reqwasm::http::Request::get(&format!(
        "/scripts/request.sh?request=eyJjaG9pY2UiOnsiSXRlbVNvdXJjZVRyZWUiOiBudWxsfX0K",
    ))
    .send()
    .await?
    .json::<Choice>()
    .await?;
    match world.choice {
        EChoice::ItemSourceTree(tree) => {
            log!("size of tree : {}", tree.items.len());
            Ok(tree)
        }
        _ => panic!("bad type"),
    }
}

async fn save_file(path: String,content:String) -> Result<()> {
    log!("save file") ;
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let choice=request::EChoice::ItemSaveFile(request::InfoSaveFile{path:path.clone(),content:content.clone()}) ;
    let request= request::Choice{choice:choice} ;
    let json_string = serde_json::to_string(&request)? ;
    log!("{}",&json_string) ;
    // dbg!(&json_string) ;
    let b64= BASE64_STANDARD.encode(&json_string) ;
    log!("{}",&b64) ;

    let data = reqwasm::http::Request::get(format!("/scripts/request.sh?request={}",b64).as_str())
        .send()
        .await?
        .text()
        .await?;
    Ok(())
}


async fn fetch_file(path: String) -> Result<String> {
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let data = reqwasm::http::Request::get(path.as_str())
        .send()
        .await?
        .text()
        .await?;
    Ok(data)
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    // we use new_unsync here because the reqwasm request type isn't Send
    // if we were doing SSR, then
    // 1) we'd want to use a Resource, so the data would be serialized to the client
    // 2) we'd need to make sure there was a thread-local spawner set up
    let world = AsyncDerived::new_unsync(move || fetch_world());

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
                <Stylesheet id="leptos" href="/style-source-tree.css"/>

                <Meta name="viewport" content="width=device-width, initial-scale=1"></Meta>

                <Script src="/src-noconflict/ace.js"></Script>
                <Script src="/my-ace.js"> </Script>
            </main>

                <Title text="songbook" />

        <div id="songpick-id">
         <label for="cars">Choose a car:</label>

<select name="cars" id="cars">
  <option value="volvo">Volvo</option>
  <option value="saab">Saab</option>
  <option value="mercedes">Mercedes</option>
  <option value="audi">Audi</option>
</select>
        </div>

<div id="containerxx">
            <div class="split right">
        <div class="xxxcentered">

                <pre id="editor">r#"

xxx

edit me...

yyy

            "#</pre>
</div>
    </div>

        <div class="split left">
        <div class="centered">
                <div>
                       <Transition fallback=|| view! { <div>"Loading..."</div> } {..spreadable}>
                        <ErrorBoundary fallback>
                                <label>songs</label>
                                <ul>
                                {move || Suspend::new(async move {
                                    let w = match world.await {
                                        Ok(w) => {
                                             log!("number of items : {} ",&w.items.len()) ;
                                            w
                                        } ,
                                        Err(_) => {
                                             log!("default") ;
                                            default_world()
                                        }
                                    };
                                    // let w = &w.items.iter().map(|w| (w.clone(),signal( false))).collect::<Vec<_>>() ;
                                    let mut items = w.items ;
                                    items.sort_by(|a,b| match compare(a.author.as_str(),b.author.as_str()) {
                                        Ordering::Equal => {
                                            compare(a.title.as_str(),b.title.as_str())
                                        },
                                        x => x
                                    }) ;

                                    view!{
                                    <h1> "number of items : " </h1>
                                    };
                                    // let aedit = my_edit("editor") ;
                                    items.into_iter().map(|i| {
                                        // let i=&(p.0) ; // item
                                        // let (expanded,set_expanded)=&(p.1) ; // signal
                                        // let expanded=&(p.1.0) ;
                                        let (expanded,set_expanded) = signal(false) ;
                                        let mjf = i.masterjsonfile.clone() ;
                                        view! {
                                            <li>
                                            <button on:click=move |_| {
                                                *set_expanded.write() = ! expanded.get()
                                            }>
                                            {i.author.clone()} / {i.title.clone()}
                                            </button>
                                            <ul style:display=move || if expanded.get() { "block" } else { "none" }>
                                                <li>
                                                    <EditFile label="master json".to_string() url=mjf editor_id="editor".to_string() />
                                                    master.json
                                                </li>

                                                <li>tex files
                                                    <ul>
                                                    { i.texfiles.clone().into_iter().map(|f| {
                                                        view! {
                                                            <li>
                                                             <EditFile label=f.clone() url=f.clone() editor_id="editor".to_string() />
                                                            {f}
                                                            </li>
                                                        }
                                                        }).collect::<Vec<_>>()
                                                    }
                                                    </ul>
                                                </li>

                                                <li>lyrics tex files
                                                    <ul>
                                                    { i.lyricstexfiles.clone().into_iter().map(|f| {
                                                        view! {
                                                            <li>
                                                             <EditFile label=f.clone() url=f.clone() editor_id="editor".to_string() />
                                                            {f}
                                                            </li>
                                                        }
                                                        }).collect::<Vec<_>>()
                                                    }
                                                    </ul>
                                                </li>

                                                <li>lilypond files
                                                    <ul>
                                                    { i.lyfiles.clone().into_iter().map(|f| {
                                                        view! {
                                                            <li>
                                                             <EditFile label=f.clone() url=f.clone() editor_id="editor".to_string() />
                                                            {f}
                                                            </li>
                                                        }
                                                        }).collect::<Vec<_>>()
                                                    }
                                                    </ul>
                                                </li>

                                            </ul>
                                        </li>
        }
                                    }).collect::<Vec<_>>()
                                    // w.songs.into_iter().map(|s| {
                                    //                 view! {
                                    //                     <li>
                                    //                         {s.path.clone()}
                                    //                     </li>
                                    //                 }
                                    //             })
                                    //             .collect::<Vec<_>>()
                                })}
                                </ul>

                        </ErrorBoundary>
                        // <ErrorBoundary fallback>
                        //         <label>books</label>
                        //         <ul>
                        //         {move || Suspend::new(async move {
                        //             let w = world.await.unwrap() ;
                        //             w.books.into_iter().map(|s| {
                        //                             view! {
                        //                                 <li>
                        //                                     {s.path.clone()}
                        //                                 </li>
                        //                             }
                        //                         })
                        //                         .collect::<Vec<_>>() ;
                        //         })}
                        //         </ul>
                        //
                        // </ErrorBoundary>
                    </Transition>
            </div>
        </div>
        </div>
        </div>

            }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    // #[wasm_bindgen]
    fn my_edit(s: &str, data: &str, nblines: usize) -> JsValue;
    fn my_set_data(editor: &JsValue, data: &str, nblines: usize) -> JsValue;
    fn my_get_data(e: &JsValue) -> String;
}

#[component]
pub fn EditFile(label: String, url: String, editor_id: String) -> impl IntoView {
    let url1=url.clone() ;
    let file_data = AsyncDerived::new_unsync(move || fetch_file(url1.clone()));

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
    let (g_url, s_url) = signal::<String>("".to_string()) ;
    let url=url.clone() ;
    s_url.set(url) ;

    view! {
                <Script src="/src-noconflict/ace.js"></Script>
                <Script src="/my-ace.js"> </Script>
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
                                    // let editor=my_edit(id.as_str(),"hello world",10) ;
                                    let editor=my_edit("editor","hello world",10) ;
                                    let editor2=editor.clone() ;
                                    view! {
                                        <button
                                        on:click=move |_| {
                                             let nblines = text.chars().filter(|c| *c == '\n').count();
                                             my_set_data(&editor,&text,nblines) ;
                                                    ()
                                        }>{g_url.get()}</button>
                                        <button
                                        on:click=move |_| {
                                             let data=my_get_data(&editor2) ;
                                            log!("going to save file") ;
                                            let result = AsyncDerived::new_unsync(move ||  save_file(g_url.get(),data.clone())) ;
                                            log!("result : {:?}",result) ;
                                                    ()
                                        }>save</button>

                                    }
                                    })}

                        </ErrorBoundary>
                    </Transition>

            }
}

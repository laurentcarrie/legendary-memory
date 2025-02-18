use index_map::IndexMap;
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
use leptos::html::Select ;

pub mod input_model;

pub mod protocol;
use protocol::model::answer::{Choice, EChoice, SourceTree,SourceTreeItem};
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
    let (g_song,w_song) = signal::<Vec<(String,String)>>(vec![]);
    let (value, set_value) = signal::<String>(BASE64_STANDARD.encode("???")) ;


    view! {
            <main>
                <Stylesheet id="leptos" href="/style-source-tree.css"/>

                <Meta name="viewport" content="width=device-width, initial-scale=1"></Meta>

                <Script src="/src-noconflict/ace.js"></Script>
                <Script src="/my-ace.js"> </Script>
            </main>

                <Title text="songbook" />



<div id="container">
            <div class="split right">
        <div class="xxxcentered">

                <pre id="editor">r#"

xxx

edit me...

yyy

            "#</pre>
</div>
    </div>

        <div class="splitx leftx">
        <div class="centered">
                <div>
                       <Transition fallback=|| view! { <div>"Loading..."</div> } {..spreadable}>
                        <ErrorBoundary fallback>
                                <label>songs</label>
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
                                    // let mut items : Vec<SourceTreeItem> = vec![] ;
                                    let data : String = BASE64_STANDARD.encode(serde_json::to_string(& items[0]).unwrap() ) ;
                                    set_value.set(data) ;


                                    view!{
                                    <h1> "number of items : " </h1>
                                <div id="songpick-id">
                                    <label for="cars">Choose a song:</label>
                                    <select name="song" id="song-select"
                                        on:change:target=move |ev| {
                                            set_value.set(ev.target().value().parse().unwrap());
                                            log!("value is {}",value.get()) ;
                                        }
                                        prop:value=move || value.get()>
                                        view! {
                                            items.clone().into_iter().enumerate().map(|c|{
                                                let index=c.0 ;
                                                let c=c.1 ;
                                                log!("option {}",&c.author) ;
                                                view! { <option value={
                                                    let data = BASE64_STANDARD.encode(serde_json::to_string(&c).unwrap() ) ;
                                                    data
                                                    }>{c.author.clone()} @ {c.title.clone()}</option>}
                                            }).collect::<Vec<_>>()
                                        }
                                    </select>
                                </div>

                                <div>{move || {
                                            let data = BASE64_STANDARD.decode(value.get()).expect("valid base64 string");
                                            let data = String::from_utf8(data).expect("utf8 string");
                                            let c:SourceTreeItem = serde_json::from_str(data.as_str()).unwrap() ;
                                            view! {
                                                <ul>
                                                <li>
                                                  <EditFile label=c.masterjsonfile.clone() url=c.masterjsonfile.clone() editor_id="editor".to_string() />
                                                            {c.masterjsonfile.clone()}
                                                </li>
                                                </ul>
                                            }}}
                                </div>


                                <div>
                                    <label>{value}</label><br/>
                                    <label>{move || {
                                            log!("label value : {}",value.read()) ;
                                            let data = BASE64_STANDARD.decode(value.get()).expect("valid base64 string");
                                            let data = String::from_utf8(data).expect("utf8 string");
                                            let c:SourceTreeItem = serde_json::from_str(data.as_str()).unwrap() ;
                                            data
                                        }}</label><br/>
                                </div>


                                        }

                                })}

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

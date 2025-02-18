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

fn base64_to_item(input:String) -> SourceTreeItem {
    let data : Option<Vec<u8>> = BASE64_STANDARD.decode(input.as_str()).ok() ;
    let data : Option<String> = data.map(|c| String::from_utf8(c).ok()).flatten() ;
    let c:Option<SourceTreeItem> = data.map(|s| serde_json::from_str(s.as_str()).ok()).flatten();
    match c {
        Some(c) => c,
        None => {
            log!("could not parse input string to SourceTreeItem : {}",&input) ;
            SourceTreeItem{
                title:"error".to_string(),
                author:"error".to_string(),
                masterjsonfile: "error".to_string(),
                texfiles:vec![],
                lyricstexfiles:vec![],
                lyfiles:vec![]
            }
        }
    }
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
    let (file_value, set_file_value) = signal::<String>("???".to_string()) ;


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
                                    log!("set_value") ;
                                    let data : String = BASE64_STANDARD.encode(serde_json::to_string(& items[0]).expect("serde-json") ) ;
                                    set_value.set(data) ;


                                    view!{
                                        <div id="songpick-id">
                                            <label for="songs">Choose a song:</label>
                                            <select name="song" id="song-select"
                                        on:change:target=move |ev| {
                                            log!("on change") ;
                                            set_value.set(ev.target().value().parse().expect("set_value"));
                                            log!("value is {}",value.get()) ;
                                        }
                                        prop:value=move || value.get()>
                                        view! {
                                            items.clone().into_iter().enumerate().map(|c|{
                                                let index=c.0 ;
                                                let c=c.1 ;
                                                // log!("option {}",&c.author) ;
                                                view! { <option value={
                                                    log!("encode") ;
                                                    let data = BASE64_STANDARD.encode(serde_json::to_string(&c).expect("base64") ) ;
                                                    data
                                                    }>{c.author.clone()} @ {c.title.clone()}</option>}
                                            }).collect::<Vec<_>>()
                                        }
                                        </select>
                                        </div>
                                    }

                                })}

                                {move || {
                                    let c = base64_to_item(value.get()) ;
                                    view! {
                                        <div id="filepick-id">
                                            <label for="songs">Choose a file:</label>
                                            <select name="file" id="file-select"
                                        on:change:target=move |ev| {
                                            log!("on change") ;
                                            set_file_value.set(ev.target().value().parse().expect("set_value"));
                                            log!("value is {}",file_value.get()) ;
                                        }
                                        prop:value=move || file_value.get()>
                                        {
                                            view! {
                                                <optgroup label="master file">
                                                    <option value={
                                                    c.masterjsonfile.clone()
                                                    }>{c.masterjsonfile.clone()}</option>
                                                </optgroup>
                                                <optgroup label="tex files">
                                                    { move ||{
                                                        c.texfiles.clone().into_iter().map(|f|{
                                                            view!{
                                                            <option value={f.clone()}>{f.clone()}</option>
                                                            }}
                                                        ).collect::<Vec<_>>()
                                                    }}
                                                </optgroup>
                                                <optgroup label="lyrics tex files">
                                                    { move ||{
                                                        c.lyricstexfiles.clone().into_iter().map(|f|{
                                                            view!{
                                                            <option value={f.clone()}>{f.clone()}</option>
                                                            }}
                                                        ).collect::<Vec<_>>()
                                                    }}
                                                </optgroup>
                                                <optgroup label="lilypond files">
                                                    { move ||{
                                                        c.lyfiles.clone().into_iter().map(|f|{
                                                            view!{
                                                            <option value={f.clone()}>{f.clone()}</option>
                                                            }}
                                                        ).collect::<Vec<_>>()
                                                    }}
                                                </optgroup>
                                            }
                                        }
                                        </select>
                                        </div>
                                    }
                                }}

                                {move || {
                                    log!("{}",file_value.get()) ;
                                    view!{
                                        {file_value}
                                    }
                                }}



                                // {move || {
                                //     log!("{}",value.get()) ;
                                //     let c = base64_to_item(value.get()) ;
                                //     view! {
                                //         <ul>
                                //         json master file
                                //         <li>
                                //             <EditFile label="master json".to_string() url=c.masterjsonfile.clone() mode="ace/mode/json".to_string() editor_id="editor".to_string() />
                                //         </li>
                                //         </ul>
                                //     }
                                // }}
                                //

                                {move || {
                                    log!("{}",value.get()) ;
                                    let c = base64_to_item(value.get()) ;
                                    view! {
                                        <ul> tex files
                                        { c.texfiles.clone().into_iter().map(|f| {
                                            view!{
                                        <li>
                                            <EditFile label="master json".to_string() url=f.clone() mode="ace/mode/latex".to_string() editor_id="editor".to_string() />
                                        </li>
                                                }
                                            }).collect::<Vec<_>>()
                                            }
                                        </ul>
                                    }
                                }}

                                {move || {
                                    log!("{}",value.get()) ;
                                    let c = base64_to_item(value.get()) ;
                                    view! {
                                        <ul> tex files for lyrics sections
                                        { c.lyricstexfiles.clone().into_iter().map(|f| {
                                            view!{
                                        <li>
                                            <EditFile label="master json".to_string() url=f.clone() mode="ace/mode/latex".to_string() editor_id="editor".to_string() />
                                        </li>
                                                }
                                            }).collect::<Vec<_>>()
                                            }
                                        </ul>
                                    }
                                }}

                                {move || {
                                    log!("{}",value.get()) ;
                                    let c = base64_to_item(value.get()) ;
                                    view! {
                                        <ul> lilypond files
                                        { c.lyfiles.clone().into_iter().map(|f| {
                                            view!{
                                        <li>
                                            <EditFile label="master json".to_string() url=f.clone() mode="ace/mode/lilypond".to_string() editor_id="editor".to_string() />
                                        </li>
                                                }
                                            }).collect::<Vec<_>>()
                                            }
                                        </ul>
                                    }
                                }}


                        </ErrorBoundary>
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
    fn my_set_data(editor: &JsValue, data: &str,nblines: usize) -> JsValue;
    fn my_set_mode(editor: &JsValue, mode:&str) -> JsValue;
    fn my_get_data(e: &JsValue) -> String;
}

#[component]
pub fn EditFile(label: String, url: String, mode:String,editor_id: String) -> impl IntoView {
    let url1=url.clone() ;
    let mode1=mode.clone() ;
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
                                    // my_set_mode(&editor2,mode.as_str()) ;
                                    view! {
                                        <button
                                        on:click=move |_| {
                                             let nblines = text.chars().filter(|c|
                                                *c == '\n').count();
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

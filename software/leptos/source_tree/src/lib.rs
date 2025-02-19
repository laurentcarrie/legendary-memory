use base64::prelude::BASE64_STANDARD;
use base64::prelude::*;
use human_sort::compare;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::tachys::html::style::style;
use leptos_meta::*;
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

pub mod input_model;

pub mod protocol;
use protocol::model::answer::{Choice, EChoice, SourceTree,SourceTreeItem};

pub mod util ;
use util::{SourceTreeItem_of_base64,default_world,fetch_world,save_file,fetch_file} ;




#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
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
    let (song_value, set_song_value) = signal::<String>(BASE64_STANDARD.encode("???")) ;
    let (file_value, set_file_value) = signal::<String>("???".to_string()) ;

    let async_file_data = LocalResource::new(move || fetch_file(file_value.get())) ;

    let async_file_result = move || {
        async_file_data
            .get()
            .as_deref()
            .map(|value| {
                log!("YYYYYY") ;
                // format!("Server returned {value:?}")
                match value {
                    Ok(t) => {
                        let nblines = t.chars().filter(|c| *c == '\n').count();
                        let editor = my_edit("editor","sss","ace/mode/json",nblines) ;
                        my_set_data(&editor,t.clone().as_str(),30) ;
                        format!("file {}",t.clone())
                    },
                    Err(e) => format!("Erreur {:?}",e)
                }
            })
            // This loading state will only show before the first load
            .unwrap_or_else(|| "XXXXX Loading...".into())
    };
    view!{
    }


    // Effect::new(move |prev_value| {
    //     // first, access the signal’s value and convert it to a string
    //     let path = file_value.get().clone() ;
    //
    //     match prev_value {
    //         None=>log!("none"),
    //         Some(s) => {
    //             if s == path {
    //                 log!("no change")
    //             } else {
    //                 log!("change") ;
    //                 let document = document();
    //                 let p = match document.get_element_by_id("xxx") {
    //                     Some(p) => {
    //                         log!("found document xxx") ;
    //                         p
    //                     }
    //                     None => {
    //                         log!("DID NOT find document xxx") ;
    //                         document.create_element("xxx").unwrap()
    //
    //                     }
    //                 } ;
    //                 let p2=path.clone() ;
    //                 let file_data = AsyncDerived::new_unsync(move || fetch_file(p2.clone()));
    //                 let p2=path.clone() ;
    //                 p.set_text_content(Some(p2.as_str())) ;
    //             }
    //         }
    //     }
    //     // if this is different from the previous value, update the node
    //     // if prev_value != Some(text) {
    //     //     log!("change") ;
    //     //     p.set_text_content(Some(text.as_str()));
    //     // }
    //
    //     // return this value so we can memoize the next update
    //     path
    // });


    view! {
            <main>
                <Stylesheet id="leptos" href="/style-source-tree.css"/>
                <Meta name="viewport" content="width=device-width, initial-scale=1"></Meta>
                <Script src="/src-noconflict/ace.js"></Script>
                <Script src="/my-ace.js"> </Script>
            </main>
            <Title text="songbook" />
            <div id="container">
            <p id="xxx">blab blah</p>
            <div class="split right">
                <div class="xxxcentered">
                    <pre id="editor">r#"

xxx

edit me...

yyy

                "#</pre>
                </div>
            </div>

        <p><pre>{async_file_result}</pre></p>


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
                                    set_song_value.set(data) ;
                                    view!{
                                        <div id="songpick-id">
                                            <label for="songs">Choose a song:</label>
                                            <select name="song" id="song-select"
                                        on:change:target=move |ev| {
                                            log!("on change song") ;
                                            set_song_value.set(ev.target().value().parse().expect("set_value"));
                                            log!("song value is {}",song_value.get()) ;
                                            // let c  = base64_to_item(song_value.get()) ;
                                            // log!("after change, pointing to ")
                                        }
                                        prop:value=move || song_value.get()>
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
                                    let c = SourceTreeItem_of_base64(song_value.get()) ;
                                    view! {
                                        <div id="filepick-id">
                                            <label for="songs">Choose a file:</label>
                                            <select name="file" id="file-select"
                                        on:change:target=move |ev| {
                                            log!("on change") ;
                                            set_file_value.set(ev.target().value().parse().expect("set_value"));
                                            log!("value is {}",file_value.get()) ;
                                        } // on:change
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

                        </ErrorBoundary>
                    </Transition>
            </div>

        </div>
        </div>
                                {move || {
                                    view!{
                                        <label>blah</label><br/>
                                      <label>{
                                            log!("current file : {}",file_value.get())}
                                            let editor = my_edit("editor","sss","xx",10) ;
                                        </label><br/>
                                    }
                                }}
        </div>

            }
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    // #[wasm_bindgen]
    fn my_edit(s: &str, data: &str, mode:&str,nblines: usize) -> JsValue;
    fn my_set_data(editor: &JsValue, data: &str,nblines: usize) -> JsValue;
    fn my_set_mode(editor: &JsValue, mode:&str) -> JsValue;
    fn my_get_data(e: &JsValue) -> String;
}

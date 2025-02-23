use chrono::Utc ;
use serde_wasm_bindgen ;
use base64::prelude::BASE64_STANDARD;
use base64::prelude::*;
use human_sort::compare;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::tachys::html::style::style;
use leptos_meta::*;
use std::cmp::Ordering;
use std::path::PathBuf;
use wasm_bindgen::prelude::*;

pub mod protocol;

pub mod util;
use util::{default_world, fetch_file,save_file, fetch_world, build,SourceTreeItem_of_base64,omake_children_info};

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
    let (song_value, set_song_value) = signal::<String>(BASE64_STANDARD.encode("???"));
    let (file_value, set_file_value) = signal::<String>("???".to_string());
    let (file_save_value, set_file_save_value) = signal::<(String,String)>(("???".to_string(),"???".to_string()));
    let (build_value, set_build_value) = signal::<Option<String>>(None);
    // let (omake_children_value, set_omake_children_value) = signal::<String>("???".to_string());
    let (see_editor,set_see_editor) = signal::<bool>(false) ;
    let (see_html,set_see_html) = signal::<bool>(false) ;
    let async_file_data = LocalResource::new(move || fetch_file(file_value.get()));
    let async_file_save_data = LocalResource::new(move || save_file(file_save_value.get().0,file_save_value.get().1));
    let async_build_data = LocalResource::new(move || { log!("xxx build") ; let now = build_value.get() ; build(now)});
    // let async_omake_children_data = LocalResource::new(move || { let _ = omake_children_value.get() ; omake_children_info() });

    let async_file_result = move || {
        async_file_data
            .get()
            .as_deref()
            .map(|value| {
                match value {
                    Ok(t) => {
                        let (url, t) = t;
                        let _nblines = t.chars().filter(|c| *c == '\n').count();
                        let p = PathBuf::from(&url) ;
                        let extension : &str = p.extension().map(|x| x.to_str()).flatten().unwrap_or("") ;
                        // log!("extension : {:?}", &extension);
                        let mode = match extension {
                            "json" => "ace/mode/json",
                            "tex" => "ace/mode/latex",
                            "html" => "html",
                            _ => "ace/mode/text"
                        } ;
                        match mode {
                            "html" => {
                                let e = document().get_element_by_id("showhtml").unwrap();
                                e.set_inner_html(t.clone().as_str());
                                set_see_editor.set(false) ;
                                set_see_html.set(true) ;
                            }
                            _ => {
                                // let editor = my_edit("editor", "sss", format, nblines);
                                log!("before unwrap") ;
                                // my_edit("editor","xxx","yyy",10) ;
                                // let example = serde_wasm_bindgen::from_value(editor);
                                // log!("example : {:?}",&example) ;
                                // let x = s.from_value(editor).unwrap();
                                // let editor = serde_wasm_bindgen::to_value(& xeditor.get()).unwrap() ;
                                // let array = Uint8Array::new(&editor);
                                // let bytes: Vec<u8> = array.to_vec();
                                // set_xeditor.set(Some(bytes)) ;
                                // let s = serde_wasm_bindgen::into_serde(editor) ;
                                my_set_data("editor", t.clone().as_str(),mode, 80);
                                set_see_editor.set(true) ;
                                set_see_html.set(false) ;
                            }
                        } ;
                        "".to_string()
                    }
                    Err(e) => format!("Erreur {:?}", e),
                }
            })
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading file ...".into())
    };

    let async_file_save_result = move || {
        async_file_data
            .get()
            .as_deref()
            .map(|value| {
                match value {
                    Ok(_) => {
                        "file saved".to_string()
                    }
                    Err(e) => format!("Erreur {:?}", e),
                }
            })
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Saving file ...".into())
    };


    let async_build_result = move || {
        async_build_data
            .get()
            .as_deref()
            .map(|value| {
                match value {
                    Ok(_) => {
                        "".to_string()
                    }
                    Err(e) => format!("Erreur {:?}", e),
                }
            })
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Building ...".into())
    };


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
                <pre id="editor" style:display=move || if see_editor.get() { "block" } else { "none" }>r#"
edit me...
                "#</pre>
                <p id="showhtml" style:display=move || if see_html.get() { "block" } else { "none" }>r#"
edit me...
                "#</p>
        </div>

    <p><pre>{async_file_result}</pre></p>
    <p><pre>{async_file_save_result}</pre></p>
    <p>build value : <pre>{build_value}</pre></p>
    <p><pre>{async_build_result}</pre></p>

        <div class="splitx leftx">
            <div class="centered">
                <div>
                   <Transition fallback=|| view! { <div>"Loading..."</div> } {..spreadable}>
                    <ErrorBoundary fallback>
                            {move || Suspend::new(async move {
                                let w = match world.await {
                                    Ok(w) => {
                                         // log!("number of items : {} ",&w.items.len()) ;
                                        w
                                    } ,
                                    Err(e) => {
                                         log!("error {:?}",e) ;
                                        default_world()
                                    }
                                };

                                let mut items = w.items ;
                                items.sort_by(|a,b| match compare(a.author.as_str(),b.author.as_str()) {
                                    Ordering::Equal => {
                                        compare(a.title.as_str(),b.title.as_str())
                                    },
                                    x => x
                                }) ;
                                let _ = {
                                    match &items.get(0)  {
                                        None => (),
                                        Some(item) => {
                                            let data : String = BASE64_STANDARD.encode(serde_json::to_string(& item).expect("serde-json") ) ;
                                            set_song_value.set(data) ;
                                            set_file_value.set(item.masterjsonfile.clone()) ;
                                            ()
                                        }
                                    }
                                };
                                view!{
                                    <div id="songpick-id">
                                        <label for="songs">Choose a song:</label>
                                        <select name="song" id="song-select"
                                    on:change:target=move |ev| {
                                        log!("on change song") ;
                                        set_song_value.set(ev.target().value().parse().expect("set_value"));
                                        log!("song value is {}",song_value.get()) ;
                                        let c  = SourceTreeItem_of_base64(song_value.get()) ;
                                        set_file_value.set(c.masterjsonfile) ;
                                        set_see_editor.set(true) ;
                                        set_see_html.set(false) ;
                                        log!("after change, pointing to master json")
                                    }
                                    prop:value=move || song_value.get()>
                                    view! {
                                        items.clone().into_iter().map(|c|{
                                            view! { <option value={
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
                                            <optgroup label="master tex file">
                                                <option value={
                                                c.mastertexfile.clone()
                                                }>{c.mastertexfile.clone()}</option>
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

        <button
            on:click=move |_|
                {
                    let file = file_value.get() ;
                    log!("save {}",file) ;
                    set_file_save_value.set((file,my_get_data("editor"))) ;
            }>"save"</button>

        <button
            on:click=move |_|
                {
                    let message = my_commit_message() ;
            }>"commit"</button>


        <hr/>

        <button
            on:click=move |_|
                {
                    log!("build") ;
                    let now : chrono::DateTime<chrono::Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
                    let now = now.format("%Y-%m-%d-%H-%M-%S").to_string() ;
                    log!("build now : {}",now) ;
                    set_build_value.set(now)
            }>"build"</button>


        <button
            on:click=move |_|
                {
                    log!("show build progress") ;
                    set_file_value.set(format!("/output/omake.{}.stdout",build_value.get()))
            }>"progress (stdout)"</button>

        <button
            on:click=move |_|
                {
                    log!("show build progress") ;
                    set_see_editor.set(false) ;
                    set_see_html.set(true) ;
                    let filename=format!("/output/progress.{}.html",build_value.get()) ;
                    set_file_value.set(filename)
            }>"progress (html)"</button>


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
    fn my_edit(id: &str, data: &str, mode: &str, nblines: usize) -> JsValue;
    fn my_set_data(id:&str, data: &str, mode:&str,nblines: usize) -> JsValue;
    fn my_set_mode(id:&str, mode: &str) -> JsValue;
    fn my_get_data(id:&str) -> String;
    fn my_commit_message() -> String;
}

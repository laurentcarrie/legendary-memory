use base64::prelude::BASE64_STANDARD;
use base64::prelude::*;
use chrono::Utc;
use human_sort::compare;
use leptos::logging::log;
use leptos::prelude::*;
use leptos::tachys::html::style::style;
use leptos_meta::*;
use std::cmp::Ordering;
use wasm_bindgen::prelude::*;

use crate::util::{
    default_world, fetch_world, get_something_to_see, SourceTreeItem_of_base64, WhatToShow,
    WhatToShowResult,
};

use crate::components::editor::Editor;
use crate::components::progress::Progress;
use crate::components::show_error::ShowError;
use crate::components::show_omake_stdout::OmakeStdout;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    // #[wasm_bindgen]
    // fn my_edit(id: &str, data: &str, mode: &str, nblines: usize) -> JsValue;
    // fn my_set_data(id: &str, data: &str, mode: &str, nblines: usize,onchange:&js_sys::Function) -> JsValue;
    fn my_set_data(id: &str, data: &str, mode: &str, nblines: usize) -> JsValue;
    // fn my_set_mode(id: &str, mode: &str) -> JsValue;
    fn my_get_data(id: &str) -> String;
    fn my_commit_message() -> String;

    // pub fn alert(s: &str);

}

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

    // enum type on what to show in the right pane
    let (what_to_show, set_what_to_show) = signal::<WhatToShow>(WhatToShow::Nothing);
    // enum type on what to show in the right pane
    let (what_to_show_result, set_what_to_show_result) =
        signal::<WhatToShowResult>(WhatToShowResult::Nothing);

    let spreadable = style(("foreground-color", "red"));
    // the json data of the current song
    let (song_value, set_song_value) = signal::<String>(BASE64_STANDARD.encode("???"));

    // the signal that triggers a build
    // let (build_value, set_build_value) = signal::<Option<String>>(None);

    // the signal that triggers a file save
    // let (file_save_value,set_file_save_value) = signal::<Option<(String,String)>>(None) ;

    // ressource to handle answers from server
    let async_request_data = LocalResource::new(move || {
        log!("{}:{}", file!(), line!());
        get_something_to_see(what_to_show.get())
    });

    // let async_file_save_data =
    //      LocalResource::new(move || match file_save_value.get() {
    //          Some((path,value)) => save_file(path,value),
    //          None => ()
    //      });

    // whenever build signal changes, start a new build
    // let _ = LocalResource::new(move || {
    //     log!("{}:{} build", file!(), line!());
    //     let now = build_value.get();
    //     let info = build(now) ;
    //     set_what_to_show_result(WhatToShowResult::OMakeStartBuild(info)) ;
    // });
    // let async_omake_children_data = LocalResource::new(move || { let _ = omake_children_value.get() ; omake_children_info() });

    let async_file_result = move || {
        async_request_data
            .get()
            .as_deref()
            .map(|value| match value {
                Ok(t) => {
                    log!("{}:{} async file result", file!(), line!());
                    // log!("{:?}", &t);
                    set_what_to_show_result.set(t.clone());
                    "".to_string()
                }
                Err(e) => format!("Erreur {:?}", e),
            })
            // This loading state will only show before the first load
            .unwrap_or_else(|| "Loading file ...".into())
    };

    view! {
        <main>
            // <Stylesheet id="leptos" href="/style-source-tree.css"/>
            <Meta name="viewport" content="width=device-width, initial-scale=1"></Meta>
            <Script src="/src-noconflict/ace.js"></Script>
            <Script src="/my-ace.js"> </Script>
        </main>
        <Title text="songbook" />


        <div id="container">
        <div class="split right">
                <Editor wtsr=what_to_show_result/>
                <Progress wtsr=what_to_show_result/>
                <OmakeStdout data=what_to_show_result/>
                <ShowError data=what_to_show_result/>
        </div>

    // <div id="fv"><pre>{ move || file_value.get() }</pre></div>
    <div id="wts">what to show <pre>{ move || {
            log!("{}:{}",file!(),line!()) ;
            serde_json::to_string(&what_to_show.get())
            }
        }</pre>
    </div>


        {async_file_result}


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
                                            ()
                                        }
                                    }
                                };

                                // the combo list of songs
                                view!{
                                    <div id="songpick-id">
                                        <label>Choose a song:</label>
                                        <select name="song" id="song-select"
                                    on:change:target=move |ev| {
                                        log!("{}:{} on change song",file!(),line!()) ;
                                        set_song_value.set(ev.target().value().parse().expect("set_value"));
                                        log!("{}:{} song value is {}",file!(),line!(),song_value.get()) ;
                                        let c  = SourceTreeItem_of_base64(song_value.get()) ;
                                        set_what_to_show.set(WhatToShow::SourceFile(c.masterjsonfile.clone())) ;
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
                                } // view

                            })}

                            { move || {
                                let c = SourceTreeItem_of_base64(song_value.get()) ;
                                #[allow(unused_assignments)]
                                let mut first_item : Option<WhatToShow> = None ;
                                let view = view! {
                                    <div id="filepick-id">
                                        <label for="songs">Choose a file:</label>
                                        <select name="file" id="file-select"
                                    on:change:target=move |ev| {
                                        log!("{}:{} on change",file!(),line!()) ;
                                        set_what_to_show.set(WhatToShow::SourceFile(ev.target().value().parse().expect("set_value"))) ;
                                    } // on:change
                                    prop:value=move || {
                                        let x = what_to_show.get() ;
                                        if let WhatToShow::SourceFile(s) = x {
                                            s
                                        }
                                        else {
                                            format!("{}:{}",file!(),line!())
                                        }
                                    }
                                    >
                                    {
                                        view! {
                                            <optgroup label="master file">
                                                <option value={
                                                first_item = Some(WhatToShow::SourceFile(c.masterjsonfile.clone())) ;
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
                                } ;
                                match first_item {
                                    Some(ws) => set_what_to_show.set(ws),
                                    None => ()
                                } ;
                                view
                            }
                            }

                    </ErrorBoundary>
                </Transition>
        </div>

        <button
            id="b-save"
            on:click=move |_|
                {
                    // let file = file_value.get() ;
                    // log!("save {}",file) ;
                    // set_file_save_value.set((file,my_get_data("editor"))) ;
                    if let WhatToShow::SourceFile(path) = what_to_show.get() {
                        log!("save file {}",&path) ;
                        let data = my_get_data("editor") ;
                        log!("{}",&data) ;
                        // @todo self ?
                        let document = leptos::leptos_dom::helpers::document() ;
                        let e = document.get_element_by_id("b-save").unwrap();
                        e.style(("background-color", "grey"));
                        set_what_to_show.set(WhatToShow::SaveSourceFile(path.clone(),data))
                        // set_what_to_show.set(WhatToShow::SourceFile(path))

                    }
            }>"save"</button>

        <button
            on:click=move |_|
                {
                    let _message = my_commit_message() ;
            }>"commit"</button>


        <hr/>

        <button
            on:click=move |_|
                {
                    log!("build") ;
                    let now : chrono::DateTime<chrono::Utc> = Utc::now();       // e.g. `2014-11-28T12:45:59.324310806Z`
                    let now = now.format("%Y-%m-%d-%H-%M-%S").to_string() ;
                    log!("build now : {}",now) ;
                    set_what_to_show.set(WhatToShow::OmakeStartBuild(now))
            }>"build"</button>


        <button
            on:click=move |_|
                {
                    set_what_to_show.set(WhatToShow::OmakeStdout) ;
            }>"omake output"</button>

        <button
            on:click=move |_|
                {
                    set_what_to_show.set(WhatToShow::OmakeProgress) ;
            }>"omake parsed output"</button>


    </div>
    </div>
    </div>

        }
}

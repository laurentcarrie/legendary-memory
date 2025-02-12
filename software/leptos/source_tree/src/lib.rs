use std::cmp::Ordering;
use human_sort::compare;use leptos::prelude::*;
use leptos::tachys::html::style::style;
use leptos_meta::*;
use leptos::logging::log;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = ace)]
    fn edit(s: &str);
}

pub mod input_model;

pub mod protocol;
use protocol::model::answer::{Choice, EChoice, SourceTree};


fn default_world() -> SourceTree {
    SourceTree{
         items: vec![],

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
            log!("size of tree : {}",tree.items.len()) ;
            Ok(tree)
        },
        _ => panic!("bad type"),
    }
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
            <Script src="/src-noconflict/ace.js"> </Script>
        </main>

            <Title text="songbook" />


            <pre id="editor">r#"

edit me...

            "#</pre>





            {view! {
            }}
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

                                items.into_iter().map(|i| {
                                    // let i=&(p.0) ; // item
                                    // let (expanded,set_expanded)=&(p.1) ; // signal
                                    // let expanded=&(p.1.0) ;
                                    let (expanded,set_expanded) = signal(false) ;
                                    view! {
                                        <li>
                                        <button on:click=move |_| { *set_expanded.write() = !expanded.get() ; log!("edit") ; edit("editor") ; } >
                                        {i.author.clone()} / {i.title.clone()}
                                        </button>
                                        <ul style:display=move || if expanded.get() { "block" } else { "none" }>
                                            <li> master json
                                            <ul><li>{ i.masterjsonfile.clone() }</li></ul>
                                            </li>

                                            <li>tex files
                                                <ul>
                                                { i.texfiles.clone().into_iter().map(|f| {
                                                    view! {
                                                        <li>
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
        }
}

use leptos::prelude::*;
use leptos::tachys::html::style::style;
use leptos_meta::*;
use leptos::logging::log;

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


pub fn fetch_example() -> impl IntoView {
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
            <Title text="songbook" />
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
                                let w = &w.items.iter().map(|w| (w.clone(),signal( false))).collect::<Vec<_>>() ;

                                view!{
                                <h1> "number of items : " </h1>
                                };

                                w.into_iter().map(|p| {
                                    let i=&(p.0) ; // item
                                    let expanded=&(p.1) ; // signal
                                    view! {
                                        <li>
                                        <button>
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

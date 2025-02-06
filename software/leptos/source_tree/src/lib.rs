use leptos::prelude::*;
use leptos::tachys::html::style::style;
use leptos_meta::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::path::PathBuf;
use thiserror::Error;

pub mod input_model;

use input_model::UserWorld;

pub mod protocol;
use protocol::model::answer::{Choice, EChoice, SourceTree};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Cat {
    url: String,
}

#[derive(Error, Clone, Debug)]
pub enum CatError {
    #[error("Please request more than zero cats.")]
    NonZeroCats,
}

type CatCount = usize;

// async fn fetch_cats(count: CatCount) -> Result<Vec<String>> {
//     if count > 0 {
//         gloo_timers::future::TimeoutFuture::new(1000).await;
//         // make the request
//         let res = reqwasm::http::Request::get(&format!(
//             "https://api.thecatapi.com/v1/images/search?limit={count}",
//         ))
//         .send()
//         .await?
//         // convert it to JSON
//         .json::<Vec<Cat>>()
//         .await?
//         // extract the URL field for each cat
//         .into_iter()
//         .take(count)
//         .map(|cat| cat.url)
//         .collect::<Vec<_>>();
//         Ok(res)
//     } else {
//         Err(CatError::NonZeroCats)?
//     }
// }

async fn fetch_world() -> Result<SourceTree> {
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let world = reqwasm::http::Request::get(&format!(
        "/scripts/request.sh?request=eyJjaG9pY2UiOnsiSXRlbVNvdXJjZVRyZWUiOiBudWxsfX0K",
    ))
    .send()
    .await?
    // convert it to JSON
    .json::<Choice>()
    .await?;
    // Ok(vec!["hello".to_string(),"world".to_string()])
    // console.log(&world) ;
    match world.choice {
        EChoice::ItemSourceTree(tree) => Ok(tree),
        _ => panic!("bad type"),
    }
}

// pub fn fetch_example() -> impl IntoView {
//     let (cat_count, set_cat_count) = signal::<CatCount>(1);
//
//     // we use new_unsync here because the reqwasm request type isn't Send
//     // if we were doing SSR, then
//     // 1) we'd want to use a Resource, so the data would be serialized to the client
//     // 2) we'd need to make sure there was a thread-local spawner set up
//     let cats = AsyncDerived::new_unsync(move || fetch_cats(cat_count.get()));
//
//     let fallback = move |errors: ArcRwSignal<Errors>| {
//         let error_list = move || {
//             errors.with(|errors| {
//                 errors
//                     .iter()
//                     .map(|(_, e)| view! { <li>{e.to_string()}</li> })
//                     .collect::<Vec<_>>()
//             })
//         };
//
//         view! {
//             <div class="error">
//                 <h2>"Error"</h2>
//                 <ul>{error_list}</ul>
//             </div>
//         }
//     };
//
//     let spreadable = style(("background-color", "AliceBlue"));
//
//     view! {
//         <div>
//             <label>
//                 "How many cats would you like?"
//                 <input
//                     type="number"
//                     prop:value=move || cat_count.get().to_string()
//                     on:input:target=move |ev| {
//                         let val = ev.target().value().parse::<CatCount>().unwrap_or(0);
//                         set_cat_count.set(val);
//                     }
//                 />
//
//             </label>
//             <Transition fallback=|| view! { <div>"Loading..."</div> } {..spreadable}>
//                 <ErrorBoundary fallback>
//                     <ul>
//                         {move || Suspend::new(async move {
//                             cats.await
//                                 .map(|cats| {
//                                     cats.iter()
//                                         .map(|s| {
//                                             view! {
//                                                 <li>
//                                                     <img src=s.clone()/>
//                                                 </li>
//                                             }
//                                         })
//                                         .collect::<Vec<_>>()
//                                 })
//                         })}
//
//                     </ul>
//                 </ErrorBoundary>
//             </Transition>
//         </div>
//     }
// }

pub fn fetch_example() -> impl IntoView {
    provide_meta_context();
    let (cat_count, set_cat_count) = signal::<CatCount>(1);

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

    let spreadable = style(("background-color", "AliceBlue"));

    view! {
            <Title text="songbook" />
            {view! {
            <div>
            </div>
            }}
            <div>
                   <Transition fallback=|| view! { <div>"Loading..."</div> } {..spreadable}>
                    <ErrorBoundary fallback>
                            <label>songs</label>
                            <ul>
                            {move || Suspend::new(async move {
                                let w = world.await.unwrap() ;
                                w.items.into_iter().map(|i| {
                                    view! {
                                        <li>

                                        {i.author.clone()} / {i.title.clone()}

                                        <ul>
                                            <li> master json
                                            <ul><li>{ i.masterjsonfile }</ul></li>
                                            </li>

                                            <li>tex files
                                                <ul>
                                                { i.texfiles.into_iter().map(|f| {
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
                                                { i.lyricstexfiles.into_iter().map(|f| {
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
                                                { i.lyfiles.into_iter().map(|f| {
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

                            <label>books</label>
                            <ul>
                            {move || Suspend::new(async move {
                                let w = world.await.unwrap() ;
                                // w.books.into_iter().map(|s| {
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

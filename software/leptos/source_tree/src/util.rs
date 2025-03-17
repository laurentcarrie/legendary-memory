use base64::prelude::BASE64_STANDARD;
use base64::prelude::*;
use leptos::logging::log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::protocol::model::answer;
use crate::protocol::model::request;

pub fn default_world() -> answer::SourceTree {
    answer::SourceTree { items: vec![] }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub enum WhatToShow {
    Nothing,
    SourceFile(String),
    SaveSourceFile(String, String),
    OmakeStartBuild(String),
    OmakeStdout,
    OmakeProgress,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub enum WhatToShowResult {
    Nothing,
    SourceFile(String, String),
    OmakeStdout(String, String),
    OmakeProgress(answer::Progress),
    OmakeBuildStarted(String),
    ProtocolError(String),
}

pub fn string_of_what_to_show(s: WhatToShow) -> String {
    match serde_json::to_string(&s) {
        Ok(s) => s,
        Err(e) => {
            log!("error in string_of_what_to_show {:?}", e);
            "".to_string()
        }
    }
}

pub fn what_to_show_of_string(s: String) -> WhatToShow {
    match serde_json::from_str::<WhatToShow>(&s) {
        Ok(o) => o,
        Err(e) => {
            log!("error in what_to_show_of_string {} {:?}", s, e);
            WhatToShow::Nothing
        }
    }
}

// convert base64 string to a SourceTreeItem
#[allow(non_snake_case)]
pub fn SourceTreeItem_of_base64(input: String) -> answer::SourceTreeItem {
    let data: Option<Vec<u8>> = BASE64_STANDARD.decode(input.as_str()).ok();
    let data: Option<String> = data.map(|c| String::from_utf8(c).ok()).flatten();
    let c: Option<answer::SourceTreeItem> = data
        .map(|s| serde_json::from_str(s.as_str()).ok())
        .flatten();
    match c {
        Some(c) => c,
        None => {
            log!(
                "{}:{} could not parse input string to SourceTreeItem : {}",
                file!(),
                line!(),
                &input
            );
            answer::SourceTreeItem {
                title: "error".to_string(),
                author: "error".to_string(),
                masterjsonfile: "error".to_string(),
                mastertexfile: "error".to_string(),
                texfiles: vec![],
                lyricstexfiles: vec![],
                lyfiles: vec![],
            }
        }
    }
}

// get all songs
pub async fn fetch_world() -> Result<answer::SourceTree> {
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let world = reqwasm::http::Request::get(&format!(
        "/scripts/request.sh?request=eyJjaG9pY2UiOnsiSXRlbVNvdXJjZVRyZWUiOiBudWxsfX0K",
    ))
    .send()
    .await?
    .json::<answer::Choice>()
    .await?;
    match world.choice {
        answer::EChoice::ItemSourceTree(tree) => {
            // log!("size of tree : {}", tree.items.len());
            Ok(tree)
        }
        _ => panic!("{}:{}, panic bad type", file!(), line!()),
    }
}

// pub async fn save_file(path: String, content: String) -> Result<()> {
//     log!("save file");
//     gloo_timers::future::TimeoutFuture::new(1000).await;
//     // make the request
//     let choice = request::EChoice::ItemSaveFile(request::InfoSaveFile {
//         path: path.clone(),
//         content: content.clone(),
//     });
//     log!("data is {:?}",&choice) ;
//     let request = request::Choice { choice: choice };
//     let json_string = serde_json::to_string(&request)?;
//     let b64 = BASE64_STANDARD.encode(&json_string);
//     let url = format!("/scripts/request.sh?request={}", b64);
//     let _ = reqwasm::http::Request::get(url.as_str())
//         .send()
//         .await?
//         .text()
//         .await?;
//     Ok(())
// }

/// calls the server to run a request
/// the request format is an instance of request::Choice, serialized as json
/// the return data is an instance of answer::Choice, serialized as json
pub async fn get_request(choice: request::Choice) -> Result<answer::Choice> {
    log!("{}:{} get_request", file!(), line!());
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    // let choice = request::Choice {
    //     choice: request::EChoice::ItemGetOMakeStdout,
    // };
    let json_string = serde_json::to_string(&choice).unwrap();
    let b64 = BASE64_STANDARD.encode(&json_string);
    let path = format!("/scripts/request.sh?request={}", &b64);
    let data = reqwasm::http::Request::get(path.as_str())
        .send()
        .await?
        .text()
        .await?;
    let data = serde_json::from_str::<answer::Choice>(&data)?;
    // log!("{:?}", &data);
    Ok(data)
}

/// a wrapper to get request, using local specific structures
pub async fn get_something_to_see(what: WhatToShow) -> Result<WhatToShowResult> {
    log!("{}:{}", file!(), line!());
    match what {
        // WhatToShow::Nothing => get_omake_stdout(),
        // WhatToShow::Nothing => get_file("xxx".to_string()),
        WhatToShow::SourceFile(path) => {
            let answer = get_request(request::Choice {
                choice: request::EChoice::ItemGetSourceFile(path),
            });
            if let answer::EChoice::ItemFileData(path, data) = answer.await?.choice {
                // Ok(answer::Choice{choice:answer::EChoice::ItemFileData("".to_string(),"".to_string())})
                Ok(WhatToShowResult::SourceFile(path, data))
            } else {
                Ok(WhatToShowResult::ProtocolError(format!(
                    "{}:{}",
                    file!(),
                    line!()
                )))
            }
        }
        WhatToShow::OmakeStartBuild(now) => {
            let answer = get_request(request::Choice {
                choice: request::EChoice::ItemBuild(now),
            });
            let choice = answer.await?.choice;
            match choice {
                answer::EChoice::ItemFileData(path, data) => {
                    // Ok(answer::Choice{choice:answer::EChoice::ItemFileData("".to_string(),"".to_string())})
                    Ok(WhatToShowResult::SourceFile(path, data))
                }
                x => Ok(WhatToShowResult::ProtocolError(format!(
                    "{}:{} {:?}",
                    file!(),
                    line!(),
                    x
                ))),
            }
        }
        WhatToShow::Nothing => {
            let _answer = get_request(request::Choice {
                choice: request::EChoice::ItemGetSourceFile("".to_string()),
            })
            .await?;
            Ok(WhatToShowResult::Nothing)
        }
        WhatToShow::OmakeStdout => {
            let answer = get_request(request::Choice {
                choice: request::EChoice::ItemGetOMakeStdout,
            });
            match answer.await?.choice {
                answer::EChoice::ItemFileData(path, data) => {
                    Ok(WhatToShowResult::OmakeStdout(path, data))
                }
                x => Ok(WhatToShowResult::ProtocolError(format!(
                    "{}:{} {:?}",
                    file!(),
                    line!(),
                    x
                ))),
            }
        }
        WhatToShow::OmakeProgress => {
            let answer = get_request(request::Choice {
                choice: request::EChoice::ItemGetOMakeProgress,
            });
            let choice = answer.await?.choice;
            if let answer::EChoice::ItemErrorMessage(message) = choice {
                Ok(WhatToShowResult::ProtocolError(message.to_string()))
            } else if let answer::EChoice::ItemSeeProgress(data) = choice {
                Ok(WhatToShowResult::OmakeProgress(data.clone()))
            } else {
                Ok(WhatToShowResult::ProtocolError(format!(
                    "{}:{}",
                    file!(),
                    line!()
                )))
            }
        }
        WhatToShow::SaveSourceFile(path, content) => {
            let answer = get_request(request::Choice {
                choice: request::EChoice::ItemSaveFile(request::InfoSaveFile {
                    path: path,
                    content: content,
                }),
            });
            match answer.await?.choice {
                answer::EChoice::ItemFileData(path, data) => Ok(WhatToShowResult::SourceFile(
                    path.to_string(),
                    data.to_string(),
                )),
                x => Ok(WhatToShowResult::ProtocolError(format!(
                    "{}:{}\n{:?}",
                    file!(),
                    line!(),
                    &x
                ))),
            }
        }
    }
}

pub async fn build(id: Option<String>) -> Result<String> {
    log!("build in util.ml, id = {:?}", id);
    match id {
        Some(id) => {
            let choice = request::Choice {
                choice: request::EChoice::ItemBuild(id),
            };
            let json_string = serde_json::to_string(&choice).unwrap(); //   echo "{\"choice\":{\"ItemBuild\": null}}" | base64
            let b64 = BASE64_STANDARD.encode(&json_string);
            let path = format!("/scripts/request.sh?request={}", &b64);
            log!("build, url is {}", &path);
            gloo_timers::future::TimeoutFuture::new(1000).await;
            // make the request
            let answer = reqwasm::http::Request::get(&path)
                .send()
                .await?
                .text()
                .await?;
            log!("{}:{} {}", file!(), line!(), answer);
            Ok(answer.to_string())
        }
        None => Ok(format!("{}:{}, huh ? no id ?", file!(), line!())),
    }
}

pub async fn omake_children_info() -> Result<String> {
    //   echo "{\"choice\":{\"ItemBuild\": null}}" | base64
    let path =
        "/scripts/request.sh?request=eyJjaG9pY2UiOiB7Ikl0ZW1PTWFrZUNoaWxkcmVuSW5mbyI6IG51bGx9fQo=";
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let data = reqwasm::http::Request::post(path)
        .send()
        .await?
        .text()
        .await?;
    Ok(data)
}

use base64::prelude::BASE64_STANDARD;
use base64::prelude::*;
use leptos::logging::log;
use leptos::prelude::*;
use serde::{Deserialize, Serialize};

use crate::protocol::model::answer::{Choice, EChoice, SourceTree, SourceTreeItem};
use crate::protocol::model::request;
use crate::format_omake::format_string ;

pub fn default_world() -> SourceTree {
    SourceTree { items: vec![] }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Hash, Clone)]
pub enum WhatToShow {
    Nothing,
    SourceFile(String),
    OmakeStdout,
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
            log!("error in what_to_show_of_string {}", s);
            WhatToShow::Nothing
        }
    }
}

// convert base64 string to a SourceTreeItem
#[allow(non_snake_case)]
pub fn SourceTreeItem_of_base64(input: String) -> SourceTreeItem {
    let data: Option<Vec<u8>> = BASE64_STANDARD.decode(input.as_str()).ok();
    let data: Option<String> = data.map(|c| String::from_utf8(c).ok()).flatten();
    let c: Option<SourceTreeItem> = data
        .map(|s| serde_json::from_str(s.as_str()).ok())
        .flatten();
    match c {
        Some(c) => c,
        None => {
            log!(
                "could not parse input string to SourceTreeItem : {}",
                &input
            );
            SourceTreeItem {
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
pub async fn fetch_world() -> Result<SourceTree> {
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
            // log!("size of tree : {}", tree.items.len());
            Ok(tree)
        }
        _ => panic!("bad type"),
    }
}

pub async fn save_file(path: String, content: String) -> Result<()> {
    log!("save file");
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let choice = request::EChoice::ItemSaveFile(request::InfoSaveFile {
        path: path.clone(),
        content: content.clone(),
    });
    let request = request::Choice { choice: choice };
    let json_string = serde_json::to_string(&request)?;
    let b64 = BASE64_STANDARD.encode(&json_string);
    let url = format!("/scripts/request.sh?request={}", b64);
    let _ = reqwasm::http::Request::get(url.as_str())
        .send()
        .await?
        .text()
        .await?;
    Ok(())
}

pub async fn get_file(path: String) -> Result<(String, String)> {
    log!("get file '{}'", &path);
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let choice = request::Choice {
        choice: request::EChoice::ItemGetSourceFile(path.clone()),
    };
    let json_string = serde_json::to_string(&choice).unwrap();
    let b64 = BASE64_STANDARD.encode(&json_string);
    let url = format!("/scripts/request.sh?request={}", &b64);
    let json = reqwasm::http::Request::get(url.as_str())
        .send()
        .await?
        .text()
        .await?;

    match serde_json::from_str::<Choice>(json.as_str()) {
        Ok(o) => match o.choice {
            EChoice::ItemFileData(data) => {
                log!("data : {}", &data);
                Ok((path, data))
            }
            _ => Ok((path, "wrong return type".to_string())),
        },
        Err(e) => {
            log!("error {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn get_omake_stdout() -> Result<(String, String)> {
    log!("get_omake_stdout");
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let choice = request::Choice {
        choice: request::EChoice::ItemGetOMakeStdout,
    };
    let json_string = serde_json::to_string(&choice).unwrap();
    let b64 = BASE64_STANDARD.encode(&json_string);
    let path = format!("/scripts/request.sh?request={}", &b64);
    let data = reqwasm::http::Request::get(path.as_str())
        .send()
        .await?
        .text()
        .await?;
    let data =  serde_json::from_str::<Choice>(&data) ;
    match data {
        Ok(x) => {
            match x.choice {
                EChoice::ItemFileData (data) => Ok(("omake.stdout".to_string(), format_string(data))),
                _ => Ok(("omake.stdout".to_string(), "bad type".to_string()))
            }
            },
        Err(e) => { log!("ERROR : {:?}",e) ; Err(e.into())}
    }
}

pub async fn get_something_to_see(what: WhatToShow) -> Result<(String, String)> {
    match what {
        // WhatToShow::Nothing => get_omake_stdout(),
        // WhatToShow::Nothing => get_file("xxx".to_string()),
        WhatToShow::SourceFile(path) => get_file(path).await,
        WhatToShow::OmakeStdout => get_omake_stdout() .await,
        WhatToShow::Nothing  => get_file("xxx".to_string()).await,
    }
}

pub async fn build(id: Option<String>) -> Result<()> {
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
            let _ = reqwasm::http::Request::get(&path)
                .send()
                .await?
                .text()
                .await?;
            Ok(())
        }
        None => Ok(()),
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

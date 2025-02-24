use base64::prelude::BASE64_STANDARD;
use base64::prelude::*;
use leptos::logging::log;
use leptos::prelude::*;

use crate::protocol::model::answer::{Choice, EChoice, SourceTree, SourceTreeItem};
use crate::protocol::model::request;

pub fn default_world() -> SourceTree {
    SourceTree { items: vec![] }
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

pub async fn get_file(path: String) -> Result<(String,String)> {
    log!("get file");
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let choice = request::Choice {
        choice: request::EChoice::ItemGetSourceFile(path),
    };
    let json_string = serde_json::to_string(&choice).unwrap();
    let b64 = BASE64_STANDARD.encode(&json_string);
    let path = format!("/scripts/request.sh?request={}", &b64);
    let json = reqwasm::http::Request::get(path.as_str())
        .send()
        .await?
        .json()
        .await?;
    match  serde_json::from_str::<Choice>(json)  {
        Ok(o) => {
            match o.choice {
                EChoice::ItemFileData(data) => Ok((path, data)),
                _ => Ok((path,"wrong return type".to_string()))
            }
        }
    }}




pub async fn get_omake_stdout() -> Result<String> {
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
    Ok(data)
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
        },
        None => Ok(())
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

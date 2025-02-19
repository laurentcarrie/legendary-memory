use leptos::prelude::*;
use leptos::logging::log;
use base64::prelude::BASE64_STANDARD;
use base64::prelude::*;

use crate::protocol::model::answer::{Choice, EChoice, SourceTree,SourceTreeItem};
use crate::protocol::model::request as request ;

pub fn default_world() -> SourceTree {
    SourceTree { items: vec![] }
}

// convert base64 string to a SourceTreeItem
#[allow(non_snake_case)]
pub fn SourceTreeItem_of_base64(input:String) -> SourceTreeItem {
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
            log!("size of tree : {}", tree.items.len());
            Ok(tree)
        }
        _ => panic!("bad type"),
    }
}




pub async fn save_file(path: String,content:String) -> Result<()> {
    log!("save file") ;
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let choice=request::EChoice::ItemSaveFile(request::InfoSaveFile{path:path.clone(),content:content.clone()}) ;
    let request= request::Choice{choice:choice} ;
    let json_string = serde_json::to_string(&request)? ;
    let b64= BASE64_STANDARD.encode(&json_string) ;
    let url = format!("/scripts/request.sh?request={}",b64) ;
    let _ = reqwasm::http::Request::get(url.as_str())
        .send()
        .await?
        .text()
        .await?;
    Ok(())
}



// get file data from url
pub async fn fetch_file(path: String) -> Result<String> {
    log!("fetch file {}",path) ;
    gloo_timers::future::TimeoutFuture::new(1000).await;
    // make the request
    let data = reqwasm::http::Request::get(path.as_str())
        .send()
        .await?
        .text()
        .await?;
    Ok(data)
}
use leptos::logging::log;
use base64::prelude::BASE64_STANDARD;
use base64::prelude::*;

pub mod protocol ;
use protocol::model::answer::{Choice, EChoice, SourceTree,SourceTreeItem};

pub fn default_world() -> SourceTree {
    SourceTree { items: vec![] }
}

pub fn base64_to_item(input:String) -> SourceTreeItem {
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
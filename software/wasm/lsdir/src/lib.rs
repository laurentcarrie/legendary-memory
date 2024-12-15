use reqwest::{Client} ;
use std::env ;
use serde::{Deserialize, Serialize};
use handlebars::Handlebars;
// use handlebars::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Universe {
    tree: Vec<String>,
}

// #[wasm_bindgen]
// impl Universe {
//     pub fn new() -> Universe {
//         Universe {
//             tree: vec!["a".to_string(), "b".to_string(), "c".to_string()],
//         }
//     }
// }

#[wasm_bindgen]
pub async fn fetch(url: &str) -> String {
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await.unwrap();
    resp.text().await.unwrap()
}

#[wasm_bindgen]
pub fn get_data() -> String {
    log("get_data") ;
    // let template =
    //     String::from_utf8(include_bytes!("tree.html").to_vec()).unwrap();
    // let mut h = Handlebars::new();
    // h.register_template_string("t1", template).unwrap();
    // let output_data = match h.render("t1", &u) {
    //     Ok(s) => s,
    //     Err(e) => format!("could not render template ; {:?}",e)
    // } ;
    // output_data
    // "xxxx".to_string()
    let mut ret = "".to_string() ;
    spawn_local(async move  {|ret|
         ret = fetch("http://185.247.117.231/input-songs/alannah_myles/black_velvet/body.tex").await ;
        log(ret.as_str()) ;
    }
    ) ;
    ret.clone()
}

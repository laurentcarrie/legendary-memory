use leptos::logging::log;
use leptos::prelude::*;
use std::path::PathBuf;
use wasm_bindgen::prelude::*;

use crate::util::WhatToShowResult;

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
pub fn Editor(wtsr: ReadSignal<WhatToShowResult>) -> impl IntoView {
    view! {
    <pre id="editor" style:display=move ||
        match wtsr.get() {
            WhatToShowResult::SourceFile(_,_) => "block",
            _ => "none"
        }>
    </pre>
        {
            move || {
                if let WhatToShowResult::SourceFile(path,data) = wtsr.get() {
                    let p = PathBuf::from(&path);
                    let extension: &str =
                    p.extension().map(|x| x.to_str()).flatten().unwrap_or("");
                    let mode = match extension {
                        "json" => "ace/mode/json",
                        "tex" => "ace/mode/latex",
                        "html" => "html",
                        _ => "ace/mode/text",
                    };
                    my_set_data("editor", data.clone().as_str(), mode, 80);
                }
            }
        }
    }
}

#[wasm_bindgen]
pub fn on_change_editor() -> () {
    // alert(&format!("Hello xxx !"));
    log!("{}:{} data has changed", file!(), line!());
    let document = leptos::leptos_dom::helpers::document();
    let e = document.get_element_by_id("b-save").unwrap();
    // e.set_property("foreground","red") ;
    e.style(("background-color", "red"));

    // let x = document.getElementById("b-save") ;
}

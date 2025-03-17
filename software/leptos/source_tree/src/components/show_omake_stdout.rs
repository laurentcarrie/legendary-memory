use leptos::prelude::*;
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
pub fn OmakeStdout(data: ReadSignal<WhatToShowResult>) -> impl IntoView {
    view! {
    <div id="omake-stdout" style:display=move ||
        match data.get() {
            WhatToShowResult::OmakeStdout(_,_) => "block",
            _ => "none"
        }>
        <pre>
        {
            move || {
            if let WhatToShowResult::OmakeStdout(_path,data) = data.get() {
                data
            } else {
                "".to_string()
            }
        }}
        </pre>
    </div>
    }
}

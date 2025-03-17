use leptos::prelude::*;

use crate::util::WhatToShowResult;

#[component]
pub fn ShowError(data: ReadSignal<WhatToShowResult>) -> impl IntoView {
    view! {
    <div id="showerror"  style:display=move ||
        match data.get() {
            WhatToShowResult::ProtocolError(_) => "block",
            _ => "none"
        }>
        <pre>
        {
            move || {
            if let WhatToShowResult::ProtocolError(msg) = data.get() {
                msg.replace("\\n","\n")
            } else {
                format!("huh ? {:?}",data.get()).to_string()
            }
        }}
        </pre>
    </div>
    }
}

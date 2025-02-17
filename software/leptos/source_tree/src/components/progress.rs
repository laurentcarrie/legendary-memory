use chrono::Utc;
use leptos::logging::log;
use leptos::prelude::*;

use crate::protocol::model::answer;

use crate::util::WhatToShowResult;

#[component]
pub fn Progress(wtsr: ReadSignal<WhatToShowResult>) -> impl IntoView {
    log!("{}:{}", file!(), line!());
    view! {
    <div id="progress" style:display=move ||
        match wtsr.get() {
            WhatToShowResult::OmakeProgress(_) => {
                "block"
            },
            _ => "none"
        }>
        <h1> Progress of omake build </h1>
        <p>this page is rendered by parsing omake build stdout </p>
        <p>books are only built when all songs are built. If a song cannot be built, a book that needs it will not appear on this page, even in the failed section</p>
        <p>click on omake output button to see the raw output</p>

        // omake progress {move || data.get().progress.len()} items

        {
            move || {
            let rows:Vec<answer::ProgressItem> = match wtsr.get() {
                WhatToShowResult::OmakeProgress(data) => data.progress,
                _ => vec![]
            } ;
            vec![
                    ("RUNNING","START ","running"),
                    ("FAILED","FAILED","failed"),
                    ("DONE",  "DONE  ","success")
            ].iter().map(|(label,pattern,class)| {
                    view! {
                        <h3>{label.to_string()}</h3>
                        <table>
                        <tr><th>type</th><th>target</th><th>start date</th><th>end date</th>
                        <th>duration</th>
                        </tr>

                        {
                            rows.iter().filter(|row| row.status==pattern.to_string()).map(|row| {
                            let duration = match row.end_date {
                                Some(ed) => ed - row.start_date ,
                                None => {
                                            chrono::Utc::now().timestamp() - row.start_date
                                }
                            } ;
                            let str_start_date = chrono::DateTime::<Utc>::from_timestamp(row.start_date,0).map_or("invalid timestamp".to_string(),|x| x.format("%Y-%m-%d %H:%M:%S").to_string()) ;
                            let str_end_date = row.end_date.map_or("".to_string(),|x| chrono::DateTime::<Utc>::from_timestamp(x,0).map_or("invalid timestamp".to_string(),|x| x.format("%Y-%m-%d %H:%M:%S").to_string()))  ;
                            let anchor = format!("/output{}/main.pdf.stdout",row.message.clone()) ;
                            view! {
                                <tr class={class.to_string()}>
                                    <td>{row.topic.clone()}</td><td><a href={anchor}>{row.message.clone()}</a></td>
                                    <td>{str_start_date}</td>
                                    <td>{str_end_date}</td>
                                    <td>{duration}</td>
                                </tr>
                            }
                        }
                        ).collect::<Vec<_>>()
                        }
                        </table>
                    }
                    }).collect::<Vec<_>>()
            }
        }
    </div>
    }
}

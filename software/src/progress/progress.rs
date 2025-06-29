use crate::protocol::model::answer::{Progress, ProgressItem};
use chrono;
use human_sort::compare;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn progress_of_string(data: String) -> Result<Progress, Box<dyn std::error::Error>> {
    let re = Regex::new(r"\[(.*)\]\[(.*)\]\[(.*)\]\[(.*)\]").unwrap();
    let mut most_recent_items: HashMap<(String, String), ProgressItem> = Default::default();

    for (_, [status, topic, message, date]) in re.captures_iter(&data).map(|c| c.extract()) {
        let previous_item = most_recent_items.get(&(topic.to_string(), message.to_string()));
        let naive = chrono::NaiveDateTime::parse_from_str(date, "%Y-%m-%d %H:%M:%S");
        if naive.is_err() {
            return Err(format!("cannot parse date '{}'", date).into());
        };
        let naive = naive.unwrap().and_utc().timestamp();
        // let now = chrono::Utc::now().timestamp();
        let (start_date, end_date) = match previous_item {
            Some(p) => (&p.start_date, Some(naive)),
            None => (&naive, None),
        };
        // let duration = now - start_date;
        let item = ProgressItem {
            status: status.to_string(),
            topic: topic.to_string(),
            message: message.to_string(),
            start_date: start_date.clone(),
            end_date: end_date.clone(),
        };
        // for a given item, over time we have (pdf,asong,START) => (pdf,asong,SUCCESS) for instance, so the the first one is overwritten
        most_recent_items.insert((topic.to_string(), message.to_string()), item);
    }

    let mut items: Vec<ProgressItem> = vec![];
    for (_, item) in most_recent_items {
        items.push(item);
    }
    let n = items.len();
    items.sort_by(|a, b| match compare(&a.topic, &b.topic) {
        Ordering::Equal => {
            if a.start_date < b.start_date {
                Ordering::Less
            } else if a.start_date == b.start_date {
                compare(a.message.as_str(), b.message.as_str())
            } else {
                Ordering::Greater
            }
        }
        other => other,
    });
    assert!(n == items.len());

    Ok(Progress { progress: items })
}

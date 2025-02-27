use crate::protocol::model::answer::{Progress, ProgressItem};
use regex::Regex;

pub fn progress_of_string(data: String) -> Progress {
    let re = Regex::new(r"\[(.*)\]\[(.*)\]\[(.*)\]").unwrap();
    let mut items: Vec<ProgressItem> = vec![];
    for (_, [status, topic, message]) in re.captures_iter(&data).map(|c| c.extract()) {
        items.push(ProgressItem {
            status: status.to_string(),
            topic: topic.to_string(),
            message: message.to_string(),
        });
    }
    Progress { progress: items }
}

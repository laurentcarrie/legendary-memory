use string_replace_all::StringReplaceAll;
use regex::Regex ;


pub fn format_string(input: &String) -> String {
    let re = Regex::new(r"\[(.*)\]\[(.*)\]\[(.*)\]").unwrap();
    let lines = input.split("\n") ;
    let ret = lines.filter_map(|line| {
        if let Some(ss) = re.captures(&line) {
            let (status,topic,message) = ss ;
            log::info!("[{}] [{}] [{}]",&status,&topic,&message) ;
            Some("xx")
        } else {
            None
        }
        })
        .collect::<Vec<_>>()
        .join("/n")
    ;

    ret
}
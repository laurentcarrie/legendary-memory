use base64::prelude::BASE64_STANDARD;
use base64::prelude::*;
// pub mod protocol ;
pub mod config;
pub mod errors;
pub mod helpers;
pub mod protocol;

use log::LevelFilter;
use std::env;
use zmq::Socket;
// use crate::protocol::model ;
use crate::protocol::model::{answer, request};

pub fn convert(data: &[u32; 1]) -> [u8; 4] {
    let mut res = [0; 4];
    for i in 0..1 {
        res[4 * i..][..4].copy_from_slice(&data[i].to_le_bytes());
    }
    res
}

fn send_request(requester: Socket, choice: request::Choice) {
    let buffer = &mut [0; 1000000];
    let s = serde_json::to_string(&choice).unwrap();
    log::info!("send request");
    requester.send(&s, 0).unwrap();
    log::info!("request sent");
    let len = requester.recv_into(buffer, 0).unwrap();
    log::info!("answer of size {}", len);
    let answer_str = String::from_utf8(buffer.to_vec().into_iter().take(len).collect()).unwrap();
    let answer: Result<answer::Choice, serde_json::Error> =
        serde_json::from_str(answer_str.as_str());
    match answer {
        Ok(_answer) => {
            // dbg!(&answer);
            println!("{}", answer_str);
        }
        Err(e) => {
            log::error!("answer is : '{}' {:?}", &answer_str, &e);
            log::error!("could not parse json value returned by server");
            // dbg!(&e);
        }
    };
    // println!("Received World {:?}", request_nbr);
    // }
    log::info!("DONE");
    ()
}

fn main() {
    // SimpleLogger::new().init().unwrap();
    // stderrlog::new().module(module_path!()).init().unwrap();

    stderrlog::new()
        .module(module_path!())
        .quiet(false)
        .verbosity(LevelFilter::Debug)
        // .timestamp(opt.ts.unwrap_or(stderrlog::Timestamp::Off))
        .init()
        .unwrap();
    // let _ = simple_logging::log_to_file("test.log", LevelFilter::Info);
    log::info!("start client");

    let mut args = env::args();
    let _ = args.next().expect("arg0 should be the name of the program"); // pop arg0
    let query = args.next().expect("at least one arg"); // we want arg1
    dbg!(&query);
    let query = BASE64_STANDARD.decode(&query).expect("valid base64 string");
    let query = String::from_utf8(query).expect("utf8 string");
    dbg!(&query);
    // // we will transform that to string again, we use serde_json to valide the data
    let choice: request::Choice = serde_json::from_str(query.as_str()).expect("uwnrap json value");

    let context = zmq::Context::new();
    let requester = context.socket(zmq::REQ).unwrap();
    assert!(requester.connect("tcp://localhost:5555").is_ok());
    send_request(requester, choice);
    // for request_nbr in 0..1 {
}

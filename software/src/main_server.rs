// use async_process::Stdio;
// use file_rotate::{compression::Compression, suffix::AppendCount, ContentLimit, FileRotate};
// use log::LevelFilter;
use crate::errors::MyError;
use simple_logger::SimpleLogger;
use sysinfo;
// use std::process::{Command, Stdio};
use crate::config::model::StructureItemContent::{ItemChords, ItemHRule, ItemRef};
use crate::config::model::World;
use crate::config::world::make;
use async_process::Command;
use backtrace::Backtrace;
use std::path::PathBuf;
use std::{env, thread, time};
use sysinfo::Pid;

pub mod config;
pub mod errors;
pub mod generate;
pub mod helpers;
pub mod makefiles;
pub mod protocol;
// use crate::protocol::model ;
use crate::generate::all::generate_all;
use crate::protocol::model::answer::{Progress, ProgressItem, SourceTree, SourceTreeItem};
use crate::protocol::model::{answer, request};

pub async fn generate(
    songdir: PathBuf,
    bookdir: PathBuf,
    builddir: PathBuf,
) -> Result<(), MyError> {
    log::info!("generate");
    generate_all(songdir, bookdir, builddir)?;
    // let output = Command::new("/var/www/songbook/scripts/songbook")
    //     .arg(songdir)
    //     .arg(bookdir)
    //     .arg(builddir)
    //     .stdout(Stdio::piped())
    //     .reap_on_drop(true)
    //     .output()
    //     .await?;
    // log::info!("OUTPUT : {}", String::from_utf8(output.stdout)?);
    // log::info!("OUTPUT : {:?}",output.stderr) ;
    // let mut p = PathBuf::from(&builddir);
    // p.push("stdout.generate.txt");
    // let mut fout = File::create(p)?;
    // fout.write(&output.stdout).unwrap();
    Ok(())

    // dbg!(&command) ;

    // let binding = binding.arg("-R")
    //     .arg("/home/laurent/work/legendary-memory/build") ;

    // let binding = binding
    //     .spawn()?
    //     .stdout
    //     .ok_or_else(|| Error::new(ErrorKind::Other, "Could not capture standard output."))?;
    // // let output = binding.wait_with_output().expect("Failed to read stdout");
    //
    // let reader = BufReader::new(binding);
    //
    // reader
    //     .lines()
    //     .filter_map(|line| line.ok())
    //     // .filter(|line| line.find("usb").is_some())
    //     .for_each(|line| {
    //         let now = Utc::now();
    //         eprintln!("(generate build tree {} ) >>> {}", now, line) ;
    //         let _ = output.write(line.as_bytes()).unwrap() ;
    //         let _ = output.write("\n".as_bytes()).unwrap() ;
    //     });
    //
    // match command.status().unwrap().code() {
    //     Some(0) => Ok(()),
    //     // _ => Err(MyError::ProcessError("docker push".to_string())),
    //     _ => Err(Error::new(
    //         ErrorKind::Other,
    //         "command exited with code != 0.",
    //     )),
    // }
}

pub async fn omake(builddir: PathBuf) -> Result<u32, MyError> {
    log::info!("omake in {:?}", &builddir);
    let mut sh = builddir
        .clone()
        .parent()
        .ok_or(MyError::MessageError("what, no parent ?".to_string()))?
        .to_path_buf();
    sh.push("scripts");
    sh.push("omake.sh");
    dbg!(&sh);
    let child = Command::new(sh)
        // .arg("-j")
        // .arg("8")
        // .arg("-k")
        // .env("PATH", "/bin")
        // .env("PATH", "/bin")
        .current_dir(&builddir)
        // .stdout(Stdio::piped())
        .reap_on_drop(true)
        .spawn()?;
    log::info!("omake spawned {}", &child.id());
    // log::info!("OUTPUT : {}", String::from_utf8(child.stdout)?);
    // log::info!("OUTPUT : {:?}", child.stderr);
    // println!("{}",String::from_utf8_lossy(&child.output.stdout)) ;
    // thread::sleep(time::Duration::from_secs(10));
    let pid = child.id();
    Ok(pid)
}

pub async fn handle_build_request(
    songdir: PathBuf,
    bookdir: PathBuf,
    builddir: PathBuf,
) -> Result<answer::EChoice, MyError> {
    log::info!(
        "generate from {:?} ; {:?} to {:?}",
        &songdir,
        &bookdir,
        &builddir
    );
    // let mut logpath = Path::new(&config.builddir).canonicalize().expect("root");
    // logpath.push("build.log");
    generate(songdir.clone(), bookdir.clone(), builddir.clone()).await?;
    log::info!("generate done");
    let pid = omake(builddir).await?;
    Ok(answer::EChoice::ItemOmakeBuild(pid))
}

pub async fn handle_omake_children_info() -> Result<answer::EChoice, MyError> {
    let s = sysinfo::System::new_all();
    let omake_pids = helpers::process::find_pids_from_name("omake".to_string())?;

    if omake_pids.is_empty() {
        let v: Vec<answer::ChildInfo> = vec![];
        return Ok(answer::EChoice::ItemOMakeOmakeChildren(v));
    }

    let omake_pid = omake_pids
        .first()
        .ok_or(MyError::MessageError("internal error".to_string()))?;

    match s.process(sysinfo::Pid::from(Pid::from_u32(*omake_pid))) {
        None => Ok(answer::EChoice::ItemErrorMessage(format!("no such pid"))),
        Some(_) => {
            let children = helpers::process::get_descendents(*omake_pid).unwrap();
            let v: Vec<_> = children
                .iter()
                .filter_map(|child_pid| {
                    match s.process(sysinfo::Pid::from(Pid::from_u32(*child_pid))) {
                        Some(child) => {
                            if helpers::process::get_children(*child_pid).unwrap().len() == 0 {
                                let cwd = child
                                    .cwd()
                                    .map(|s| s.to_str())
                                    .flatten()
                                    .map(|s| String::from(s));
                                let name = String::from(child.name().to_str().unwrap());
                                Some(answer::ChildInfo {
                                    pid: child.pid().as_u32(),
                                    cwd: cwd,
                                    name: name,
                                    run_time: child.run_time(),
                                })
                                // log::info!("{:?}", child.name());
                                // log::info!(".....{:?}", child.cwd());
                                // log::info!(".....{:?}", child.cpu_usage());
                                // log::info!(".....{:?}", child.start_time());
                                // log::info!(".....{:?}", child.run_time());
                            } else {
                                None
                            }
                        }
                        None => {
                            log::error!("no process for {}", child_pid);
                            None
                        }
                    }
                })
                .collect();
            Ok(answer::EChoice::ItemOMakeOmakeChildren(v))
        }
    }
}

pub fn handle_omake_kill() -> Result<answer::EChoice, MyError> {
    let s = sysinfo::System::new_all();
    // @todo : use self pid
    let omake_pids = helpers::process::find_pids_from_name("omake".to_string()).unwrap();
    for pid in omake_pids {
        let p = s.process(Pid::from_u32(pid));
        match p {
            None => (),
            Some(p) => {
                p.kill();
                ()
            }
        }
    }

    Ok(answer::EChoice::ItemOkMessage)
}

pub fn handle_clean_build_tree(builddir: PathBuf) -> Result<answer::EChoice, MyError> {
    let paths = vec!["delivery", "songs", "books"];
    let _ret: Vec<_> = paths
        .iter()
        .map(|p| {
            let mut path_to_delete: PathBuf = builddir.clone();
            path_to_delete.push(p);
            log::info!("{:?}", &path_to_delete);
            std::fs::remove_dir_all(path_to_delete.as_os_str())
        })
        .collect();

    Ok(answer::EChoice::ItemOkMessage)
}

pub fn handle_build_progress(
    _srcdir: PathBuf,
    _builddir: PathBuf,
) -> Result<answer::EChoice, MyError> {
    // let world: UserWorld = {
    //     let mut filepath = PathBuf::new();
    //     filepath.push(builddir.as_str());
    //     filepath.push("world.json");
    //     serde_json::from_str(fs::read_to_string(filepath.as_path())?.as_str())?
    // };
    let ret: Vec<ProgressItem> = vec![];
    // for s in world.songs {
    //     let mut path = PathBuf::from(&builddir.as_str());
    //     let path2 = PathBuf::from(
    //         s.path
    //             .replace(format!("{}/", &srcdir.as_str()).as_str(), ""),
    //     );
    //     path.push("songs");
    //     let path = path.join(&path2);
    //     let mut p = PathBuf::from(path)
    //         .parent()
    //         .ok_or_else(|| MyError::MessageError("cannot get parent".to_string()))?
    //         .to_path_buf();
    //     p.push("main.pdf.stdout");
    //     let status = p.exists();
    //     ret.push(ProgressItem {
    //         path: p
    //             .to_str()
    //             .ok_or_else(|| MyError::MessageError("filename cast error".to_string()))?
    //             .to_string(),
    //         status: status,
    //     })
    // }
    Ok(answer::EChoice::ItemSeeProgress(Progress { progress: ret }))
}

pub fn handle_source_tree(
    songdir: PathBuf,
    bookdir: PathBuf,
    builddir: PathBuf,
) -> Result<answer::EChoice, MyError> {
    let world: World = make(&songdir, &bookdir, &builddir);

    let mut ret: Vec<SourceTreeItem> = vec![];
    for song in world.songs {
        dbg!(&song.srcdir);
        let mut texfiles: Vec<String> = vec![];
        let mut lyricstexfiles: Vec<String> = vec![];
        let mut lyfiles: Vec<String> = vec![];
        let masterjsonfile = format!("{}/song.json", song.srcdir.to_string());
        for f in &song.texfiles {
            texfiles.push(format!("{}/{}", song.srcdir.to_string(), f));
        }
        for section in &song.structure {
            match &section.item {
                ItemChords(c) => {
                    lyricstexfiles.push(format!(
                        "{}/lyrics/{}.tex",
                        song.srcdir.to_string(),
                        c.section_id
                    ));
                }
                ItemRef(c) => {
                    lyricstexfiles.push(format!(
                        "{}/lyrics/{}.tex",
                        song.srcdir.to_string(),
                        c.section_id
                    ));
                }
                ItemHRule(_) => {}
            }
        }
        for lyfile in &song.lilypondfiles {
            lyfiles.push(format!("{}/{}", song.srcdir.to_string(), lyfile));
        }
        // for texfile in &s.texfiles {
        //     let path = PathBuf::from(song.path.as_str());
        //     let path = path
        //         .parent()
        //         .ok_or(MyError::MessageError("internal error".to_string()))?;
        //     let mut path = path.to_path_buf();
        //     path.push(texfile);
        //     ret.push(path.as_path().to_str().unwrap().to_string());
        // }
        //
        ret.push(SourceTreeItem {
            title: song.title.clone(),
            author: song.author.clone(),
            masterjsonfile: masterjsonfile,
            lyricstexfiles: lyricstexfiles,
            lyfiles: lyfiles,
            texfiles: texfiles,
        });
    }
    Ok(answer::EChoice::ItemSourceTree {
        0: SourceTree { items: ret },
    })
}

#[tokio::main]
async fn main() -> () {
    let _ = SimpleLogger::new().init().unwrap();
    // let _ = simple_logging::log_to_file("/var/log/songbook/songbook.log", LevelFilter::Info);
    // let log_path = directory.join("/var/log/songbook/songbook.log");

    // let mut log = FileRotate::new(
    //     "/var/log/songbook/songbook.log",
    //     // log_path.clone(),
    //     AppendCount::new(2),
    //     ContentLimit::Lines(3),
    //     Compression::None,
    //     #[cfg(unix)]
    //     None,
    // );

    // log::set_max_level(LevelFilter::Debug);
    let mut args: std::env::Args = env::args();
    log::info!("found {} args on command line", args.len());
    let songdir = PathBuf::from(args.nth(1).unwrap());
    log::info!("songdir : {:?}", songdir);
    let bookdir = PathBuf::from(args.nth(0).unwrap());
    log::info!("bookdir : {:?}", bookdir);
    let builddir = PathBuf::from(args.nth(0).unwrap());
    log::info!("builddir : {:?}", builddir);

    let context = zmq::Context::new();
    let responder = context.socket(zmq::REP).unwrap();
    assert!(responder.bind("tcp://*:5555").is_ok());

    log::info!("start server...");

    loop {
        let bt = Backtrace::new();
        let buffer = &mut [0; 1000000];
        log::info!("wait for command...");
        let len = responder.recv_into(buffer, 0).unwrap();
        let command = String::from_utf8(buffer.to_vec().into_iter().take(len).collect()).unwrap();
        // dbg!(& command) ;
        let what: request::Choice = serde_json::from_str(&command).unwrap();
        dbg!(&what);
        log::info!("received command");
        let answer_choice = match what.choice {
            request::EChoice::ItemBuild => {
                handle_build_request(songdir.clone(), bookdir.clone(), builddir.clone()).await
            }
            request::EChoice::ItemOMakeChildrenInfo => {
                log::info!("request check pid");
                handle_omake_children_info().await
            }
            request::EChoice::ItemOMakeKill => handle_omake_kill(),
            request::EChoice::ItemCleanBuildTree => handle_clean_build_tree(builddir.clone()),
            request::EChoice::ItemHealthCheck => Ok(answer::EChoice::ItemHealthOk),
            request::EChoice::ItemSeeProgress => {
                handle_build_progress(songdir.clone(), builddir.clone())
            }
            request::EChoice::ItemSourceTree => {
                handle_source_tree(songdir.clone(), bookdir.clone(), builddir.clone())
            }
        };
        let answer = match answer_choice {
            Ok(x) => {
                let answer = answer::Choice { choice: x };
                answer
            }
            Err(e) => answer::Choice {
                choice: answer::EChoice::ItemErrorMessage(format!(
                    "{:?} ; {:?}",
                    e.to_string(),
                    &bt
                )),
            },
        };
        log::info!("DONE !");
        // log::info!("send response");
        // let answer = answer::Choice {
        //     choice: answer::EChoice::ItemOMakeOmakeChildren(omake_pid),
        // };
        match serde_json::to_string(&answer) {
            Ok(s) => {
                dbg!(&answer);
                responder.send(s.as_str(), 0).unwrap();
            }
            Err(e) => {
                log::error!("could not serialize {:?}", &answer);
                log::error!("{}", e);
            }
        }
        log::info!("response sent");
        thread::sleep(time::Duration::from_secs(1));
    }
}

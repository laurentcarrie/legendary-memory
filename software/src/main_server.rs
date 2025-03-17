use human_sort::compare;
use regex::Regex;
// use async_process::Stdio;
// use async_process::Stdio;
// use file_rotate::{compression::Compression, suffix::AppendCount, ContentLimit, FileRotate};
// use log::LevelFilter;
use crate::errors::MyError;
use log::LevelFilter;
use simple_logger::SimpleLogger;
use sysinfo;
// use std::process::{Command, Stdio};
use crate::model::model::StructureItemContent::{ItemChords, ItemHRule, ItemRef};
use crate::model::model::World;
use crate::model::world::make;
use async_process::Command;
use backtrace::Backtrace;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::{env, fs, thread, time};
use sysinfo::Pid;
pub mod errors;
pub mod generate;
pub mod helpers;
pub mod makefiles;
pub mod model;
pub mod progress;
pub mod protocol;
// use crate::protocol::model ;
use crate::generate::all::generate_all;
use crate::protocol::model::answer::{Progress, ProgressItem, SourceTree, SourceTreeItem};
use crate::protocol::model::request::InfoSaveFile;
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

pub async fn omake(
    id: String,
    songdir: PathBuf,
    bookdir: PathBuf,
    builddir: PathBuf,
) -> Result<u32, MyError> {
    log::info!("omake id={}, builddir={:?}", &id, &builddir);
    let mut sh = builddir
        .clone()
        .parent()
        .ok_or(MyError::MessageError("what, no parent ?".to_string()))?
        .to_path_buf();
    // sh.push("scripts");
    sh.push(&builddir.to_str().unwrap());
    sh.push("omake.sh");
    let sh = sh
        .to_str()
        .ok_or(MyError::MessageError("cannot get omake string".to_string()))?;
    let child = Command::new("bash")
        .arg(sh)
        .arg(id.as_str())
        // .arg("8")
        // .arg("-k")
        // .env("PATH", "/bin")
        // .env("PATH", "/bin")
        .env(
            "html_output",
            format!("{}/progress.{}.html", &builddir.to_str().unwrap(), &id),
        )
        .env(
            "nocolor_output",
            format!("{}/omake.{}.txt", &builddir.to_str().unwrap(), &id),
        )
        .env("omake_output_format", "text")
        .env("songdir", &songdir.to_str().unwrap())
        .env("bookdir", &bookdir.to_str().unwrap())
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
    id: String,
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
    let pid = omake(id, songdir.clone(), bookdir.clone(), builddir.clone()).await?;
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
    let world: World = make(&songdir, &bookdir, &builddir)?;

    let mut ret: Vec<SourceTreeItem> = vec![];
    for song in world.songs {
        dbg!(&song.srcdir);
        // let root = song.srcdir.to_string();
        let root = {
            let root = song.srcdir.replace(songdir.to_str().unwrap(), "");
            // let root = format!("/input-songs{}", root);
            root
        };
        log::info!("ROOT is {}", &root);
        let root = root.replace(songdir.to_str().unwrap(), "");
        let mut texfiles: Vec<String> = vec![];
        let mut lyricstexfiles: Vec<String> = vec![];
        let mut lyfiles: Vec<String> = vec![];
        let masterjsonfile = format!("{}/song.json", root);
        let mastertexfile = format!("{}/body.tex", root);
        for f in &song.texfiles {
            texfiles.push(format!("{}/{}", root, f));
        }
        for section in &song.structure {
            match &section.item {
                ItemChords(c) => {
                    lyricstexfiles.push(format!("{}/lyrics/{}.tex", root, c.section_id));
                }
                ItemRef(c) => {
                    lyricstexfiles.push(format!("{}/lyrics/{}.tex", root, c.section_id));
                }
                ItemHRule(_) => {}
            }
        }
        for lyfile in &song.lilypondfiles {
            lyfiles.push(format!("{}/{}", root, lyfile));
        }
        ret.push(SourceTreeItem {
            title: song.title.clone(),
            author: song.author.clone(),
            masterjsonfile: masterjsonfile,
            mastertexfile: mastertexfile,
            lyricstexfiles: lyricstexfiles,
            lyfiles: lyfiles,
            texfiles: texfiles,
        });
    }
    Ok(answer::EChoice::ItemSourceTree {
        0: SourceTree { items: ret },
    })
}

pub fn handle_save_file(songdir: PathBuf, info: InfoSaveFile) -> Result<answer::EChoice, MyError> {
    log::info!("{}:{} {:?}", file!(), line!(), &info);
    let re = Regex::new(r"/(.*)").unwrap();
    let relpath = re.replace(info.path.as_str(), "${1}").to_string();
    log::info!("{}:{} {:?}", file!(), line!(), &relpath);
    let mut path = songdir;
    path.push(relpath);
    log::info!("{}:{} {:?}", file!(), line!(), &path);
    let mut output = File::create(path)?;
    let _ = output.write(info.content.as_bytes()).unwrap();
    Ok(answer::EChoice::ItemOkMessage)
}

/// returns stdout and stderr for the most recent omake runs
/// each run is redirected to omake.<date>.stdout and omake.<date>.stderr
/// note that we use text compare for sorting dates... so don't change the format
fn get_omake_stdout_data(builddir: PathBuf) -> (String, String) {
    let mut candidates: Vec<PathBuf> = vec![];
    for p in builddir.read_dir().expect("read dir failed") {
        if let Ok(p) = p {
            if let Ok(file_type) = p.file_type() {
                if file_type.is_file() {
                    let re = Regex::new(r"omake\..*\.stdout").unwrap();
                    if re.is_match(p.file_name().as_os_str().to_str().unwrap()) {
                        candidates.push(p.path());
                    }
                }
            }
        }
    }
    candidates.sort_by(|a, b| {
        // let x = a.file_name().unwrap().to_str().unwrap();
        compare(
            b.file_name().unwrap().to_str().unwrap(),
            a.file_name().unwrap().to_str().unwrap(),
        )
    });
    let (data_stdout, data_stderr) = match candidates.first() {
        None => ("no build yet".to_string(), "".to_string()),
        Some(p) => {
            let data_stdout = fs::read_to_string(p).unwrap();

            let stderrpath = {
                let re = Regex::new(r"(.*)stdout(.*)").unwrap();
                let strpath = p.to_str().unwrap();
                let s = re.replace(strpath, "${1}stderr${2}").to_string();
                PathBuf::from(s)
            };
            let data_stderr = fs::read_to_string(stderrpath).unwrap();
            (data_stdout, data_stderr)
        }
    };
    (data_stdout, data_stderr)
}

pub fn handle_get_omake_stdout(builddir: PathBuf) -> Result<answer::EChoice, MyError> {
    let (data_stdout, data_stderr) = get_omake_stdout_data(builddir);

    Ok(answer::EChoice::ItemFileData(
        "omake.stdout".to_string(),
        format!("{}\n{}", data_stderr, data_stdout),
    ))
}

pub fn handle_get_omake_progress(builddir: PathBuf) -> Result<answer::EChoice, MyError> {
    let (data, _) = get_omake_stdout_data(builddir);
    let progress = crate::progress::progress::progress_of_string(data);

    Ok(answer::EChoice::ItemSeeProgress(progress?))
}

pub fn handle_get_source_file(songdir: PathBuf, spath: String) -> Result<answer::EChoice, MyError> {
    log::info!("{:?}", songdir);
    log::info!("{:?}", spath);
    let mut path = songdir.clone();
    let spath = PathBuf::from(spath);
    let spath = if spath.is_absolute() {
        let mut spath = spath.to_str().unwrap().to_string();
        if spath.len() > 0 {
            spath.remove(0);
        };
        PathBuf::from(spath)
    } else {
        spath
    };
    path.push(&spath);
    log::info!("get source file '{:?}'", &path);
    let data = match fs::read_to_string(path) {
        Ok(data) => data,
        Err(e) => format!("{:?}", e),
    };
    Ok(answer::EChoice::ItemFileData(
        spath.as_path().to_str().unwrap().to_string(),
        data,
    ))
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

    log::set_max_level(LevelFilter::Info);
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
        let what: request::Choice = serde_json::from_str(&command).unwrap();
        log::info!("{:?}", &what);
        log::info!("received command");
        let answer_choice = match what.choice {
            request::EChoice::ItemBuild(id) => {
                handle_build_request(id, songdir.clone(), bookdir.clone(), builddir.clone()).await
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
            request::EChoice::ItemSaveFile(info) => handle_save_file(songdir.clone(), info.clone()),
            request::EChoice::ItemGetOMakeStdout => handle_get_omake_stdout(builddir.clone()),
            request::EChoice::ItemGetSourceFile(path) => {
                handle_get_source_file(songdir.clone(), path)
            }
            request::EChoice::ItemGetOMakeProgress => handle_get_omake_progress(builddir.clone()),
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

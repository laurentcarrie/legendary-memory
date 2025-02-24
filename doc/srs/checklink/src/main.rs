use std::process::exit;
use log::LevelFilter;
use regex::Regex;
use std::ffi::OsStr;
use simple_logger::SimpleLogger;
use std::env;
use std::path::Path;
use std::path::PathBuf;

fn walk(path: &Path) -> Vec<PathBuf> {
    let mut acc: Vec<PathBuf> = vec![];
    // log::info!("ENTER {:?}",&path) ;
    for p in path.read_dir().expect("read dir failed") {
        if let Ok(p) = p {
            if let Ok(file_type) = p.file_type() {
                if file_type.is_dir() {
                    let mut other = walk(p.path().as_path());
                    acc.append(&mut other);
                } else if file_type.is_file() {
                    if p.path().extension() == Some(OsStr::new("md")) {
                        // log::info!("... file {:?}",&p);
                        acc.push(p.path());
                    }
                }
            }
        }
    }
    acc
}

fn check_reference(target_file:&PathBuf,target_link:&str) -> bool {
    let data = std::fs::read_to_string(&target_file);
    if let Ok(data) = data {
        let re = format!(r###"<a id="{}"/>"###,target_link) ;
        // log::info!("re : {}",&re) ;
        let re = Regex::new(&re).unwrap() ;
        match re.find(&data) {
            Some(_) => true,
            None => false
        }
    } else {
        log::error!("error while reading {:?}",&target_file) ;
        false
    }
}

fn check_md_file(p:PathBuf) -> bool {
    let mut ok = true ;
    // log::info!("check file {:?}",&p) ;
    let data = std::fs::read_to_string(&p).expect("read file") ;
    // (render.md#sections)
    let re = Regex::new(r"\((.*?)#(.*?)\)").unwrap();
    // let re = Regex::new(r"\\((.*?)\\)").unwrap();
    // let mut results : Vec<String> = vec![];
    for (_, [target_file,target_link]) in re.captures_iter(&data).map(|c| c.extract()) {
        // log::info!("{:?} {:?}",target_file,target_link) ;
        let target_path = match target_file {
            "" => p.clone(),
            s => {
                let  x = p.as_path().parent().expect("has parent") ;
                let mut y = PathBuf::from(x) ;
                y.push(s) ;
                y
            }
        } ;
        let this_ok = check_reference(&target_path,&target_link) ;
        if ! this_ok {
           log::error!("in {:?}, unresolved link : {:?} {:?}",&p,&target_path,&target_link)
        }
        ok = ok && this_ok ;
    } ;
    ok
}


fn main() {
    let _ = SimpleLogger::new().init().unwrap();
    log::set_max_level(LevelFilter::Error);

    let mut args: std::env::Args = env::args();
    log::info!("found {} args on command line", args.len());
    let root = PathBuf::from(args.nth(1).unwrap());
    let mdfiles = walk(root.as_path());
    let mut ok = true ;
    for p in mdfiles {
        ok = ok && check_md_file(p);
    }
    if !ok {
        exit(1)
    }
}

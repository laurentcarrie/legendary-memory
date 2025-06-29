use crate::helpers::digest::{digest_has_changed, write_digest};
use crate::helpers::io::read_to_string;
use crate::model::use_model as M;
use regex::Regex;
use std::fs::File;
use std::path::PathBuf;
use tokio::process::Command;

fn needs_rerun(path: PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
    let mut p = path.clone();
    p.push("main.log");
    let data = read_to_string(&p)?;
    let re = Regex::new(r".*Rerun to get.*").unwrap();
    let len = re.find_iter(data.as_str()).collect::<Vec<_>>().len();
    Ok(len > 0)
}

pub async fn run(
    _world: M::World,
    song: M::Song,
    deps: Vec<PathBuf>,
) -> Result<M::BuildType, Box<dyn std::error::Error>> {
    let mut pdffile = song.builddir.clone();
    pdffile.push("main.pdf");

    if pdffile.try_exists()? {
        if !digest_has_changed(pdffile.clone(), deps.clone())? {
            return Ok(M::BuildType::NotTouched(pdffile));
        }
    }

    let mut count = 1;
    loop {
        let logstream = |s: &str| {
            let mut p: PathBuf = PathBuf::from(&song.builddir);
            p.push(format!("lualatex-{}.log", s));
            File::create(p)
        };

        log::info!("{}:{} running lualatex", file!(), line!());
        let child = Command::new("lualatex")
            .arg("--interaction=nonstopmode")
            .arg("main.tex")
            //.env("HOME", song.builddir.to_str().unwrap())
            .kill_on_drop(true)
            // .stdout(Stdio::piped())
            .stdout(logstream("stdout")?)
            .stderr(logstream("stderr")?)
            .current_dir(&song.builddir)
            .spawn()?;

        log::debug!(
            "{}:{} run lualatex for {} {}",
            file!(),
            line!(),
            &song.author,
            &song.title
        );
        let output = &child.wait_with_output().await?;
        if !(output.status.success()) {
            log::error!("lualatex failed for {} {}", &song.author, &song.title);
            return Err("lualatex failed".into());
        }
        if !needs_rerun(song.builddir.clone())? {
            break;
        }
        count += 1;
        if count > 3 {
            log::error!(
                "lualatex failed for {} {} after 3 attempts",
                &song.author,
                &song.title
            );
            return Err("lualatex failed after 3 attempts".into());
        }
    }
    log::info!("done, now running manage_digest");
    // write digest
    write_digest(pdffile.clone(), deps)?;
    Ok(M::BuildType::Rebuilt(pdffile))
}

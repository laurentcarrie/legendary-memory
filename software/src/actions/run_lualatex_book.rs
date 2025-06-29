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
    book: M::Book,
    deps: Vec<PathBuf>,
) -> Result<M::BuildType, Box<dyn std::error::Error>> {
    for d in &deps {
        log::info!("for book {} ; dep {:?}", book.title, d);
    }

    let mut target_file = PathBuf::from(&book.builddir);
    target_file.push("main.pdf");

    if target_file.try_exists()? {
        if !digest_has_changed(target_file.clone(), deps.clone())? {
            return Ok(M::BuildType::NotTouched(target_file));
        }
    }

    let mut count = 1;
    loop {
        let mut p: PathBuf = PathBuf::from(&book.builddir);
        p.push("lualatex.log");
        let fout = File::create(p)?;
        log::info!("{}:{} running lualatex", file!(), line!());

        let child = Command::new("lualatex")
            .arg("--interaction=nonstopmode")
            .arg("main.tex")
            //.env("HOME", song.builddir.to_str().unwrap())
            .kill_on_drop(true)
            // .stdout(Stdio::piped())
            .stdout(fout)
            .current_dir(&book.builddir)
            .spawn()?;

        log::debug!("{}:{} run lualatex for {}", file!(), line!(), &book.title);
        let output = &child.wait_with_output().await?;
        if !(output.status.success()) {
            log::error!("lualatex failed for {}", &book.title);
            return Err("lualatex failed".into());
        }
        if !needs_rerun(book.builddir.clone())? {
            break;
        }
        count += 1;
        if count > 3 {
            log::error!("lualatex failed for {} after 3 attempts", &book.title);
            return Err("lualatex failed after 3 attempts".into());
        }
    }
    write_digest(target_file.clone(), deps)?;

    Ok(M::BuildType::Rebuilt(target_file))
}

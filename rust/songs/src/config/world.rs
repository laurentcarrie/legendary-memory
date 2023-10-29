use crate::config::config::decode_song;
use crate::config::get_config_files::get_song_yml_paths;
use crate::config::model::{Song, World};
use std::path::PathBuf;
pub fn make(srcdir: &PathBuf, builddir: &PathBuf) -> World {
    let project_yaml_paths: Vec<PathBuf> = get_song_yml_paths(&srcdir);
    let songs: Vec<Song> = project_yaml_paths
        .iter()
        .map(|p| decode_song(&builddir, &p).ok())
        .filter(|p| p.is_some())
        .map(|p| p.unwrap())
        .collect();
    let world = World {
        builddir: builddir.to_path_buf(),
        srcdir: srcdir.to_path_buf(),
        project_yaml_paths: project_yaml_paths,
        songs: songs,
    };
    // dbg!(&world);
    world
}

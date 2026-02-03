use std::path::PathBuf;

pub fn make_path(root: PathBuf, elems: Vec<&str>) -> PathBuf {
    let mut p = root.clone();
    for x in elems.iter() {
        p.push(x);
    }
    p
}

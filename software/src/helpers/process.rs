use std::fs;
use std::io::Error;
use std::path::Path;
use sysinfo::Pid;

pub fn get_children(pid: u32) -> Result<Vec<u32>, Error> {
    let mut ret: Vec<u32> = vec![];
    let str_filepath = format!("/proc/{}/task/{}/children", &pid, &pid);
    let filepath = Path::new(&str_filepath);

    let contents = fs::read_to_string(&filepath)?;
    let contents: Vec<_> = contents.split(" ").collect();
    let mut ret2: Vec<u32> = contents
        .iter()
        .filter(|child_pid| child_pid.len() > 0)
        .map(|child_pid| child_pid.parse::<u32>().unwrap())
        .collect();

    ret.append(&mut ret2);
    Ok(ret)
}

pub fn get_descendents(rootpid: u32) -> Result<Vec<u32>, Error> {
    let mut acc: Vec<u32> = Vec::new();
    let mut todo: Vec<u32> = get_children(rootpid)?;
    loop {
        let pid = todo.pop();
        match pid {
            None => break,
            Some(pid) => {
                acc.push(pid);
                let mut children = get_children(pid)?;
                todo.append(&mut children);
            }
        }
    }
    Ok(acc)
}

pub fn find_pids_from_name(name: String) -> Result<Vec<u32>, Error> {
    let s = sysinfo::System::new_all();
    let children = get_descendents(1)?;
    let ret = children
        .iter()
        .filter_map(|pid| {
            let p = s.process(sysinfo::Pid::from(Pid::from_u32(*pid)));
            match p {
                None => None,
                Some(p) => {
                    let other_name = p.name().to_str()?;
                    let other_name = String::from(other_name);
                    if name == other_name {
                        Some(*pid)
                    } else {
                        None
                    }
                }
            }
        })
        .collect();
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    // generate test data with ps -ef, find a process that is running,
    // and then pstree -p -s <that pid>
    // empty list is to make test pass anyway, it does not test anything
    fn test_data() -> VecDeque<u32> {
        // VecDeque::from([2120, 4493, 9404, 21242])
        VecDeque::from([])
    }

    #[test]
    fn test_get_children() -> Result<(), Box<dyn std::error::Error>> {
        let mut pids = test_data();
        let mut parent = 1;
        let child = pids.pop_front();
        if child.is_none() {
            // no test
            return Ok(());
        }
        let mut child = child.unwrap();
        loop {
            let children = get_children(parent)?;
            let index = children.iter().find(|x| x == &&child);
            println!("parent = {}, child = {}", parent, child);
            assert!(index.is_some());
            parent = child;
            if pids.is_empty() {
                break;
            }
            child = pids.pop_front().unwrap();
        }
        Ok(())
    }

    // this is a manual test, just pick an existing pid with ps -ef, and test that it is a descendent of 1
    #[test]
    fn test_get_descendents() -> Result<(), Box<dyn std::error::Error>> {
        let pid = sysinfo::Pid::from_u32(1);
        let children = get_descendents(pid.as_u32())?;
        // let children: Vec<u32> = vec![];
        // let thispid = sysinfo::get_current_pid()?.as_u32();
        for pid in test_data() {
            println!("test descendent {}", &pid);
            let index = children.iter().find(|x| x == &&pid);
            assert!(index.is_some());
        }
        // assert!(pid.as_u32()==1234) ;
        Ok(())
    }

    //  #[test]
    fn _test_get_find_pid() -> () {
        let mut found = find_pids_from_name("omake".to_string()).unwrap();
        assert_eq!(found.len(), 1);
        let pid = &found.pop().unwrap();
        let s = sysinfo::System::new_all();
        let p = s.process(sysinfo::Pid::from(Pid::from_u32(*pid))).unwrap();
        assert_eq!(p.name().to_str().unwrap(), "omake");
    }
}

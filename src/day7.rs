use std::{
    cell::RefCell,
    collections::HashMap,
    rc::{Rc, Weak},
};

use lazy_static::lazy_static;
use regex::Regex;

type DirLink = Rc<RefCell<Dir>>;
type DirWeak = Weak<RefCell<Dir>>;

// I know this solution is silly, but my main goal was to struggle with
// Rust's borrow machinery, not solving the bloody problem.

#[derive(Debug)]
struct Dir {
    name: String,
    parent: Option<DirWeak>,
    files: HashMap<String, usize>,
    dirs: HashMap<String, DirLink>,
}

impl Dir {
    pub fn new(name: &str, parent: Option<DirWeak>) -> Self {
        Dir {
            name: name.into(),
            parent,
            files: HashMap::new(),
            dirs: HashMap::new(),
        }
    }

    pub fn new_link(name: &str, parent: Option<DirWeak>) -> DirLink {
        Rc::new(RefCell::new(Dir::new(name, parent)))
    }
}

fn parse(input: &[String]) -> DirLink {
    lazy_static! {
        static ref RE_CD: Regex = Regex::new(r"^\$ cd (\S+)$").unwrap();
        static ref RE_FILE: Regex = Regex::new(r"^([0-9]+) (\S+)$").unwrap();
    }

    let root = Dir::new_link("/", None);
    let mut current = Rc::clone(&root);

    for line in input {
        if let Some(caps) = RE_FILE.captures(line) {
            current
                .borrow_mut()
                .files
                .insert(caps[2].into(), caps[1].parse().unwrap());
        } else if let Some(caps) = RE_CD.captures(line) {
            let dirname = &caps[1];
            if dirname == "/" {
                current = Rc::clone(&root);
            } else if dirname == ".." {
                let x = current.borrow().parent.as_ref().unwrap().upgrade().unwrap();
                current = x;
            } else {
                let new_dir = Dir::new_link(dirname, Some(Rc::downgrade(&current)));
                current
                    .borrow_mut()
                    .dirs
                    .insert(dirname.into(), Rc::clone(&new_dir));
                current = Rc::clone(&new_dir);
            }
        }
    }

    root
}

fn total_size(dir: DirLink) -> usize {
    dir.borrow().files.values().sum::<usize>()
        + dir
            .borrow()
            .dirs
            .values()
            .map(|x| total_size(Rc::clone(x)))
            .sum::<usize>()
}

fn all_dirs(dir: DirLink) -> Vec<DirLink> {
    let mut result = vec![Rc::clone(&dir)];
    for d in dir.borrow().dirs.values() {
        result.append(&mut all_dirs(Rc::clone(d)));
    }
    result
}

pub fn solution1(data: &[String]) -> usize {
    let root = parse(data);
    // for d in all_dirs(dir)
    println!("{:?}", root);
    println!("{:?}", total_size(root));
    todo!()
}

pub fn solution2(data: &[String]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::{common::str2lines, day7};

    fn data() -> Vec<String> {
        str2lines(
            r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#,
        )
    }

    #[test]
    fn test_solution1() {
        assert_eq!(95437, day7::solution1(&data()))
    }
}
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
// Not sure I'm using Rc<> here as God intended.

#[derive(Debug)]
struct Dir {
    #[allow(dead_code)]
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

fn total_size(dir: &DirLink) -> usize {
    dir.borrow().files.values().sum::<usize>()
        + dir
            .borrow()
            .dirs
            .values()
            .map(|x| total_size(&Rc::clone(x)))
            .sum::<usize>()
}

fn all_dirs(dir: &DirLink) -> Vec<DirLink> {
    let mut result = vec![Rc::clone(dir)];
    for d in dir.borrow().dirs.values() {
        result.append(&mut all_dirs(&Rc::clone(d)));
    }
    result
}

pub fn solution1(data: &[String]) -> usize {
    let root = parse(data);
    all_dirs(&root)
        .iter()
        .map(|x| total_size(&Rc::clone(x)))
        .filter(|x| *x <= 100_000)
        .sum()
}

pub fn solution2(data: &[String]) -> usize {
    let root = parse(data);
    let dir_sizes: Vec<_> = all_dirs(&root)
        .iter()
        .map(|x| total_size(&Rc::clone(x)))
        .collect();
    let target_space = 70_000_000 - 30_000_000;
    let amount_to_remove = dir_sizes.iter().max().unwrap() - target_space;
    *dir_sizes
        .iter()
        .filter(|x| **x >= amount_to_remove)
        .min()
        .unwrap()
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
        assert_eq!(95437, day7::solution1(&data()));
    }

    #[test]
    fn test_solution2() {
        assert_eq!(24_933_642, day7::solution2(&data()));
    }
}

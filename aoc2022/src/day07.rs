use std::{
    cell::RefCell,
    collections::BTreeSet,
    rc::{Rc, Weak},
};

use crate::*;

pub const SOLUTION: Solution = Solution { day: 7, solve };

// 510 us
fn solve(input: &str) -> AnswerSet {
    let filesystem = build_filesystem(input);

    let mut ordered_sizes = BTreeSet::new();
    let (_, p1) = sum_sizes(&filesystem, 100000, &mut ordered_sizes);

    let min_delete = ordered_sizes.iter().next_back().unwrap() - 40000000;
    let &p2 = ordered_sizes.range(min_delete..).next().unwrap();

    AnswerSet {
        p1: Answer::U64(p1),
        p2: Answer::U64(p2),
    }
}

fn sum_sizes(folder: &Folder, limit: u64, sizes: &mut BTreeSet<u64>) -> (u64, u64) {
    let size = folder.files.borrow().iter().map(|file| file.size).sum();
    let folders = folder.folders.borrow();

    if folders.is_empty() {
        sizes.insert(size);
        (size, if size <= limit { size } else { 0 })
    } else {
        let (sub_total, sub_under) = folders
            .iter()
            .map(|folder| sum_sizes(folder, limit, sizes))
            .reduce(|(acc_total, acc_under), (total, under)| (acc_total + total, acc_under + under))
            .unwrap();

        let size = sub_total + size;
        sizes.insert(size);
        (
            size,
            if size <= limit {
                size + sub_under
            } else {
                sub_under
            },
        )
    }
}

fn build_filesystem(input: &str) -> Rc<Folder> {
    let mut stack = Vec::new();

    let root = Rc::new(Folder::new("/".to_string(), None));
    let mut curr_folder: Rc<Folder> = root.clone();

    for line in input.lines().skip(1) {
        let mut bytes = line.bytes();

        let front = bytes.next().unwrap();
        match front {
            b'$' => {
                let command = bytes.nth(1).unwrap();
                match command {
                    b'c' => {
                        // $ cd
                        // Consume d and space
                        bytes.next();
                        bytes.next();

                        // Get folder name
                        // This also assumes that every ls command will be immediately preceded by a cd command which explicitly states the folder name
                        let curr_folder_name: String = bytes.map(|b| b as char).collect();

                        if curr_folder_name == ".." {
                            let tmp = curr_folder
                                .parent
                                .borrow()
                                .clone()
                                .unwrap()
                                .upgrade()
                                .unwrap();
                            curr_folder = tmp;
                        } else {
                            let tmp = curr_folder
                                .folders
                                .borrow()
                                .iter()
                                .find(|folder| folder.name == curr_folder_name)
                                .unwrap()
                                .clone();
                            curr_folder = tmp;
                            stack.push(curr_folder_name);
                        }
                    }

                    b'l' => {
                        // $ ls
                        // Do nothing
                    }

                    _ => {}
                }
            }

            b'd' => {
                // dir
                // Consume rest of command and space
                bytes.nth(2);

                let folder_name: String = bytes.map(|b| b as char).collect();
                curr_folder.folders.borrow_mut().push(Rc::new(Folder::new(
                    folder_name,
                    Some(Rc::downgrade(&curr_folder)),
                )));
            }

            _ => {
                let size = parse_u64_from_ascii_iter(&mut bytes, front);

                let file_name: String = bytes.map(|b| b as char).collect();

                curr_folder.files.borrow_mut().push(File {
                    name: file_name,
                    size,
                });
            }
        }
    }

    root
}

#[derive(Debug)]
struct Folder {
    name: String,
    files: RefCell<Vec<File>>,
    folders: RefCell<Vec<Rc<Folder>>>,
    parent: RefCell<Option<Weak<Folder>>>,
}

impl Folder {
    fn new(name: String, parent: Option<Weak<Folder>>) -> Folder {
        Folder {
            name,
            files: RefCell::new(vec![]),
            folders: RefCell::new(vec![]),
            parent: RefCell::new(parent),
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: u64,
}

fn parse_u64_from_ascii_iter(ascii_iter: &mut impl Iterator<Item = u8>, init: u8) -> u64 {
    let mut num = (init - b'0') as u64;

    while let Some(byte) = ascii_iter.next() {
        if !byte.is_ascii_digit() {
            break;
        }

        num *= 10;
        num += (byte - b'0') as u64;
    }

    num
}

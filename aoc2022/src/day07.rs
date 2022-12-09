use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    rc::{Rc, Weak},
};

use crate::*;

pub const SOLUTION: Solution = Solution {
    day: 7,
    solve: solve_dfs,
};

/* ======== OPTIMIZED 1-PASS DFS ======== */
/* Key assumptions:
    - The input data is laid out such that all folders will be visited according to an exhaustive DFS algorithm
    - That is: all folders are visited top-down only once, and we will always move into a subfolder if it exists
*/
// ~55 us
fn solve_dfs(input: &str) -> AnswerSet {
    let filesystem = dfs(&mut input.lines());
    let p1 = filesystem.small_size;

    let del_target = filesystem.size - 40_000_000;
    let p2 = find_min_target(del_target, &filesystem).unwrap();

    AnswerSet {
        p1: Answer::U64(p1),
        p2: Answer::U64(p2),
    }
}

// DFS to find p2
fn find_min_target(target: u64, folder: &FolderDFS) -> Option<u64> {
    let mut min_target = None;

    for folder in folder.folders.iter() {
        if folder.size >= target {
            if let Some(min) = find_min_target(target, folder) {
                if min_target.is_none() || min_target.unwrap() > min {
                    min_target = Some(min);
                }
            }
        }
    }

    if min_target.is_none() && folder.size >= target {
        Some(folder.size)
    } else {
        min_target
    }
}

// DFS to build filesystem, accumulating part 1's sum along the way
fn dfs<'a>(lines: &mut impl Iterator<Item = &'a str>) -> FolderDFS {
    let mut size = 0;
    let mut small_sum = 0;
    let mut subfolders = Vec::new();

    while let Some(line) = lines.next() {
        let mut bytes = line.bytes();

        let front = bytes.next().unwrap();
        match front {
            b'$' => {
                let command = bytes.nth(1).unwrap();
                match command {
                    b'c' => {
                        // $ cd
                        if bytes.nth(2) == Some(b'.')
                            && bytes.next() == Some(b'.')
                            && bytes.next() == None
                        {
                            // Move up
                            return FolderDFS {
                                size,
                                small_size: if size <= 100_000 {
                                    size + small_sum
                                } else {
                                    small_sum
                                },
                                folders: subfolders,
                            };
                        } else {
                            // Enter subfolder
                            let subfolder = dfs(lines);
                            size += subfolder.size;
                            small_sum += subfolder.small_size;
                            subfolders.push(subfolder);
                        }
                    }

                    _ => {
                        // $ ls
                        // Do nothing
                    }
                }
            }

            b'd' => {
                // dir
                // Do nothing
            }

            _ => {
                let file_size = parse_u64_from_ascii_iter(&mut bytes, front);
                size += file_size;
            }
        }
    }

    FolderDFS {
        size,
        small_size: if size <= 100_000 {
            size + small_sum
        } else {
            small_sum
        },
        folders: subfolders,
    }
}

#[derive(Debug)]
struct FolderDFS {
    size: u64,
    small_size: u64,
    folders: Vec<FolderDFS>,
}

/* ======== OPTIMIZED HASHMAP ======== */
// 225 us
fn solve_map(input: &str) -> AnswerSet {
    let filesystem = build_filesystem_map(input);

    let mut p1 = 0;
    let mut p2 = u64::MAX;

    let total_size = filesystem.get("/").unwrap();

    for size in filesystem.values() {
        if size <= &100000 {
            p1 += size;
        }

        if size >= &(total_size - 40000000) && size < &p2 {
            p2 = *size;
        }
    }

    AnswerSet {
        p1: Answer::U64(p1),
        p2: Answer::U64(p2),
    }
}

fn build_filesystem_map(input: &str) -> HashMap<String, u64> {
    let mut stack = Vec::new();
    let mut filepath = "/".to_string();
    stack.push(filepath.to_string());

    let mut filesystem = HashMap::new();
    filesystem.insert(filepath.to_string(), 0);

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
                        bytes.nth(1);

                        // Get folder name
                        // This also assumes that every ls command will be immediately preceded by a cd command which explicitly states the folder name
                        let curr_folder_name: String = bytes.map(|b| b as char).collect();

                        if curr_folder_name == ".." {
                            let last_folder = stack.pop().unwrap();
                            let &last_size = filesystem.get(&filepath).unwrap_or(&0);
                            filepath.truncate(filepath.len() - last_folder.len() - 1);
                            *filesystem.get_mut(&filepath).unwrap() += last_size;
                        } else {
                            filepath.push('/');
                            filepath.push_str(&curr_folder_name);
                            stack.push(curr_folder_name);

                            // This assumes that no folder is visited from the top-down more than once
                            filesystem.insert(filepath.clone(), 0);
                        }
                    }

                    _ => {
                        // $ ls
                        // Do nothing
                    }
                }
            }

            b'd' => {
                // dir
            }

            _ => {
                let size = parse_u64_from_ascii_iter(&mut bytes, front);

                *filesystem.get_mut(&filepath).unwrap() += size;
            }
        }
    }

    // Unwind stack
    while stack.len() > 1 {
        let last_folder = stack.pop().unwrap();
        let &last_size = filesystem.get(&filepath).unwrap_or(&0);
        filepath.truncate(filepath.len() - last_folder.len() - 1);
        *filesystem.get_mut(&filepath).unwrap() += last_size;
    }

    filesystem
}

/* ======== LINKED TREE SOLUTION ======== */
// ~500 us
fn solve(input: &str) -> AnswerSet {
    let filesystem = build_filesystem(input);

    let mut ordered_sizes = BTreeMap::new();
    let root_size = sum_sizes(&filesystem, &mut ordered_sizes);

    let p1 = ordered_sizes
        .range(..=100000)
        .map(|(size, &freq)| size * freq as u64)
        .sum();

    let min_delete = root_size - 40000000;
    let &p2 = ordered_sizes.range(min_delete..).next().unwrap().0;

    AnswerSet {
        p1: Answer::U64(p1),
        p2: Answer::U64(p2),
    }
}

fn sum_sizes(folder: &Folder, sizes: &mut BTreeMap<u64, usize>) -> u64 {
    let size = folder.files.borrow().iter().map(|file| file.size).sum();
    let folders = folder.folders.borrow();

    if folders.is_empty() {
        // Need to do this to keep track of duplicate sizes
        *sizes.entry(size).or_insert(0) += 1;
        size
    } else {
        let sub_total: u64 = folders.iter().map(|folder| sum_sizes(folder, sizes)).sum();

        let size = sub_total + size;
        *sizes.entry(size).or_insert(0) += 1;
        size
    }
}

fn build_filesystem(input: &str) -> Rc<Folder> {
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
                        }
                    }

                    _ => {
                        // $ ls
                        // Do nothing
                    }
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

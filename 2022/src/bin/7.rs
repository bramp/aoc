
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ffi::OsStr;

fn puzzle() -> i32 {
    return env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
        .expect("Unknown puzzle number")
        .parse::<i32>()
        .unwrap();
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct FileEntry {
    name: String,
    size: u64,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
struct DirEntry {
    name: String,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
enum Entry {
    Dir(DirEntry),
    File(FileEntry)
}

fn dirtree(filename: &str) -> BTreeSet<Entry> {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let mut cwd = "".to_string();

    let mut entries = BTreeSet::new();
    entries.insert(Entry::Dir(DirEntry{
        name: cwd.clone(),
    }));

    for line in lines {
        if line.starts_with("$") {
            // Command
            if line.starts_with("$ cd") {
                let dir = line[4..].trim();
                if dir == "/" {
                    cwd = "".to_string()
                } else if dir == ".." {
                    //todo!();
                    if let Some(i) = cwd.rfind('/') {
                        cwd = cwd[0..i].to_string();
                    }
                } else {
                    cwd += "/";
                    cwd += dir;
                }
            }
        } else {
            let (size, name) = line.split_once(" ").expect("couldn't parse line");
            let fullname = cwd.clone() + "/" + name;

            if size == "dir" {
                entries.insert(Entry::Dir(DirEntry{
                    name: fullname,
                }));
            } else {
                let size = size.parse().unwrap();
                entries.insert(Entry::File(FileEntry{
                    name: fullname,
                    size: size,
                }));
            }

        }
    }

    entries
}


fn solution1(filename: &str) -> u64 {
    let entries = dirtree(filename);

    // Work out size of each directory
    let mut dirs = HashMap::<String, u64>::new();

    for entry in entries {
        match entry {
            Entry::File(e) => {
                for i in e.name.match_indices("/") {
                    let parent = e.name[0..(i.0 + 1)].to_string();

                    *dirs.entry(parent)
                        .or_insert(0) += e.size;
                }
            }
            Entry::Dir(_) => continue,
        }
    }

    let mut count = 0;
    for d in dirs {
        if d.1 <= 100000 {
            count += d.1;
        }
    }

    count
}


fn solution2(filename: &str) -> u64 {
    let entries = dirtree(filename);

    // Work out size of each directory
    let mut dirs = HashMap::<String, u64>::new();

    for entry in entries {
        match entry {
            Entry::File(e) => {
                for i in e.name.match_indices("/") {
                    let parent = e.name[0..(i.0 + 1)].to_string();

                    *dirs.entry(parent)
                        .or_insert(0) += e.size;
                }
            }
            Entry::Dir(_) => continue,
        }
    }

    let size = 70000000;
    let used = dirs["/"];
    let available = size - used;

    let target = 30000000;
    let need_to_free = target - available;

    let mut dirs: Vec<_> = dirs.iter().collect();
    dirs.sort_by(|a, b| a.1.cmp(b.1));

    // Search though (from smallest to largest) looking for the
    // first entry large enough.
    for d in dirs {
        if *d.1 > need_to_free {
            return *d.1;
        }
    }

    panic!();
}

fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));

    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 95437);

    let answer2 = solution1(&format!("{}b.txt", num));

    println!("Answer2: {:?}", answer2);
    assert_eq!(answer2, 1084134);

    let answer3 = solution2(&format!("{}.txt", num));

    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 24933642);

    let answer4 = solution2(&format!("{}b.txt", num));
    println!("Answer4: {:?}", answer4);
    assert_eq!(answer4, 6183184);
}

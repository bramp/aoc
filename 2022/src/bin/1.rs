
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Reverse;
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

fn solution1(filename: &str) -> i32 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file).lines();

    let mut elf = Vec::new();
    elf.push(0);

    for line in lines {
        let line = line.expect("failed to read line");
        let line = line.trim();

        if line.is_empty() {
            // Next elf.
            elf.push(0);
            continue;
        }

        *(elf.last_mut().unwrap()) += line.parse::<i32>().unwrap();
    }

    return *elf.iter().max().unwrap()
}


fn solution2(filename: &str) -> i32 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file).lines();

    let mut elf = Vec::new();
    elf.push(0);

    for line in lines {
        let line = line.expect("failed to read line");
        let line = line.trim();

        if line.is_empty() {
            // Next elf.
            elf.push(0);
            continue;
        }

        *(elf.last_mut().unwrap()) += line.parse::<i32>().unwrap();
    }

    elf.sort_by_key(|w| Reverse(*w));

    return elf.iter().take(3).sum();
}

fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));
    let answer2 = solution1(&format!("{}b.txt", num));

    assert_eq!(answer1, 24000);
    assert_eq!(answer2, 66616);

    println!("Answer1: {:?}", answer1);
    println!("Answer2: {:?}", answer2);

    let answer3 = solution2(&format!("{}.txt", num));
    let answer4 = solution2(&format!("{}b.txt", num));

    assert_eq!(answer3, 45000);
    assert_eq!(answer4, 199172);

    println!("Answer3: {:?}", answer3);
    println!("Answer4: {:?}", answer4);
}

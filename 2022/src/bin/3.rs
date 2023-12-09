
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ffi::OsStr;
use std::iter::FromIterator;

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

fn solution1(filename: &str) -> u32 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file).lines();

    let mut sum = 0;

    for line in lines {
        let line = line.expect("failed to read line");
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let left: HashSet<char> = HashSet::from_iter(line.chars().take(line.len() / 2));
        let right: HashSet<char> = HashSet::from_iter(line.chars().skip(line.len() / 2));

        let common = left.intersection(&right).next().unwrap();
        sum += match common {
            'a'..='z' => (*common as u32 - 'a' as u32) + 1,
            'A'..='Z' => (*common as u32 - 'A' as u32) + 27,
            _ => panic!(),
        };
    }

    return sum
}


fn solution2(filename: &str) -> u32 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file).lines().map(|line| line.expect("failed to read line"));

    let mut sum = 0;

    for lines in lines.collect::<Vec<_>>().chunks(3) {

        let ls = lines.iter().map(|line| -> HashSet<char> { HashSet::from_iter(line.trim().chars()) });

        let common = ls.reduce(|acc, x| HashSet::from_iter(acc.intersection(&x).cloned()) ).unwrap();

        assert!(common.len() == 1);

        let common = common.iter().next().unwrap();
        sum += match common {
            'a'..='z' => (*common as u32 - 'a' as u32) + 1,
            'A'..='Z' => (*common as u32 - 'A' as u32) + 27,
            _ => panic!(),
        };
    }
    return sum
}

fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));
    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 157);
    
    let answer2 = solution1(&format!("{}b.txt", num));
    println!("Answer2: {:?}", answer2);
    assert_eq!(answer2, 7568);

    let answer3 = solution2(&format!("{}.txt", num));
    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 70);

    let answer4 = solution2(&format!("{}b.txt", num));
    println!("Answer4: {:?}", answer4);
    assert_eq!(answer4, 2780);
}

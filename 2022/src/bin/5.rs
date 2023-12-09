
use std::collections::BTreeMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ffi::OsStr;
use regex::Regex;
use std::collections::VecDeque;


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

fn solution1(filename: &str) -> String {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"));

    // move 1 from 2 to 1
    let re1 = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();

    //     [D]    
    let re2 = Regex::new(r"\[([A-Z]+)\]").unwrap();

    let mut stacks: BTreeMap<usize, VecDeque<String>> = BTreeMap::new();

    for line in lines {
        if let Some((_, c)) = re1.captures(&line).map(|c| c.extract()) {
            let [mut count, from, to] = c.map(|x| x.parse::<usize>().unwrap());

            // Move
            while count > 0 {
                let m = stacks
                    .get_mut(&from)
                    .expect("from stack is missing")
                    .pop_front()
                    .expect("stack is empty");

                stacks.entry(to)
                    .or_insert_with(VecDeque::new)
                    .push_front(m);

                count -= 1;
            }

        } else {
            for m in re2.find_iter(&line) {
                let to = m.start() / 4 + 1;
                let c = m.as_str().trim_matches(|c| c == '[' || c == ']').to_owned();

                stacks.entry(to)
                    .or_insert_with(VecDeque::new)
                    .push_back(c);
            }
        }
    }

    return stacks
        .iter()
        .fold(String::new(), |acc, e| acc + e.1.front().unwrap());
}


fn solution2(filename: &str) -> String {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"));

    // move 1 from 2 to 1
    let re1 = Regex::new(r"^move ([0-9]+) from ([0-9]+) to ([0-9]+)$").unwrap();

    //     [D]    
    let re2 = Regex::new(r"\[([A-Z]+)\]").unwrap();

    let mut stacks: BTreeMap<usize, VecDeque<String>> = BTreeMap::new();

    for line in lines {
        if let Some((_, c)) = re1.captures(&line).map(|c| c.extract()) {
            let [count, from, to] = c.map(|x| x.parse::<usize>().unwrap());

            // Move
            let drained: Vec<_> = stacks
                .get_mut(&from)
                .expect("from stack is missing")
                .drain(0..count)
                .collect();

            let to = stacks.entry(to)
                .or_insert_with(VecDeque::new);

            // TODO If we swap front/back thoughout, we can do to.append(drained).
            for c in drained.iter().rev() {
                to.push_front(c.to_owned());
            }

        } else {
            for m in re2.find_iter(&line) {
                let to = m.start() / 4 + 1;
                let c = m.as_str().trim_matches(|c| c == '[' || c == ']').to_owned();

                stacks.entry(to)
                    .or_insert_with(VecDeque::new)
                    .push_back(c);
            }
        }
    }

    return stacks
        .iter()
        .fold(String::new(), |acc, e| acc + e.1.front().unwrap());
}

fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));

    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, "CMZ");

    let answer2 = solution1(&format!("{}b.txt", num));

    println!("Answer2: {:?}", answer2);
    assert_eq!(answer2, "QGTHFZBHV");

    let answer3 = solution2(&format!("{}.txt", num));

    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, "MCD");

    let answer4 = solution2(&format!("{}b.txt", num));
    println!("Answer4: {:?}", answer4);
    assert_eq!(answer4, "MGDMPSZTM");
}


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

fn find_start(line: &str) -> i32 {
    let mut count = 4;

    for w in line.as_bytes().windows(4) {
        if w[0] != w[1] && w[0] != w[1] && w[0] != w[2] && w[0] != w[3] &&
            w[1] != w[2] && w[1] != w[3] &&
            w[2] != w[3] {
                return count;
            }
        count += 1;
    }

    panic!();
}

fn solution1(filename: &str) -> i32 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    for line in lines {
        return find_start(&line);
    }

    panic!();
}

fn find_start2(line: &str) -> usize {
    const LEN: usize = 14;
    let mut count = LEN;

    for w in line.as_bytes().windows(LEN) {
        let mut w = Vec::from(w);
        w.sort();
        w.dedup();
        if w.len() == LEN {
            return count;
        }
        count += 1;
    }

    panic!();
}

fn solution2(filename: &str) -> usize {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    for line in lines {
        return find_start2(&line);
    }

    panic!();
}

fn main() {
    let num = puzzle();

    let answer1 = find_start("bvwbjplbgvbhsrlpgdmjqwftvncz");
    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 5);

    let answer1 = find_start("nppdvjthqldpwncqszvftbrmjlhg");
    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 6);

    let answer1 = find_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 10);

    let answer1 = find_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 11);

    let answer2 = solution1(&format!("{}b.txt", num));

    println!("Answer2: {:?}", answer2);
    assert_eq!(answer2, 1794);



    let answer3 = find_start2("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 19);

    let answer3 = find_start2("bvwbjplbgvbhsrlpgdmjqwftvncz");
    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 23);

    let answer3 = find_start2("nppdvjthqldpwncqszvftbrmjlhg");
    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 23);

    let answer3 = find_start2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 29);

    let answer3 = find_start2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");
    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 26);

    let answer4 = solution2(&format!("{}b.txt", num));

    println!("Answer4: {:?}", answer4);
    assert_eq!(answer4, 2851);

}

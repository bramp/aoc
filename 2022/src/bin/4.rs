
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

fn to_range(range: &str) -> Result<std::ops::RangeInclusive<i32>, std::num::ParseIntError> {
    if let Some((start, end)) = range.split_once('-') {
        return Ok(start.parse()?..=end.parse()?)
    }

    // Hack to get a ParseIntError error
    return "".parse::<i32>().map(|x| x..=x );
}

fn solution1(filename: &str) -> i32 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let mut count = 0;

    for line in lines {
        if let Some((a, b)) = line.split_once(',') {
            let a = to_range(a).unwrap();
            let b = to_range(b).unwrap();

            if (a.contains(b.start()) && a.contains(b.end())) || (b.contains(a.start()) && b.contains(a.end())) {
                count += 1;
            }


        }
    }

    return count
}


fn solution2(filename: &str) -> i32 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let mut count = 0;

    for line in lines {
        if let Some((a, b)) = line.split_once(',') {
            let a = to_range(a).unwrap();
            let b = to_range(b).unwrap();

            if (a.contains(b.start()) || a.contains(b.end())) || (b.contains(a.start()) || b.contains(a.end())) {
                count += 1;
            }


        }
    }

    return count
}

fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));

    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 2);

    let answer2 = solution1(&format!("{}b.txt", num));

    println!("Answer2: {:?}", answer2);
    assert_eq!(answer2, 524);

    let answer3 = solution2(&format!("{}.txt", num));

    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 4);

    let answer4 = solution2(&format!("{}b.txt", num));
    println!("Answer4: {:?}", answer4);
    assert_eq!(answer4, 798);
}

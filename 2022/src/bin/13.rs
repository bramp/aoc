
use core::cmp::Ordering;
use core::cmp::max;
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

#[derive(Debug)]
#[derive(PartialEq)]
enum Entry {
    Value(i64),
    List(Vec<Entry>),
}

fn parse_entry(mut input: &str) -> (Entry, &str) {
    if input.starts_with('[') {
        let mut entries = Vec::new();
        input = &input[1..];

        while !input.is_empty() {

            if input.starts_with(']') {
                return (Entry::List(entries), &input[1..]);

            } else if input.starts_with(',') {
                // skip
                input = &input[1..];

            } else {
                let (entry, remaining) = parse_entry(input);
                entries.push(entry);
                input = remaining;
            }
        }
    }

    let next = input.find(|c: char| !c.is_digit(10));
    let next = match next {
        Some(i) => i,
        None => input.len(),
    };
    return (
        Entry::Value(input[0..next].parse::<i64>().unwrap()),
        &input[next..]
    );
}


fn parse_input(filename: &str) -> Vec<Entry> {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let mut entries = Vec::new();

    for line in lines {
        let (entry, remaining) = parse_entry(&line);
        assert!(remaining.is_empty());

        entries.push(entry);
    }

    return entries;

}

fn compare(left: &Entry, right: &Entry) -> Ordering {
    return match left {
        Entry::Value(l) => match right {
            Entry::Value(r) => {
                if l < r {
                    Ordering::Less
                } else if l > r {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            },
            Entry::List(_) => {
                let left = &Entry::List(vec![Entry::Value(*l)]);
                compare(left, right)
            },
        },
        Entry::List(l) => match right {
            Entry::Value(r) => {
                let right = &Entry::List(vec![Entry::Value(*r)]);
                compare(left, right)
            },
            Entry::List(r) => {
                for i in 0..max(l.len(), r.len()) {
                    if i >= l.len() {
                        return Ordering::Less;
                    }

                    if i >= r.len() {
                        return Ordering::Greater;
                    }

                    let c = compare(&l[i], &r[i]);
                    if c == Ordering::Equal {
                        continue;
                    }

                    return c;
                }

                // identical lists
                Ordering::Equal
            },
        },
    };
}

fn solution1(filename: &str) -> i64 {
    let entries = parse_input(filename);

    let mut i = 1;
    let mut sum = 0;
    for pair in entries.chunks(2) {
        if compare(&pair[0], &pair[1]) != Ordering::Greater {
            sum += i;
        }
        i += 1;
    }

    return sum;
}


fn solution2(filename: &str) -> usize {
    let mut entries = parse_input(filename);

    let two = parse_entry("[[2]]").0;
    let six = parse_entry("[[6]]").0;

    entries.push(two);
    entries.push(six);

    let two = parse_entry("[[2]]").0;
    let six = parse_entry("[[6]]").0;

    entries.sort_by(|a, b| compare(a, b));

    return 
        (entries.iter().position(|r| *r == two).unwrap() + 1) *
        (entries.iter().position(|r| *r == six).unwrap() + 1) ;
}

fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));

    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 13);

    let answer2 = solution1(&format!("{}b.txt", num));

    println!("Answer2: {:?}", answer2);
    assert_eq!(answer2, 5330);

    let answer3 = solution2(&format!("{}.txt", num));

    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 140);

    let answer4 = solution2(&format!("{}b.txt", num));
    println!("Answer4: {:?}", answer4);
    assert_eq!(answer4, 27648);
}

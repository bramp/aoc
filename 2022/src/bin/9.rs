
use std::collections::HashSet;
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

#[derive(Copy, Clone)]
#[derive(Eq, Hash, PartialEq)]
#[derive(Debug)]
struct XY {
    x: i64,
    y: i64,
}

fn solution1(filename: &str) -> usize {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let mut head = XY{x:0, y:0};
    let mut tail = head.clone();

    let mut seen = HashSet::<XY>::new();

    for line in lines {
        let (dir, count) = line.split_once(" ").unwrap();
        let mut count: i64 = count.parse().unwrap();

        while count > 0 {
            // Move head
            match dir {
                "L" => head.x -= 1,
                "R" => head.x += 1,
                "U" => head.y -= 1,
                "D" => head.y += 1,
                _ => panic!(),
            }

            // Move tail
            if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
                // do nothing they are adjancent
            } else if head.x == tail.x {
                tail.y += if head.y > tail.y { 1 } else { -1 }
            } else if head.y == tail.y {
                tail.x += if head.x > tail.x { 1 } else { -1 }
            } else {
                // must be diagonal
                tail.x += if head.x > tail.x { 1 } else { -1 };
                tail.y += if head.y > tail.y { 1 } else { -1 };
            }

            seen.insert(tail);

            count -= 1;
        }
    }

    return seen.len();
}


fn solution2(filename: &str) -> usize {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let mut rope = vec![XY{x:0, y:0}; 10];

    let mut seen = HashSet::<XY>::new();

    for line in lines {
        let (dir, count) = line.split_once(" ").unwrap();
        let mut count: i64 = count.parse().unwrap();

        while count > 0 {
            let head = rope.get_mut(0).unwrap();
            // Move head
            match dir {
                "L" => head.x -= 1,
                "R" => head.x += 1,
                "U" => head.y -= 1,
                "D" => head.y += 1,
                _ => panic!(),
            }

            for i in 1..rope.len() {
                let head = rope[i - 1];
                let tail = rope.get_mut(i).unwrap();

                // Move tail
                if (head.x - tail.x).abs() <= 1 && (head.y - tail.y).abs() <= 1 {
                    // do nothing they are adjancent
                } else if head.x == tail.x {
                    tail.y += if head.y > tail.y { 1 } else { -1 }
                } else if head.y == tail.y {
                    tail.x += if head.x > tail.x { 1 } else { -1 }
                } else {
                    // must be diagonal
                    tail.x += if head.x > tail.x { 1 } else { -1 };
                    tail.y += if head.y > tail.y { 1 } else { -1 };
                }
            }

            seen.insert(*rope.last().unwrap());

            count -= 1;
        }
    }

    return seen.len();
}

fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));

    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 13);

    let answer2 = solution1(&format!("{}b.txt", num));

    println!("Answer2: {:?}", answer2);
    assert_eq!(answer2, 6391);

    let answer3 = solution2(&format!("{}.txt", num));

    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 1);

    let answer4 = solution2(&format!("{}c.txt", num));

    println!("Answer4: {:?}", answer4);
    assert_eq!(answer4, 36);

    let answer5 = solution2(&format!("{}b.txt", num));
    println!("Answer5: {:?}", answer5);
    assert_eq!(answer5, 2593);
}

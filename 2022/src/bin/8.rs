
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

fn visible(r: usize, c: usize, trees: &Vec<Vec<u8>>) -> bool {
    let height = trees[r][c];

    // Can we be seen from the left, right, top or bottom?
    return 
        (0..c).map(|i| trees[r][i]).all(|x| x < height) |
        (c+1..trees[r].len()).map(|i| trees[r][i]).all(|x| x < height) |
        (0..r).map(|i| trees[i][c]).all(|x| x < height) |
        (r+1..trees.len()).map(|i| trees[i][c]).all(|x| x < height);
}


fn score(r: usize, c: usize, trees: &Vec<Vec<u8>>) -> u64 {
    let height = trees[r][c];

    if r == 0 || c == 0 || r == trees.len() - 1 || c == trees[0].len() - 1 {
        return 0;
    }

    let left = (0..c).rev().map(|i| trees[r][i]);
    let right = (c+1..trees[r].len()).map(|i| trees[r][i]);
    let up = (0..r).rev().map(|i| trees[i][c]);
    let down = (r+1..trees.len()).map(|i| trees[i][c]);

    // Distance to first tree as large or higher. If we find nothing, we return the range's length.
    return (
        left.clone().position(|x| x >= height).map_or(left.len(), |x| x + 1) *
        right.clone().position(|x| x >= height).map_or(right.len(), |x| x + 1) *
        up.clone().position(|x| x >= height).map_or(up.len(), |x| x + 1) *
        down.clone().position(|x| x >= height).map_or(down.len(), |x| x + 1)
    ).try_into().unwrap();
}



fn solution1(filename: &str) -> i32 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let mut trees: Vec<Vec::<u8>> = Vec::new();

    for line in lines {
        let row = line.chars().map(|x| (x as u8) - ('0' as u8)).collect();
        trees.push(row);
    }

    let rows = trees.len();
    let cols = trees[0].len();

    let mut count = 0;

    for r in 0..rows {
        for c in 0..cols {
            if visible(r, c, &trees) {
                count += 1;
            }
        }
    }

    return count;
}


fn solution2(filename: &str) -> u64 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let mut trees: Vec<Vec::<u8>> = Vec::new();

    for line in lines {
        let row = line.chars().map(|x| (x as u8) - ('0' as u8)).collect();
        trees.push(row);
    }

    let rows = trees.len();
    let cols = trees[0].len();

    let mut best = 0;

    for r in 0..rows {
        for c in 0..cols {
            best = max(best, score(r, c, &trees));
        }
    }

    return best.try_into().unwrap();
}

fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));

    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 21);

    let answer2 = solution1(&format!("{}b.txt", num));

    println!("Answer2: {:?}", answer2);
    assert_eq!(answer2, 1854);

    let answer3 = solution2(&format!("{}.txt", num));

    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 8);

    let answer4 = solution2(&format!("{}b.txt", num));
    println!("Answer4: {:?}", answer4);
    assert_eq!(answer4, 527340);
}

use core::cmp::max;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[macro_use]
extern crate text_io;

struct Target {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl Target {
    fn within(&self, x: i32, y: i32) -> bool {
        x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y
    }

    fn overshoot(&self, x: i32, y: i32) -> bool {
        x > self.max_x || y < self.min_y
    }
}

// Returns the max height for this angle
fn height(mut dx: i32, mut dy: i32, target: &Target) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;

    let mut max_height = 0;

    while !target.overshoot(x, y) {
        x += dx;
        y += dy;
        dx = max(dx - 1, 0); // Drag
        dy -= 1; // Gravity

        max_height = max(max_height, y);

        if target.within(x, y) {
            return Some(max_height);
        }
    }
    None
}

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    // target area: x=20..30, y=-10..-5
    let (min_x, max_x, min_y, max_y): (String, String, String, String);
    scan!(lines[0].bytes() => "target area: x={}..{}, y={}..{}", min_x, max_x, min_y, max_y);

    let t = Target {
        min_x: min_x.parse::<i32>().unwrap(),
        max_x: max_x.parse::<i32>().unwrap(),
        min_y: min_y.parse::<i32>().unwrap(),
        max_y: max_y.parse::<i32>().unwrap(),
    };

    // Lazy brute force. I should be able to work out the bounds on y
    let mut max_height = 0;
    for dx in 1..t.min_x {
        for dy in 1..1000 {
            if let Some(h) = height(dx, dy, &t) {
                max_height = max(max_height, h)
            }
        }
    }

    Ok(max_height)
}

fn part2(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    // target area: x=20..30, y=-10..-5
    let (min_x, max_x, min_y, max_y): (String, String, String, String);
    scan!(lines[0].bytes() => "target area: x={}..{}, y={}..{}", min_x, max_x, min_y, max_y);

    let t = Target {
        min_x: min_x.parse::<i32>().unwrap(),
        max_x: max_x.parse::<i32>().unwrap(),
        min_y: min_y.parse::<i32>().unwrap(),
        max_y: max_y.parse::<i32>().unwrap(),
    };

    // Lazy brute force. I should be able to work out the bounds on y
    let mut hit = 0;
    for dx in 1..=t.max_x {
        for dy in t.min_y..=1000 {
            if height(dx, dy, &t).is_some() {
                hit+=1
            }
        }
    }

    Ok(hit)
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/17.txt")?);
    println!("Part 2: {}", part2("data/17.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/17_test.txt").unwrap(), 45);
        assert_eq!(part1("data/17.txt").unwrap(), 10011);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/17_test.txt").unwrap(), 112);
        assert_eq!(part2("data/17.txt").unwrap(), 2994);
    }
}

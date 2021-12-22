use std::cmp::min;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut nums: Vec<i32> = lines[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    nums.sort_unstable();

    let target = nums[nums.len() / 2]; // 50%tile aka Median

    Ok(nums.iter().map(|x| (target - x).abs()).sum())
}

fn part2(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let nums: Vec<i32> = lines[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    let mut best_fuel = i32::MAX;
    let max_num = nums.iter().max().unwrap();

    // Brute force
    for target in 0..=*max_num {
        let fuel = nums
            .iter()
            .map(|x| (target - x).abs())
            .map(|x| ((x + 1) * x) / 2) // Triangle number forumula
            .sum();
        best_fuel = min(fuel, best_fuel)
    }

    Ok(best_fuel)
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/7.txt")?);
    println!("Part 2: {}", part2("data/7.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/7_test.txt").unwrap(), 37);
        assert_eq!(part1("data/7.txt").unwrap(), 340987);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/7_test.txt").unwrap(), 168);
        assert_eq!(part2("data/7.txt").unwrap(), 96987874);
    }
}

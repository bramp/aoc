use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut fish: Vec<i32> = lines[0]
        .split(',')
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for _ in 1..=80 {
        let mut to_add = 0;
        for f in fish.iter_mut() {
            if f == &0 {
                *f = 6;
                to_add += 1;
            } else {
                *f -= 1
            }
        }

        while to_add > 0 {
            fish.push(8);
            to_add -= 1
        }
    }

    Ok(fish.len().try_into().unwrap())
}

fn part2(filename: &str) -> io::Result<i64> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut ages = HashMap::<u8, i64>::new();

    lines[0]
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .for_each(|x| *ages.entry(x).or_default() += 1);

    for _ in 1..=256 {
        let to_add = *ages.entry(0).or_default();
        for age in 1..=8 {
            // Make every fish one year older
            *ages.entry(age - 1).or_default() = *ages.entry(age).or_default()
        }

        *ages.entry(6).or_default() += to_add; // those who reset
        *ages.entry(8).or_default() = to_add; // those new ones
    }

    Ok(ages.iter().map(|(_k, v)| v).sum())
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/6.txt")?);
    println!("Part 2: {}", part2("data/6.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/6_test.txt").unwrap(), 5934);
        assert_eq!(part1("data/6.txt").unwrap(), 352872);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/6_test.txt").unwrap(), 26984457539);
        assert_eq!(part2("data/6.txt").unwrap(), 1604361182149);
    }
}

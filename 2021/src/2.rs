use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec::Vec;

#[macro_use]
extern crate text_io;

fn part1() -> io::Result<i32> {
    let file = File::open("data/2.txt")?;
    let reader = BufReader::new(file);

    let mut pos = 0;
    let mut depth = 0;

    for line in reader.lines() {
        let line = line?;
        let (command, value): (String, i32);
        scan!(line.bytes() => "{} {}", command, value);
        match command.as_str() {
            "forward" => pos += value,
            "down" => depth += value,
            "up" => {
                if value > depth {
                    depth = 0;
                } else {
                    depth -= value;
                }
            }
            _ => panic!(),
        }
    }

    Ok(pos * depth)
}

fn part2() -> io::Result<i32> {
    let file = File::open("data/2.txt")?;
    let reader = BufReader::new(file);

    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line?;
        let (command, value): (String, i32);
        scan!(line.bytes() => "{} {}", command, value);
        match command.as_str() {
            "forward" => {
                pos += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!(),
        }
    }

    Ok(pos * depth)
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1()?);
    println!("Part 2: {}", part2()?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1().unwrap(), 2019945);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2().unwrap(), 1599311480);
    }
}

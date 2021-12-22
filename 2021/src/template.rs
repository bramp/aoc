use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(filename: &str) -> io::Result<i32> {
    let answer = 0;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    for _line in lines {}

    Ok(answer)
}

fn part2(filename: &str) -> io::Result<i32> {
    let answer = 0;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    for _line in lines {}

    Ok(answer)
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/TODO.txt")?);
    println!("Part 2: {}", part2("data/TODO.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/TODO.txt").unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/TODO.txt").unwrap(), 0);
    }
}

use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn part1() -> io::Result<i32> {
    let answer = 0;

    let file = File::open("data/TODO.txt")?;
    let reader = BufReader::new(file);

    for _line in reader.lines() {

    }

    Ok(answer)
}

fn part2() -> io::Result<i32> {
    let answer = 0;

    let file = File::open("data/TODO.txt")?;
    let reader = BufReader::new(file);

    for _line in reader.lines() {

    }


    Ok(answer)
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
        assert_eq!(part1().unwrap(), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2().unwrap(), 0);
    }
}
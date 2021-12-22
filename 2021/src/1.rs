use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::vec::Vec;

#[allow(dead_code)]
fn part1_simple()  -> io::Result<i32> {
    let file = File::open("data/1.txt")?;
    let reader = BufReader::new(file);

    let mut answer = 0;
    let mut lastvalue = 99999; // Ignore first value
    for line in reader.lines() {
        let value = line?.parse::<i32>().unwrap();
        if value > lastvalue {
            answer += 1;
        }
        lastvalue = value
    }

    Ok(answer)
}

fn part1()  -> io::Result<usize> {
    let file = File::open("data/1.txt")?;
    let reader = BufReader::new(file);

    Ok(reader.lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(2)
        .filter(|x| x[0] < x[1] )
        .count())

}

#[allow(dead_code)]
fn part2_simple()  -> io::Result<i32> {
    let file = File::open("data/1.txt")?;
    let reader = BufReader::new(file);

    let mut answer = 0;
    let lines : Vec<i32> = reader.lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    for i in 0..lines.len() - 3 {
        let a = lines[i] + lines[i + 1] + lines[i + 2];
        let b = lines[i+1] + lines[i + 2] + lines[i + 3];
        if b > a {
            answer += 1
        }
        println!("{} {}", a, b)
    }

    Ok(answer)
}

fn part2()  -> io::Result<usize> {
    let file = File::open("data/1.txt")?;
    let reader = BufReader::new(file);

    Ok(reader.lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>()
        .windows(4)
        .map(|x| (x[0] + x[1] + x[2], x[1] + x[2] + x[3]))
        .filter(|x| x.0 < x.1 )
        .count())

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
        assert_eq!(part1().unwrap(), 1226);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2().unwrap(), 1252);
    }
}
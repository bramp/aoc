use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};


fn part1(path: &str, digits: usize) -> io::Result<i32> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut digits_freq: Vec<HashMap<char, usize>> = Vec::new();
    for _i in 0..digits {
        digits_freq.push(HashMap::new());
    }

    for line in reader.lines() {
        let line = line?;

        for (i, c) in line.chars().enumerate() {
            *digits_freq[i].entry(c).or_default() += 1;
        }
    }

    let gamma = digits_freq.iter()
        .map(|freq| freq.iter()
            .max_by(|a, b| a.1.cmp(b.1)) // Find the most popular value
            .map(|(k, _v)| k)            // Return just the key
            .unwrap()
        ).collect::<String>();

    let epsilon = gamma.chars().map(|x| if x == '0' { '1' } else { '0' } ).collect::<String>();

    let power = i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilon, 2).unwrap();

    Ok(power)
}

fn digit_freq(list: &[String], position: usize) -> HashMap::<char, usize> {
    let mut freq = HashMap::<char, usize>::new();
    for s in list {
        *freq.entry(s.chars().nth(position).unwrap()).or_default() += 1;
    }

    freq
}

fn max_freq(freq : &HashMap::<char, usize>, default: char) -> char {
    if freq[&'0'] == freq[&'1'] {
        return default
    }

    *freq.iter()
        .max_by(|a, b| a.1.cmp(b.1)) // Find the most popular value
        .map(|(k, _v)| k)            // Return just the key
        .unwrap()
}

fn oxygen(lines: &[String], digits: usize) -> i32 {
    let mut lines = Vec::from(lines);

    for i in 0..digits {
        let freq = digit_freq(&lines, i);
        let c = max_freq(&freq, '1');

        lines = lines.iter()
            .filter(|x| x.chars().nth(i).unwrap() == c)
            .cloned()
            .collect();

        if lines.len() == 1 {
            return i32::from_str_radix(&lines[0], 2).unwrap();
        }

        assert!(!lines.is_empty());
    }

    panic!("not found");
}

// This seems way more complex than it needs to be, and needlessly duplicate oxygen
fn co2(lines: &[String], digits: usize) -> i32 {
    let mut lines = Vec::from(lines);

    for i in 0..digits {
        let freq = digit_freq(&lines, i);
        let c = max_freq(&freq, '1');
        let c = if c == '0' { '1' } else { '0' };

        lines = lines.iter()
            .filter(|x| x.chars().nth(i).unwrap() == c)
            .cloned()
            .collect();

        if lines.len() == 1 {
            return i32::from_str_radix(&lines[0], 2).unwrap();
        }

        assert!(!lines.is_empty());
    }

    panic!("not found");
}

fn part2(path: &str, digits: usize) -> io::Result<i32> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let lines : Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let o = oxygen(&lines, digits);
    let c = co2(&lines, digits);

    Ok(o * c)
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/3.txt", 12)?);
    println!("Part 2: {}", part2("data/3.txt", 12)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/3_test.txt", 5).unwrap(), 198);
        assert_eq!(part1("data/3.txt", 12).unwrap(), 4174964);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/3_test.txt", 5).unwrap(), 230);
        assert_eq!(part2("data/3.txt", 12).unwrap(), 4474944);
        
    }
}
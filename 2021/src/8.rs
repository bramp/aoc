use bimap::BiMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
//use itertools::Itertools;

fn part1(filename: &str) -> io::Result<i32> {
    let mut answer = 0;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    for line in lines {
        let mut line: Vec<String> = line.split_whitespace().map(String::from).collect();
        for segment in line.iter_mut() {
            // Sort all segments, to make later stuff easier
            let mut s = segment.chars().collect::<Vec<char>>();
            s.sort_unstable();
            *segment = s.iter().collect::<String>()
        }

        let seen = &line[0..10];
        let output = &line[11..15];

        let mut digits = HashMap::<String, u8>::new();

        for s in seen {
            match s.len() {
                2 => digits.insert(s.to_string(), 1),
                3 => digits.insert(s.to_string(), 7),
                4 => digits.insert(s.to_string(), 4),
                7 => digits.insert(s.to_string(), 8),
                _ => continue,
            };
        }

        for s in output {
            if digits.get(s).is_some() {
                answer += 1
            }
        }
    }

    Ok(answer)
}

fn unique(segment: &str) -> String {
    let mut s = segment.chars().collect::<Vec<char>>();
    s.sort_unstable();
    s.dedup();
    s.iter().collect::<String>()
}

fn part2(filename: &str) -> io::Result<i32> {
    let mut answer = 0;

    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    for line in lines {
        let mut line: Vec<String> = line.split_whitespace().map(String::from).collect();
        for segment in line.iter_mut() {
            // Sort all segments, to make later stuff easier
            *segment = unique(segment)
        }

        let seen = &line[0..10];
        let output = &line[11..15];

        let mut digits = BiMap::<String, u8>::new();
        let mut missing5 = Vec::new();
        let mut missing6 = Vec::new();

        // Easy ones (based on length)
        for s in seen {
            match s.len() {
                2 => {
                    digits.insert(s.to_string(), 1);
                }
                3 => {
                    digits.insert(s.to_string(), 7);
                }
                4 => {
                    digits.insert(s.to_string(), 4);
                }
                7 => {
                    digits.insert(s.to_string(), 8);
                }
                5 => {
                    // 2, 3, 5,
                    missing5.push(s.to_string());
                }
                6 => {
                    // 0, 6, 9,
                    missing6.push(s.to_string());
                }
                _ => {
                    panic!("unexpected length")
                }
            };
        }

        let d1 = digits.get_by_right(&1).unwrap().to_owned();
        let d8 = digits.get_by_right(&8).unwrap().to_owned();
        let d47 = unique(
            &(digits.get_by_right(&4).unwrap().to_owned() + digits.get_by_right(&7).unwrap()),
        );

        // Length 6 is 0, 6 and 9
        for s in &missing6 {
            // 9 = 4 + 7 + gggg.
            if unique(&(s.to_owned() + &d47)).len() == 6 {
                // Found 9
                digits.insert(s.to_string(), 9);

            // 8 = 6 + 1
            } else if unique(&(s.to_owned() + &d1)) == d8 {
                // Found 6
                digits.insert(s.to_string(), 6);

            // otherwise 
            } else {
                // Found 0
                digits.insert(s.to_string(), 0);
            }
        }

        let d6 = digits.get_by_right(&6).unwrap().to_owned();

        for s in &missing5 {
            // (2 or 3) + 6 is 8, otherwise it's 5
            if unique(&(s.to_owned() + &d6)) != d8 {
                // Found 5
                digits.insert(s.to_string(), 5);
            } else if unique(&(s.to_owned() + &d1)).len() == 5 {
                // Found 3
                digits.insert(s.to_string(), 3);
            } else {
                // Found 2
                digits.insert(s.to_string(), 2);
            }
        }

        let num : String = output.iter()
            .map(|x| digits.get_by_left(x).unwrap())
            .map(|x| x.to_string())
            .collect();

        answer += num.parse::<i32>().unwrap();
    }

    Ok(answer)
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/8.txt")?);
    println!("Part 2: {}", part2("data/8.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/8_test.txt").unwrap(), 26);
        assert_eq!(part1("data/8.txt").unwrap(), 512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/8_test.txt").unwrap(), 61229);
        assert_eq!(part2("data/8.txt").unwrap(), 1091165);
        
    }
}

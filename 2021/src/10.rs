use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut score = 0;

    for line in lines {
        let mut stack = Vec::new();

        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),

                ')' | ']' | '}' | '>' => {
                    let expected = stack.pop().unwrap(); // TODO incomplete
                    if c != expected {
                        score += match c {
                            ')' => 3,
                            ']' => 57,
                            '}' => 1197,
                            '>' => 25137,
                            _ => panic!("invalid char"),
                        };

                        //println!("Expected {}, but found {} instead.", expected, c);
                        break;
                    }
                }

                _ => panic!("invalid char"),
            };
        }
    }

    Ok(score)
}

fn part2(filename: &str) -> io::Result<i64> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut scores = Vec::new();

    for line in lines {
        let mut stack = Vec::new();

        for c in line.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),

                ')' | ']' | '}' | '>' => {
                    let expected = stack.pop().unwrap();
                    if c != expected {
                        stack.clear();
                        break; // Skip corrupt lines
                    }
                }

                _ => panic!("invalid char"),
            };
        }

        if !stack.is_empty() {
            let mut score = 0i64;
            stack.reverse();
            for c in &stack {
                score *= 5;
                score += match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => panic!("invalid char"),
                };
            }
            //println!("Complete by adding {:?}", stack);
            scores.push(score)
        }
    }
    scores.sort_unstable();
    Ok(scores[scores.len() / 2])
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/10.txt")?);
    println!("Part 2: {}", part2("data/10.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/10_test.txt").unwrap(), 26397);
        assert_eq!(part1("data/10.txt").unwrap(), 268845);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/10_test.txt").unwrap(), 288957);
        assert_eq!(part2("data/10.txt").unwrap(), 4038824534);
    }
}

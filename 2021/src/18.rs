use core::cmp::max;
use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[derive(Clone)]
struct Tokens(Vec<Token>);

#[derive(Copy, Clone, Debug, PartialEq)]
enum Token {
    Open,
    Close,
    Comma, // I don't really need this, but it makes printing easier
    Digit(i32),
}

fn prev_digit(input: &[Token], offset: usize) -> Option<(usize, i32)> {
    for j in (0..offset).rev() {
        if let Token::Digit(d) = input[j] {
            return Some((j, d));
        }
    }
    None
}

fn next_digit(input: &[Token], offset: usize) -> Option<(usize, i32)> {
    #[allow(clippy::needless_range_loop)]
    for j in offset..input.len() {
        if let Token::Digit(d) = input[j] {
            return Some((j, d));
        }
    }
    None
}

impl Tokens {
    fn new(input: &str) -> Self {
        // Tokenizes the input
        let mut output = Vec::<Token>::new();
        for c in input.chars() {
            match c {
                '[' => output.push(Token::Open),
                ']' => output.push(Token::Close),
                ',' => output.push(Token::Comma),

                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    // This assumes each number is a single digit
                    output.push(Token::Digit(c as i32 - '0' as i32))
                }
                _ => {
                    panic!("unexpected character")
                }
            }
        }
        Tokens(output)
    }

    fn add(&mut self, rhs: &Tokens) {
        self.0.insert(0, Token::Open);
        self.0.push(Token::Comma);
        self.0.extend_from_slice(&rhs.0);
        self.0.push(Token::Close);
    }

    fn explode(&mut self) -> bool {
        let input = &mut self.0;
        let mut depth = 0;

        for i in 0..input.len() {
            match input[i] {
                Token::Open => depth += 1,
                Token::Close => depth -= 1,

                _ => {
                    if depth > 4 {
                        // The left and right numbers in this pair
                        let left = if let Token::Digit(left) = input[i] {
                            left
                        } else {
                            panic!("expected digit");
                        };
                        assert!(input[i + 1] == Token::Comma);
                        let right = if let Token::Digit(right) = input[i + 2] {
                            right
                        } else {
                            panic!("expected digit");
                        };

                        if let Some((i, prev)) = prev_digit(input, i) {
                            input[i] = Token::Digit(prev + left);
                        }

                        if let Some((i, next)) = next_digit(input, i + 3) {
                            input[i] = Token::Digit(next + right);
                        }

                        // Now remove myself, and return a new vector
                        // [i,j]
                        input.drain(i..=i + 3);
                        input[i - 1] = Token::Digit(0);

                        return true;
                    }
                }
            }
        }

        false
    }

    fn split(&mut self) -> bool {
        let input = &mut self.0;
        for i in 0..input.len() {
            if let Token::Digit(d) = input[i] {
                if d >= 10 {
                    // SPLIT
                    let new = [
                        Token::Open,
                        Token::Digit((d as f32 / 2.0).floor() as i32),
                        Token::Comma,
                        Token::Digit((d as f32 / 2.0).ceil() as i32),
                        Token::Close,
                    ];
                    input.splice(i..i + 1, new);
                    return true;
                }
            }
        }
        false
    }

    fn reduce(&mut self) {
        loop {
            if self.explode() {
                continue;
            }
            if self.split() {
                continue;
            }
            // As soon as no operation was done, we can finish
            break;
        }
    }

    fn magnitude(&mut self) -> i32 {
        let input = &mut self.0;
        // Warning this could loop forever, we should failsafe this.
        while input.len() > 1 {
            for i in 0..input.len() - 4 {
                // Find all Open, Digit, Comma, Digit, Close
                if input[i] == Token::Open
                    && input[i + 2] == Token::Comma
                    && input[i + 4] == Token::Close
                {
                    let mut sum = 0;
                    if let Token::Digit(d) = input[i + 1] {
                        sum += 3 * d;
                    }
                    if let Token::Digit(d) = input[i + 3] {
                        sum += 2 * d;
                    }

                    input.splice(i..i + 5, vec![Token::Digit(sum)]);

                    break;
                }
            }
        }

        if let Token::Digit(d) = input[0] {
            return d;
        }

        panic!("invalid result")
    }
}

impl fmt::Display for Tokens {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for c in &self.0 {
            match c {
                Token::Open => write!(f, "[")?,
                Token::Close => write!(f, "]")?,
                Token::Comma => write!(f, ",")?,
                Token::Digit(d) => write!(f, "{}", d)?,
            };
        }
        Ok(())
    }
}

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut result = Tokens::new(&lines[0]);

    for line in lines[1..].iter() {
        let next = Tokens::new(line);
        result.add(&next);
        result.reduce();
    }

    Ok(result.magnitude())
}

fn part2(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<Tokens> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .map(|x| Tokens::new(&x))
        .collect();

    let mut max_magnitude = 0;

    // Brute force
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue;
            }

            let mut result = lines[i].clone();
            result.add(&lines[j]);
            result.reduce();
            max_magnitude = max(max_magnitude, result.magnitude());
        }
    }

    Ok(max_magnitude)
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/18.txt")?);
    println!("Part 2: {}", part2("data/18.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_explode() {
        fn run_explode(s: &str) -> String {
            let mut v = Tokens::new(s);
            v.explode();
            format!("{}", v)
        }

        assert_eq!(run_explode("[[[[[9,8],1],2],3],4]"), "[[[[0,9],2],3],4]");

        assert_eq!(run_explode("[7,[6,[5,[4,[3,2]]]]]"), "[7,[6,[5,[7,0]]]]");

        assert_eq!(run_explode("[[6,[5,[4,[3,2]]]],1]"), "[[6,[5,[7,0]]],3]");

        assert_eq!(
            run_explode("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),
            "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"
        );

        assert_eq!(
            run_explode("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
            "[[3,[2,[8,0]]],[9,[5,[7,0]]]]"
        );
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Tokens::new("[[1,2],[[3,4],5]]").magnitude(), 143);
        assert_eq!(
            Tokens::new("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]").magnitude(),
            1384
        );
        assert_eq!(
            Tokens::new("[[[[1,1],[2,2]],[3,3]],[4,4]]").magnitude(),
            445
        );
        assert_eq!(
            Tokens::new("[[[[3,0],[5,3]],[4,4]],[5,5]]").magnitude(),
            791
        );
        assert_eq!(
            Tokens::new("[[[[5,0],[7,4]],[5,5]],[6,6]]").magnitude(),
            1137
        );
        assert_eq!(
            Tokens::new("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]").magnitude(),
            3488
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/18_test1.txt").unwrap(), 3488);
        assert_eq!(part1("data/18_test2.txt").unwrap(), 4140);
        assert_eq!(part1("data/18.txt").unwrap(), 4365);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/18_test2.txt").unwrap(), 3993);
        assert_eq!(part2("data/18_test2.txt").unwrap(), 4490);
    }
}

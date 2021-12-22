use std::fs::File;
use std::io::{self, prelude::*, BufReader};

struct Board {
    board: Vec<Vec<u8>>,
    marked: Vec<Vec<bool>>,
}

impl Board {
    fn new() -> Self {
        Board {
            board: Vec::with_capacity(5),
            marked: vec![vec![false; 5]; 5],
        }
    }

    fn mark(&mut self, called: u8) {
        for (y, row) in self.board.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if *value == called {
                    self.marked[y][x] = true
                }
            }
        }
    }

    fn solved(&self) -> bool {
        // If any row is all true
        for row in self.marked.iter() {
            if row.iter().all(|x| *x) {
                return true;
            }
        }

        // If any col is all true
        for col in 0..5 {
            if self.marked.iter().map(|row| row[col]).all(|x| x) {
                return true;
            }
        }

        false
    }

    fn unmarked_sum(&self) -> i32 {
        let mut sum = 0i32;

        // If any row is all true
        for (y, row) in self.marked.iter().enumerate() {
            for (x, value) in row.iter().enumerate() {
                if !value {
                    sum += self.board[y][x] as i32;
                }
            }
        }

        sum
    }
}

fn read_input(filename: &str) -> io::Result<(Vec<u8>, Vec<Board>)> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let calls = lines[0]
        .split(',')
        .map(|x| x.parse::<u8>().unwrap())
        .collect();

    let mut boards = Vec::<Board>::new();
    for board in lines[1..].chunks(6) {
        let mut b = Board::new();

        for line in board.iter().skip(1) {
            // Skip blank line
            b.board.push(
                line.split_whitespace()
                    .map(|x| x.parse::<u8>().unwrap())
                    .collect(),
            );
        }
        boards.push(b);
    }

    Ok((calls, boards))
}

fn part1(filename: &str) -> io::Result<i32> {
    let (calls, mut boards) = read_input(filename)?;
    for call in calls {
        for board in boards.iter_mut() {
            board.mark(call);
            if board.solved() {
                return Ok((call as i32) * board.unmarked_sum());
            }
        }
    }

    panic!("No solution")
}

fn part2(filename: &str) -> io::Result<i32> {
    let (calls, mut boards) = read_input(filename)?;
    for call in calls {
        for board in boards.iter_mut() {
            board.mark(call);
        }

        if boards.len() == 1 && boards[0].solved() {
            return Ok((call as i32) * boards[0].unmarked_sum());
        }

        boards.retain(|board| !board.solved());
        assert!(!boards.is_empty());
    }

    panic!("No solution")
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/4.txt")?);
    println!("Part 2: {}", part2("data/4.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/4_test.txt").unwrap(), 4512);
        assert_eq!(part1("data/4.txt").unwrap(), 6592);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/4_test.txt").unwrap(), 1924);
        assert_eq!(part2("data/4.txt").unwrap(), 31755);
    }
}

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[macro_use]
extern crate text_io;

fn part1(filename: &str) -> io::Result<usize> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut grid = HashSet::new();
    let mut instructions = Vec::new();

    for line in lines.iter() {
        if line.is_empty() {
            break;
        }

        let mut s = line.split(',');
        let x = s.next().unwrap();
        let y = s.next().unwrap();

        grid.insert(
            (x.parse::<i32>().unwrap(),
                y.parse::<i32>().unwrap()));
    }

    // TODO skip seems wasteful, I assumed I could reuse the iterator
    for line in lines.iter().skip(grid.len() + 1) {
        instructions.push(line)
    }

    let (dir, pos): (char, i32);
    // fold along y=7

    scan!(instructions[0].bytes() => "fold along {}={}", dir, pos);

    let mut fold = HashSet::new();

    for (x, y) in grid.iter() {
        match dir {
            'x' => {
                // Mirror
                if x > &pos {
                    fold.insert( (pos - (x-pos), *y ) );
                } else if x < &pos {
                    fold.insert((*x,*y));
                };
            }
            'y' => {
                if y > &pos {
                    fold.insert( (*x, pos - (y-pos)) );
                } else if y < &pos {
                    fold.insert((*x,*y));
                };
            }
            _ => panic!("Unexpected direction"),
        }
    }

    Ok(fold.len())
}

fn part2(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut grid = HashSet::new();
    for line in lines.iter() {
        if line.is_empty() {
            break;
        }

        let mut s = line.split(',');
        let x = s.next().unwrap();
        let y = s.next().unwrap();

        grid.insert(
            (x.parse::<i32>().unwrap(),
                y.parse::<i32>().unwrap()));
    }

    // TODO skip seems wasteful, I assumed I could reuse the iterator
    for line in lines.iter().skip(grid.len() + 1) {
        let (dir, pos): (char, i32);
        scan!(line.bytes() => "fold along {}={}", dir, pos);

        let mut fold = HashSet::new();

        for (x, y) in grid.iter() {
            match dir {
                'x' => {
                    // Mirror
                    if x > &pos {
                        fold.insert( (pos - (x-pos), *y ) );
                    } else if x < &pos {
                        fold.insert((*x,*y));
                    };
                }
                'y' => {
                    if y > &pos {
                        fold.insert( (*x, pos - (y-pos)) );
                    } else if y < &pos {
                        fold.insert((*x,*y));
                    };
                }
                _ => panic!("Unexpected direction"),
            }
        }

        grid = fold;
    }

    for y in 0..10 {
        for x in 0..80 {
            if grid.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }

    Ok(0)
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/13.txt")?);
    println!("Part 2: {}", part2("data/13.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/13_test.txt").unwrap(), 17);
        assert_eq!(part1("data/13.txt").unwrap(), 647);
    }

    #[test]
    fn test_part2() {
        // The actual answer is "HEJHJRCJ"
        assert_eq!(part2("data/13.txt").unwrap(), 0);
    }
}

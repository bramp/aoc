use std::cmp::max;
use std::cmp::min;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

#[macro_use]
extern crate text_io;

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut points = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        let (x1, y1, x2, y2): (usize, usize, usize, usize);
        scan!(line.bytes() => "{},{} -> {},{}", x1, y1, x2, y2);
        points.push((x1, y1, x2, y2));

        max_x = max(max_x, max(x1, x2));
        max_y = max(max_y, max(y1, y2));
    }

    let mut grid: Vec<Vec<i32>> = vec![vec![0; max_x + 1]; max_y + 1];

    // Only vert/horz
    points.retain(|(x1, y1, x2, y2)| x1 == x2 || y1 == y2);

    // Populate grid
    for point in points {
        let (x1, y1, x2, y2) = point;
        if x1 == x2 {
            for y in min(y1, y2)..=max(y1, y2) {
                grid[y][x1] += 1;
            }
        } else if y1 == y2 {
            for x in min(x1, x2)..=max(x1, x2) {
                grid[y1][x] += 1;
            }
        }
    }

    let answer = grid
        .iter()
        .map(|row| row.iter().filter(|x| **x >= 2).count() as i32)
        .sum();

    Ok(answer)
}

fn part2(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut points = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for line in lines {
        let (x1, y1, x2, y2): (i32, i32, i32, i32);
        scan!(line.bytes() => "{},{} -> {},{}", x1, y1, x2, y2);
        points.push((x1, y1, x2, y2));

        max_x = max(max_x, max(x1, x2));
        max_y = max(max_y, max(y1, y2));
    }

    let mut grid: Vec<Vec<i32>> =
        vec![vec![0; (max_x + 1).try_into().unwrap()]; (max_y + 1).try_into().unwrap()];

    // Populate grid
    for point in points {
        let (x1, y1, x2, y2) = point;
        let steps = max(max(x1, x2) - min(x1, x2), max(y1, y2) - min(y1, y2));
        let step_x: i32 = match x1.cmp(&x2) {
            Ordering::Greater => -1,
            Ordering::Equal => 0,
            Ordering::Less => 1,
        };
        let step_y: i32 = match y1.cmp(&y2) {
            Ordering::Greater => -1,
            Ordering::Equal => 0,
            Ordering::Less => 1,
        };

        for i in 0..=steps {
            grid[(y1 + i * step_y) as usize][(x1 + i * step_x) as usize] += 1;
        }
    }

    let answer = grid
        .iter()
        .map(|row| row.iter().filter(|x| **x >= 2).count() as i32)
        .sum();

    Ok(answer)
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/5.txt")?);
    println!("Part 2: {}", part2("data/5.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/5_test.txt").unwrap(), 5);
        assert_eq!(part1("data/5.txt").unwrap(), 5690);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/5_test.txt").unwrap(), 12);
        assert_eq!(part2("data/5.txt").unwrap(), 17741);
    }
}

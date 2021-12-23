use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in lines {
        grid.push(line.chars().map(|x| x as u8 - b'0').collect())
    }

    let mut answer = 0;

    // Search
    for (y, row) in grid.iter().enumerate() {
        for (x, value) in row.iter().enumerate() {
            if y > 0 && grid[y - 1][x] <= *value {
                continue;
            }
            if y < grid.len() - 1 && grid[y + 1][x] <= *value {
                continue;
            }
            if x > 0 && grid[y][x - 1] <= *value {
                continue;
            }
            if x < row.len() - 1 && grid[y][x + 1] <= *value {
                continue;
            }

            answer += 1 + *value as i32;
        }
    }

    Ok(answer)
}

fn dfs(grid: &mut Vec<Vec<u8>>, x: usize, y: usize) -> i32 {
    if grid[y][x] == 9 {
        return 0;
    }

    let mut size = 1;

    // Mark here so we don't revisit
    grid[y][x] = 9;

    if y > 0 {
        size += dfs(grid, x, y - 1)
    }
    if y < grid.len() - 1 {
        size += dfs(grid, x, y + 1)
    }
    if x > 0 {
        size += dfs(grid, x - 1, y)
    }
    if x < grid[y].len() - 1 {
        size += dfs(grid, x + 1, y)
    }

    size
}

fn part2(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut grid: Vec<Vec<u8>> = Vec::new();

    for line in lines {
        grid.push(line.chars().map(|x| x as u8 - b'0').collect())
    }

    let mut found = Vec::<i32>::new();

    // Search
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != 9 {
                found.push(dfs(&mut grid, x, y));
            }
        }
    }
    found.sort_unstable();
    found.reverse();

    Ok(found[0] * found[1] * found[2])
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/9.txt")?);
    println!("Part 2: {}", part2("data/9.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/9_test.txt").unwrap(), 15);
        assert_eq!(part1("data/9.txt").unwrap(), 444);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/9_test.txt").unwrap(), 1134);
        assert_eq!(part2("data/9.txt").unwrap(), 1168440);
    }
}

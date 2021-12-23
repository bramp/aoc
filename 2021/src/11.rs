use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut grid = Vec::<Vec<i32>>::new();

    for line in lines {
        grid.push(line.chars().map(|x| x as i32 - '0' as i32).collect())
    }

    let mut flashes = 0;

    for _step in 0..100 {
        // Increase all by one
        for row in grid.iter_mut() {
            for v in row.iter_mut() {
                *v += 1
            }
        }

        // Find all the flashes
        loop {
            let mut found = false;
            let height = grid.len();
            for y in 0..height {
                let width = grid[y].len();
                for x in 0..width {
                    if grid[y][x] > 9 {
                        found = true;
                        grid[y][x] = -999999; // Hackish

                        if y > 0 {
                            if x > 0 {
                                grid[y - 1][x - 1] += 1;
                            }

                            grid[y - 1][x] += 1;

                            if x < width - 1 {
                                grid[y - 1][x + 1] += 1;
                            }
                        }

                        if x > 0 {
                            grid[y][x - 1] += 1;
                        }
                        if x < width - 1 {
                            grid[y][x + 1] += 1;
                        }

                        if y < height - 1 {
                            if x > 0 {
                                grid[y + 1][x - 1] += 1;
                            }

                            grid[y + 1][x] += 1;

                            if x < width - 1 {
                                grid[y + 1][x + 1] += 1;
                            }
                        }
                    }
                }
            }

            if !found {
                break;
            }
        }

        for row in grid.iter_mut() {
            for v in row.iter_mut() {
                // These are the ones that flashed this round
                if *v < 0 {
                    *v = 0;
                    flashes += 1;
                };
            }
        }
    }

    Ok(flashes)
}

fn part2(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut grid = Vec::<Vec<i32>>::new();

    let mut height = 0;
    let mut width = 0;

    for line in lines {
        grid.push(line.chars().map(|x| x as i32 - '0' as i32).collect());

        height += 1;
        width = line.len();
    }

    for step in 1..10000 {
        // Increase all by one
        for row in grid.iter_mut() {
            for v in row.iter_mut() {
                *v += 1
            }
        }

        // Find all the flashes
        loop {
            let mut found = false;

            for y in 0..height {
                for x in 0..width {
                    if grid[y][x] > 9 {
                        found = true;
                        grid[y][x] = -999999; // Hackish

                        if y > 0 {
                            if x > 0 {
                                grid[y - 1][x - 1] += 1;
                            }

                            grid[y - 1][x] += 1;

                            if x < width - 1 {
                                grid[y - 1][x + 1] += 1;
                            }
                        }

                        if x > 0 {
                            grid[y][x - 1] += 1;
                        }
                        if x < width - 1 {
                            grid[y][x + 1] += 1;
                        }

                        if y < height - 1 {
                            if x > 0 {
                                grid[y + 1][x - 1] += 1;
                            }

                            grid[y + 1][x] += 1;

                            if x < width - 1 {
                                grid[y + 1][x + 1] += 1;
                            }
                        }
                    }
                }
            }

            if !found {
                break;
            }
        }

        let mut flashes = 0;
        for row in grid.iter_mut() {
            for v in row.iter_mut() {
                // These are the ones that flashed this round
                if *v < 0 {
                    *v = 0;
                    flashes += 1;
                };
            }
        }
        if flashes == width * height {
            return Ok(step);
        }
    }

    panic!("Too many loops");
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/11.txt")?);
    println!("Part 2: {}", part2("data/11.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/11_test.txt").unwrap(), 1656);
        assert_eq!(part1("data/11.txt").unwrap(), 1652);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/11_test.txt").unwrap(), 195);
        assert_eq!(part2("data/11.txt").unwrap(), 220);
    }
}

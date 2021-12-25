use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::collections::HashSet;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// TODO I'm sure there is a nice iterator way of doing this
fn neighbors(grid: &[Vec<u8>], start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::<(usize, usize)>::with_capacity(4);

    // Assume a full rectangle.
    let height = grid.len();
    let width = grid[0].len();

    if start.0 > 0 {
        result.push((start.0 - 1, start.1));
    }
    if start.0 < height - 1 {
        result.push((start.0 + 1, start.1));
    }
    if start.1 > 0 {
        result.push((start.0, start.1 - 1));
    }
    if start.1 < width - 1 {
        result.push((start.0, start.1 + 1));
    }
    result
}

type CostMap = HashMap<(usize, usize), i32>;

// Using https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm as a source
// Originally didn't use a min heap, reduced time from 49.163s to 0.359s!
fn dijkstra(grid: &[Vec<u8>], start: (usize, usize)) -> CostMap {
    let height = grid.len();
    let width = grid[0].len();

    let mut q = PriorityQueue::new();
    let mut dist = HashMap::<(usize, usize), i32>::with_capacity(width * height);

    dist.insert(start, 0);
    q.push(start, Reverse(0));

    // u ← Q.extract_min()                    // Remove and return best vertex
    while let Some((u, _c)) = q.pop() {
        let ud = dist[&u];
        for v in neighbors(grid, u) {
            let alt = ud + grid[v.1][v.0] as i32; // Edge cost is the cost of landing on v

            let x = dist.entry(v).or_insert(i32::MAX);
            if alt < *x {
                *x = alt;
                q.push(v, Reverse(alt));

                // If we wanted to keep track of previous:
                //prev.insert(v, u);
            }
        }
    }

    dist
}

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut grid = Vec::<Vec<u8>>::new();
    for line in lines {
        let line = line.chars().map(|x| x as u8 - b'0').collect();
        grid.push(line)
    }

    let height = grid.len();
    let width = grid[0].len();

    let start = (0, 0);
    let end = (width - 1, height - 1);

    let dist = dijkstra(&grid, start);

    Ok(dist[&end])
}

// Uniform-cost search
// Using:
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// https://www.geeksforgeeks.org/uniform-cost-search-dijkstra-for-large-graphs/
// 0m2.951s
fn ucs(grid: &[Vec<u8>], start: (usize, usize), end: (usize, usize)) -> i32 {
    let mut q = PriorityQueue::new();
    let mut visited = HashSet::<(usize, usize)>::new();

    q.push(start, Reverse(0));

    while let Some((node, c)) = q.pop() {
        let c = c.0;

        if node == end {
            return c; // We have a solution
        }

        visited.insert(node);
        for n in neighbors(grid, node) {
            if !visited.contains(&n) {
                // We visited a new node, update if this is a quicker path.
                q.push_increase(n, Reverse(c + grid[n.1][n.0] as i32));
            }
        }
    }

    panic!("no solution")
}

// TODO I feel this is too long now
// 0m7.044s original
// 0m5.516s hardcode neighbors (instead of creating a vector)
// 0m4.026s start with a single entry Q (it helps prune the graph)
// 0m3.489s avoid a dict lookup
// 0m3.260s avoid more dict lookups
// 0m2.891s if I drop prev
// 0m2.770s if I pre-alloc dist
fn part2(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut grid = Vec::<Vec<u8>>::new();
    for line in lines {
        let mut final_line = Vec::new();
        let line: Vec<u8> = line.chars().map(|x| x as u8 - b'0').collect();

        // We need to repeat the line 5 more times.
        for i in 0..5 {
            // Always in range 1 to 9.
            final_line.extend(
                line.iter()
                    .map(|x| x + i)
                    .map(|x| if x > 9 { x - 9 } else { x }),
            );
        }

        grid.push(final_line)
    }

    // Now repeat the grid 5 times

    let mut final_grid = Vec::<Vec<u8>>::new();
    for i in 0..5 {
        for row in &grid {
            final_grid.push(
                row.iter()
                    .map(|x| x + i)
                    .map(|x| if x > 9 { x - 9 } else { x })
                    .collect(),
            );
        }
    }

    let height = final_grid.len();
    let width = final_grid[0].len();

    let start = (0, 0);
    let end = (width - 1, height - 1);

    let dist = ucs(&final_grid, start, end);

    Ok(dist)
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/15.txt")?);
    println!("Part 2: {}", part2("data/15.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/15_test.txt").unwrap(), 40);
        assert_eq!(part1("data/15.txt").unwrap(), 745);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/15_test.txt").unwrap(), 315);
        assert_eq!(part2("data/15.txt").unwrap(), 3002);
    }
}

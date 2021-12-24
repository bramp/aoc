use std::cmp::Reverse;
use priority_queue::PriorityQueue;
use std::collections::HashMap;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};

// TODO I'm sure there is a nice iterator way of doing this
fn neighbors(grid: &[Vec<u8>], start: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::<(usize, usize)>::new();

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
type NextMap = HashMap<(usize, usize), (usize, usize)>;

/*
#[derive(Eq)]
struct NodeCost {
    pub pos: (usize, usize),
    pub cost: i32
}

impl Ord for NodeCost {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for NodeCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for NodeCost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
*/

// Using https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm as a source
// Originally didn't use a min heap, reduced time from 49.163s to 0.359s!
fn dijkstra(grid: Vec<Vec<u8>>, start: (usize, usize)) -> (CostMap, NextMap) {
    let mut q = PriorityQueue::new();
    let mut dist = HashMap::<(usize, usize), i32>::new();
    let mut prev = HashMap::<(usize, usize), (usize, usize)>::new();

    dist.insert(start, 0);

    for (y, row) in grid.iter().enumerate() {
        for x in 0..row.len() {
            let v = (x, y);
            if v != start {
                dist.insert(v, i32::MAX);
            }
            q.push(v, Reverse(dist[&v]));
        }
    }

    while let Some((u, _)) = q.pop() {
        // u ← Q.extract_min()                    // Remove and return best vertex

        for v in neighbors(&grid, u) {
            let alt = dist[&u] + grid[v.1][v.0] as i32; // Edge cost is the cost of landing on v
            if alt < dist[&v] {
                dist.insert(v, alt);
                prev.insert(v, u);

                q.push_increase(v, Reverse(alt));
            }
        }
    }

    (dist, prev)
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

    let (dist, _prev) = dijkstra(grid, (0,0));

    Ok(dist[ &(width -1, height - 1) ])
}

// TODO I feel this is too long now
// 0m7.044s
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
        let line : Vec<u8> = line.chars().map(|x| x as u8 - b'0').collect();

        // We need to repeat the line 5 more times.
        for i in 0..5 {
            // Always in range 1 to 9.
            final_line.extend(line.iter().map(|x| x + i).map(|x| if x > 9 { x - 9 } else { x } ));
        }

        grid.push(final_line)
    }

    // Now repeat the grid 5 times

    let mut final_grid = Vec::<Vec<u8>>::new();
    for i in 0..5 { 
        for row in &grid {
            final_grid.push(row.iter().map(|x| x + i).map(|x| if x > 9 { x - 9 } else { x } ).collect());
        }
    }

    let height = final_grid.len();
    let width = final_grid[0].len();

    let (dist, _prev) = dijkstra(final_grid, (0,0));

    Ok(dist[ &(width -1, height - 1) ])
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
        assert_eq!(part2("data/15_test.txt").unwrap(), 3002);
    }
}

use std::collections::HashMap;
use multimap::MultiMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn part1(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let mut maze = MultiMap::<String, String>::new();

    for line in lines {
        let mut s = line.split('-');
        let a = s.next().unwrap();
        let b = s.next().unwrap();

        // Go both ways
        maze.insert(a.to_string(), b.to_string());
        maze.insert(b.to_string(), a.to_string());
    }

    let mut stack = Vec::<String>::new();

    let answer = dfs1(&maze, &mut stack, &"start".to_string());

    Ok(answer)
}

fn dfs1(maze: &MultiMap<String, String>, stack: &mut Vec<String>, start: &str) -> i32 {
    if start == "end" {
        //println!("{:?}", stack);
        return 1;
    }

    let mut paths = 0;

    stack.push(start.to_string());
    for n in maze.get_vec(start).unwrap().iter() {
        // Don't allow reentry if it's small
        if !n.eq(&n.to_lowercase()) || !stack.contains(n) {
            paths += dfs1(maze, stack, &n.to_string());
        }
    }
    stack.pop();

    paths
}

fn part2(filename: &str) -> io::Result<i32> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    //let mut nodes = HashSet::<String>::new();
    let mut maze = MultiMap::<&str, &str>::new();

    for line in lines.iter() {
        let mut s = line.split('-');
        let a = s.next().unwrap();
        let b = s.next().unwrap();

        // Go both ways
        maze.insert(a, b);
        maze.insert(b, a);
    }

    let mut visited = HashMap::<&str, i32>::new();

    let answer = dfs2(&maze, &mut visited, "start");

    Ok(answer)
}

fn can_visit(visited : &HashMap::<&str, i32>, next: &str) -> bool {
    // Don't revisit start
    if next == "start" {
        return false;
    }

    // Is big cave?
    if next.eq(&next.to_uppercase()) {
        return true;
    }

    // Have we visited a small cave twice yet?
    let has_dup = visited.iter()
        .any(|(_k, v)| v > &1);

    if visited.get(next).unwrap_or(&0) > &0 {
        return !has_dup;
    }

    true
}

fn dfs2<'a>(maze: &MultiMap<&str, &'a str>, visited: &mut HashMap::<&'a str, i32>, start: &'a str) -> i32 {
    if start == "end" {
        //println!("{:?}", visited);
        return 1;
    }

    let mut paths = 0;
    let is_small = start.to_lowercase() == start;

    if is_small {
        // Only keep track of small caves
        *visited.entry(start).or_default() += 1;
    }

    for n in maze.get_vec(start).unwrap().iter() {
        if can_visit(visited, n) {
            paths += dfs2(maze, visited, n);
        }
    }

    if is_small {
        *visited.entry(start).or_default() -= 1;
    }

    paths
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/12.txt")?);
    println!("Part 2: {}", part2("data/12.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/12_test.txt").unwrap(), 10);
        assert_eq!(part1("data/12_test2.txt").unwrap(), 226);
        assert_eq!(part1("data/12.txt").unwrap(), 3230);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/12_test.txt").unwrap(), 36);
        assert_eq!(part2("data/12_test2.txt").unwrap(), 3509);
        assert_eq!(part2("data/12.txt").unwrap(), 83475);
    }
}

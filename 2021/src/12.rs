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

// TODO This solution is really slow :(
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

    let mut stack = Vec::<&str>::new();

    let answer = dfs2(&maze, &mut stack, "start");

    Ok(answer)
}

fn can_visit(stack: &[&str], next: &str) -> bool {
    // Don't revisit start
    if next == "start" {
        return false;
    }

    // Is big cave?
    if next.eq(&next.to_uppercase()) {
        return true;
    }

    // Have we visited a small cave twice yet?
    let mut has_dup = false;
    let mut visited = HashSet::<&str>::new();
    for n in stack.iter().filter(|x| x.to_lowercase() == **x) {
        has_dup |= !visited.insert(n);
        // TODO was 0m16.998s
        //      now 0m14.126s
        //      now 0m12.716s
        //      now 0m11.873s
    }

    if visited.contains(next) {
        return !has_dup;
    }

    true
}

fn dfs2<'a>(maze: &MultiMap<&str, &'a str>, stack: &mut Vec<&'a str>, start: &'a str) -> i32 {
    if start == "end" {
        //println!("{:?}", stack);
        return 1;
    }

    let mut paths = 0;

    stack.push(start);
    for n in maze.get_vec(start).unwrap().iter() {
        if can_visit(stack, n) {
            paths += dfs2(maze, stack, n);
        }
    }
    stack.pop();

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

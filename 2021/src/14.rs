use std::collections::HashMap;
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

    let mut template: Vec<char> = lines[0].chars().collect();
    let mut rules = HashMap::<Vec<char>, char>::new();

    for line in lines.iter().skip(2) {
        let (from, to): (String, char);
        // CH -> B
        scan!(line.bytes() => "{} -> {}", from, to);

        rules.insert(from.chars().collect::<Vec<char>>(), to);
    }

    for _step in 0..10 {
        // println!("{:?}", template);

        let mut output = vec![template[0]];

        for i in 0..template.len() - 1 {
            let m = &template[i..i + 2];
            if let Some(new) = rules.get(m) {
                output.push(*new);
            }
            output.push(template[i + 1]);
        }

        template = output;
    }

    let mut counts = HashMap::<char, i32>::new();
    template
        .iter()
        .for_each(|x| *counts.entry(*x).or_default() += 1);

    let mut counts: Vec<_> = counts.values().collect();
    counts.sort_unstable();

    Ok(counts[counts.len() - 1] - counts[0])
}

fn part2(filename: &str) -> io::Result<i64> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect();

    let template: Vec<char> = lines[0].chars().collect();
    let mut rules = HashMap::<(char, char), char>::new();

    for line in lines.iter().skip(2) {
        let (from, to): (String, char);
        // CH -> B
        scan!(line.bytes() => "{} -> {}", from, to);

        let x = from.chars().next().unwrap();
        let y = from.chars().nth(1).unwrap();

        rules.insert((x, y), to);
    }

    let mut poly = HashMap::<(char, char), i64>::new();

    for i in 0..template.len() - 1 {
        let x = template[i];
        let y = template[i + 1];

        *poly.entry((x, y)).or_default() += 1
    }

    for _step in 0..40 {
        //println!("{:?}", poly);

        let mut output = HashMap::<(char, char), i64>::new();

        for (key, counts) in poly.iter() {
            if let Some(new) = rules.get(key) {
                // NN turns into NC and CN
                *output.entry((key.0, *new)).or_default() += counts;
                *output.entry((*new, key.1)).or_default() += counts;
            } else {
                *output.entry(*key).or_default() += counts;
            }
        }

        poly = output;
    }

    // Count how many times the first character appears
    let mut counts = HashMap::<char, i64>::new();
    for (key, count) in &poly {
        *counts.entry(key.0).or_default() += count
    }

    let mut counts: Vec<_> = counts.values().collect();
    counts.sort_unstable();

    // Add one for the last character (TODO that might be a bug)
    Ok(counts[counts.len() - 1] - counts[0] + 1)
}

fn main() -> io::Result<()> {
    println!("Part 1: {}", part1("data/14.txt")?);
    println!("Part 2: {}", part2("data/14.txt")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1("data/14_test.txt").unwrap(), 1588);
        assert_eq!(part1("data/14.txt").unwrap(), 2408);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2("data/14_test.txt").unwrap(), 2188189693529);
        assert_eq!(part2("data/14.txt").unwrap(), 2651311098752);
    }
}

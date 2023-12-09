
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ffi::OsStr;

fn puzzle() -> i32 {
    return env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
        .expect("Unknown puzzle number")
        .parse::<i32>()
        .unwrap();
}

fn solution1(filename: &str) -> i32 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let mut strengths = vec![];
    let interestin_cycles = HashSet::from([20, 60, 100, 140, 180, 220]);

    let mut cycle = 1;
    let mut x = 1;

    for line in lines {
        if interestin_cycles.contains(&cycle) {
            strengths.push(cycle * x);
        }
        
        if line.starts_with("noop") {
            cycle += 1;

        } else if line.starts_with("addx") {
            let amount = line.split_once(" ").unwrap().1.parse::<i32>().unwrap();
            
            cycle += 1;

            if interestin_cycles.contains(&cycle) {
                strengths.push(cycle * x);
            }

            x += amount;
            cycle += 1;

        } else {
            panic!("Unknown instruction: {}", line);
        }
    }

    return strengths.iter().sum();
}


fn solution2(filename: &str) -> String {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let mut cycle = 0;
    let mut x = 1;

    let mut screen = "".to_string();

    for line in lines {
        if (cycle % 40) >= (x-1) && (cycle % 40) <= x + 1 {
            // Draw
            screen.push('#');
        } else {
            screen.push('.');
        }

        if line.starts_with("noop") {
            cycle += 1;

        } else if line.starts_with("addx") {
            let amount = line.split_once(" ").unwrap().1.parse::<i32>().unwrap();
            
            cycle += 1;

             if (cycle % 40) >= (x-1) && (cycle % 40) <= x + 1 {
                // Draw
                screen.push('#');
            } else {
                screen.push('.');
            }

            x += amount;
            cycle += 1;

        } else {
            panic!("Unknown instruction: {}", line);
        }
    }

    return screen.as_bytes().chunks(40).map(|x| std::str::from_utf8(x).unwrap()).collect::<Vec<_>>().join("\n");
}

fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));

    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 13140);

    let answer2 = solution1(&format!("{}b.txt", num));

    println!("Answer2: {:?}", answer2);
    assert_eq!(answer2, 13220);

    let answer3 = solution2(&format!("{}.txt", num));

    println!("Answer3:\n{}\n", answer3);
    assert_eq!(answer3, 
"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
".trim());

    let answer4 = solution2(&format!("{}b.txt", num));
    println!("Answer4: \n{}\n", answer4);
    assert_eq!(answer4,"
###..#..#..##..#..#.#..#.###..####.#..#.
#..#.#..#.#..#.#.#..#..#.#..#.#....#.#..
#..#.#..#.#..#.##...####.###..###..##...
###..#..#.####.#.#..#..#.#..#.#....#.#..
#.#..#..#.#..#.#.#..#..#.#..#.#....#.#..
#..#..##..#..#.#..#.#..#.###..####.#..#.
".trim());
}

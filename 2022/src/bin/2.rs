
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::Reverse;
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

enum Play {
    Rock,
    Paper,
    Scissors,
}

impl Play { 
    fn score(&self) -> i32 { 
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    } 
} 

enum Game {
    Win,
    Lose,
    Draw,
}

impl Game { 
    fn score(&self) -> i32 { 
        match self {
            Game::Lose => 0,
            Game::Draw => 3,
            Game::Win => 6,
        }
    } 
} 


fn play(play1: &Play, play2: &Play) -> Game {
    match play1 {
        Play::Rock => match play2 {
            Play::Rock => Game::Draw,
            Play::Paper => Game::Lose,
            Play::Scissors => Game::Win,
        },
        Play::Paper => match play2 {
            Play::Rock => Game::Win,
            Play::Paper => Game::Draw,
            Play::Scissors => Game::Lose,
        },
        Play::Scissors => match play2 {
            Play::Rock => Game::Lose,
            Play::Paper => Game::Win,
            Play::Scissors => Game::Draw,
        },
    }
}

fn should_play(theirs: &Play, outcome: &Game) -> Play {
    match theirs {
        Play::Rock => match outcome {
            Game::Draw => Play::Rock,
            Game::Lose => Play::Scissors,
            Game::Win  => Play::Paper,
        },
        Play::Paper => match outcome {
            Game::Win =>  Play::Scissors,
            Game::Draw => Play::Paper ,
            Game::Lose => Play::Rock ,
        },
        Play::Scissors => match outcome {
            Game::Lose => Play::Paper,
            Game::Win  => Play::Rock,
            Game::Draw => Play::Scissors, 
        },
    }
}



fn solution1(filename: &str) -> i32 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file).lines();

    let mut total = 0;

    for line in lines {
        let line = line.expect("failed to read line");
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let plays = line.split_whitespace();

        let plays = plays.map(|play| {
            match play {
                "A" | "X" => Play::Rock,
                "B" | "Y" => Play::Paper,
                "C" | "Z" => Play::Scissors,
                _ => panic!("Unknown play: {}", play),
            }
        }).collect::<Vec<Play>>();
        assert!(plays.len() == 2);

        let r = play(&plays[1], &plays[0]);
        let score = &plays[1].score() + r.score();
        total += score;
    }

    return total;
}


fn solution2(filename: &str) -> i32 {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file).lines();

    let mut total = 0;

    for line in lines {
        let line = line.expect("failed to read line");
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let plays = line.split_whitespace().collect::<Vec<&str>>();
        assert!(plays.len() == 2);

        let their = match plays[0] {
            "A" => Play::Rock,
            "B" => Play::Paper,
            "C" => Play::Scissors,
            _ => panic!("Unknown play: {}", plays[0]),
        };

        let outcome = match plays[1] {
            "X" => Game::Lose,
            "Y" => Game::Draw,
            "Z" => Game::Win,
            _ => panic!("Unknown game: {}", plays[1]),
    
        };

        let mine = should_play(&their, &outcome);

        let score = mine.score() + outcome.score();
        total += score;
    }

    return total;
}

fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));
    let answer2 = solution1(&format!("{}b.txt", num));

    assert_eq!(answer1, 15);
    assert_eq!(answer2, 14375);

    println!("Answer1: {:?}", answer1);
    println!("Answer2: {:?}", answer2);

    let answer3 = solution2(&format!("{}.txt", num));
    let answer4 = solution2(&format!("{}b.txt", num));

    assert_eq!(answer3, 12);
    assert_eq!(answer4, 10274);

    println!("Answer3: {:?}", answer3);
    println!("Answer4: {:?}", answer4);

}

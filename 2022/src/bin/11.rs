
use derivative::Derivative;
use regex::Regex;
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

#[derive(Derivative)]
#[derivative(Debug)]
struct Monkey<'a> {
    items: Vec<u64>,

    #[derivative(Debug="ignore")]
    operation: Box<dyn Fn(u64) -> u64 + 'a>,

    #[derivative(Debug="ignore")]
    test: Box<dyn Fn(u64) -> bool + 'a>,
    test_divisor: u64,

    next_monkey_true: usize,
    next_monkey_false: usize,

    inspections: u64,
}

enum Value {
    Old,
    Value(u64),
}

fn parse_monkeys(filename: &str) -> Vec<Monkey> {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let re1 = Regex::new(r"^Monkey ([0-9]+):").unwrap();
    let re2 = Regex::new(r"Starting items: ([0-9, ]+)").unwrap();
    let re3 = Regex::new(r"Operation: new = old ([*+]) ([0-9]+|old)").unwrap();
    let re4 = Regex::new(r"Test: divisible by ([0-9]+)").unwrap();
    let re5 = Regex::new(r"If (true|false): throw to monkey ([0-9]+)").unwrap();
    
    // Parse Monkey input
    let mut current_monkey: Option<usize> = None;
    let mut monkeys = vec![];

    for line in lines {
        // let re1 = Regex::new(r"^Monkey ([0-9]+): ").unwrap();
        if let Some((_, [monkey])) = re1.captures(&line).map(|c| c.extract()) {
            current_monkey = Some(monkey.parse::<usize>().unwrap());
            assert!(monkeys.len() == current_monkey.unwrap());

            monkeys.push(Monkey{
                items: Vec::new(),

                operation: Box::new(&|x| x),
                test: Box::new(|_x| false),
                test_divisor: 0,

                next_monkey_true: 0,
                next_monkey_false: 0,

                inspections: 0,
            });

        // (r"Starting items: ([0-9, ]+): ").unwrap();
        } else if let Some((_, [items])) = re2.captures(&line).map(|c| c.extract()) {
            println!("items: {:?}", items);

            monkeys[current_monkey.unwrap()].items = items.split(", ")
                .map(|i| i.parse::<u64>().unwrap())
                .collect::<Vec<_>>();

        // r"Operation: new = old ([*+]) ([0-9]+|old): "
        } else if let Some((_, [op, value])) = re3.captures(&line).map(|c| c.extract()) {
            let op = op.to_owned();
            let value = if value == "old" {
                Value::Old
            } else {
                Value::Value(value.parse().unwrap())
            };

            monkeys[current_monkey.unwrap()].operation = Box::new(move |x| {
                let value = match value {
                    Value::Old => x,
                    Value::Value(y) => y,
                };

                return if op == "+" {
                    x + value
                } else if op == "*" {
                    x * value
                } else {
                    panic!();
                }
            });

        // "Test: divisible by ([0-9]+) 
        } else if let Some((_, [div])) = re4.captures(&line).map(|c| c.extract()) {
            let div: u64 = div.parse().unwrap();

            monkeys[current_monkey.unwrap()].test = Box::new(move |x| {
                return x % div == 0;
            });
            monkeys[current_monkey.unwrap()].test_divisor = div;

        // "If ([true|false]): throw to monkey ([0-9]+)"
        } else if let Some((_, [cond, monkey])) = re5.captures(&line).map(|c| c.extract()) {
            let monkey = monkey.parse().unwrap();

            // Can't throw to self
            assert!(monkey != current_monkey.unwrap());

            if cond == "true" {
                monkeys[current_monkey.unwrap()].next_monkey_true = monkey;
            } else if cond == "false" {
                monkeys[current_monkey.unwrap()].next_monkey_false = monkey;
            } else {
                panic!();
            }
        } else {
            panic!("unknown line {:?}", line);
        }
    }

    return monkeys;

}

fn solution1(filename: &str) -> u64 {
    let mut monkeys = parse_monkeys(filename);

    // Now run
    for _i in 0..20 {
        for m in 0..monkeys.len() {

            for item in 0..monkeys[m].items.len() {
                let monkey = &mut monkeys[m];
                monkey.inspections += 1;

                let item = monkey.items[item];
                let new_item = (monkey.operation)(item);
                let new_item = new_item / 3;

                let next = if (monkey.test)(new_item) {
                    monkey.next_monkey_true
                } else {
                    monkey.next_monkey_false
                };

                assert!(next != m);

                monkeys[next].items.push(new_item);
            }

            // All thrown.
            monkeys[m].items.clear();
        }
    }

    let mut inspections = monkeys.iter()
        .map(|m| m.inspections)
        .collect::<Vec<_>>();

    inspections.sort();

    return inspections.iter().rev()
        .take(2)
        .product();
}


fn solution2(filename: &str) -> u64 {
    let mut monkeys = parse_monkeys(filename);

    let lim: u64 = monkeys.iter().map(|m| m.test_divisor).product();

    // Now run
    for _i in 0..10000 {
        println!("{}", _i);

        for m in 0..monkeys.len() {

            for item in 0..monkeys[m].items.len() {
                let monkey = &mut monkeys[m];
                monkey.inspections += 1;

                let item = monkey.items[item];
                let new_item = (monkey.operation)(item);
                let new_item = new_item % lim;

                let next = if (monkey.test)(new_item) {
                    monkey.next_monkey_true
                } else {
                    monkey.next_monkey_false
                };

                assert!(next != m);

                monkeys[next].items.push(new_item);
            }

            // All thrown.
            monkeys[m].items.clear();
        }
    }

    println!("monkeys: {:?}", monkeys);

    let mut inspections = monkeys.iter()
        .map(|m| m.inspections)
        .collect::<Vec<_>>();

    inspections.sort();

    return inspections.iter().rev()
        .take(2)
        .product();
}


fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));

    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 10605);

    let answer2 = solution1(&format!("{}b.txt", num));

    println!("Answer2: {:?}", answer2);
    assert_eq!(answer2, 58786);

    let answer3 = solution2(&format!("{}.txt", num));

    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 2713310158);

    let answer4 = solution2(&format!("{}b.txt", num));
    println!("Answer4: {:?}", answer4);
    assert_eq!(answer4, 14952185856);
}

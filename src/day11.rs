use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use regex::Regex;

enum OPERATION {
    SUM,
    SUB,
    MULT,
    DIV
}

pub struct Monkey {
    id: usize,
    items: Vec<u32>,
    operation: (String, Option<u32>),
    test: u32,
    true_branch: usize,
    false_branch: usize
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> u32 {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut line_iter = input.lines();
    let num_re = Regex::new(r"\d+").unwrap();
    let operation_re = Regex::new(r"(\*|\+)|(\d+|old)").unwrap();
    let mut res = 0;

    while let Some(line) = line_iter.next() {
        let id = num_re.find(line).unwrap().as_str().parse::<usize>().unwrap();
        
        let items: Vec<u32> = num_re.find_iter(line_iter.next().unwrap()).map(|item| {
            item.as_str().parse::<u32>().unwrap()
        }).collect_vec();

        let op_str = operation_re.find_iter(line_iter.next().unwrap()).map(|item| {
            item.as_str()
        }).collect_vec();
        let operation: (String, Option<u32>);
        if op_str[2] == "old" {
            operation = (op_str[1].to_string(), None);
        } else {
            operation = (op_str[1].to_string(), Some(op_str[2].parse::<u32>().unwrap()));
        }

        let test = num_re.find(line_iter.next().unwrap()).unwrap().as_str().parse::<u32>().unwrap();

        let true_branch = num_re.find(line_iter.next().unwrap()).unwrap().as_str().parse::<usize>().unwrap();

        let false_branch = num_re.find(line_iter.next().unwrap()).unwrap().as_str().parse::<usize>().unwrap();

        monkeys.push(Monkey {id, items, operation, test, true_branch, false_branch});

        let _ = line_iter.next();
    }
    
    for monkey in monkeys {
        for item in &monkey.items {
            match monkey.operation.0.as_str() {
                "+" => {
                    match monkey.operation.1.is_some() {
                        true => {*item += monkey.operation.1.unwrap()},
                        false => {*item += *item}
                    }
                },
                _ => {
                    match monkey.operation.1.is_some() {
                        true => {*item *= monkey.operation.1.unwrap()},
                        false => {*item *= *item}
                    }
                }
            }
            *item /= 3;

            if item % monkey.test == 0 {
                monkeys[monkey.true_branch].items.push(*item);
            } else {
                monkeys[monkey.false_branch].items.push(*item);
            }
        }
    }

    res
}

#[aoc(day11, part2)]
pub fn part2(games: &str) -> u32 {
    let mut res = 0;
    

    res
}
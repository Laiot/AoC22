use aoc_runner_derive::aoc;
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
    false_branch: usize,
    activity: usize
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut line_iter = input.lines();
    let num_re = Regex::new(r"\d+").unwrap();
    let operation_re = Regex::new(r"(\*|\+)|(\d+|old)").unwrap();

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

        monkeys.push(Monkey {id, items, operation, test, true_branch, false_branch, activity: 0});

        let _ = line_iter.next();
    }
    
    for _ in 0..19 {
        for ind in 0..monkeys.len() {
            monkeys[ind].activity += monkeys[ind].items.len();
            for _ in 0..monkeys[ind].items.len() {
                let mut item = monkeys[ind].items.pop().unwrap();
                match monkeys[ind].operation.0.as_str() {
                    "+" => {
                        match monkeys[ind].operation.1.is_some() {
                            true => {item += monkeys[ind].operation.1.unwrap()},
                            false => {item += item}
                        }
                    },
                    _ => {
                        match monkeys[ind].operation.1.is_some() {
                            true => {item *= monkeys[ind].operation.1.unwrap()},
                            false => {item *= item}
                        }
                    }
                }
                item /= 3;
                if item % monkeys[ind].test == 0 {
                    let tbranch = monkeys[ind].true_branch;
                    monkeys[tbranch].items.push(item);
                } else {
                    let fbranch = monkeys[ind].false_branch;
                    monkeys[fbranch].items.push(item);
                }
            }
        }
    }

    let mut res1 = 0;
    let mut res2 = 0;

    for monkey in monkeys {
        if monkey.activity > res1 {
            if monkey.activity > res2 {
                res2 = monkey.activity;
            } else {
                res1 = monkey.activity;
            }
        }
    }
    res1 * res2
}

#[aoc(day11, part2)]
pub fn part2(games: &str) -> u32 {
    let mut res = 0;
    

    res
}
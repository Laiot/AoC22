use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

pub struct Step {
    num_of_items: usize,
    init_pos: usize,
    final_pos: usize
}

#[aoc_generator(day5)]
pub fn part1_gen(input: &str) -> (Vec<Vec<char>>, Vec<Step>) {
    let mut lines_iter_own = input.lines().peekable();
    let lines_iter = lines_iter_own.by_ref();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); lines_iter.peek().unwrap().len()/4 + 1];
    let mut steps: Vec<Step> = Vec::new();

    while !lines_iter.peek().unwrap().contains('1'){
        for (ind, item) in lines_iter.next().unwrap().chars().enumerate() {
            if ind != 0 && (ind - 1) % 4 == 0 {
                stacks[ind / 4].push(item);
            }
        }
    }

    for i in 0..stacks.len(){
        stacks[i].reverse();
        stacks[i].retain(|&x| x != ' ');
    }

    let empty1 = lines_iter.next();
    let empty2 = lines_iter.next();

    let re = Regex::new(r"\d+").unwrap();

    for line in lines_iter {
        let step_params: Vec<usize> = re.find_iter(line)
            .filter_map(|digits| digits.as_str().parse().ok())
            .collect();
        
        steps.push(Step {num_of_items: *step_params.get(0).unwrap(), 
            init_pos: *step_params.get(1).unwrap(), 
            final_pos: *step_params.get(2).unwrap()});
    }

    (stacks, steps)
}

#[aoc(day5, part1)]
pub fn part1(input: &(Vec<Vec<char>>, Vec<Step>)) -> String {
    let mut stacks = input.0.clone();
    let mut res = String::new();
    for step in &input.1 {
        for _ in 0..step.num_of_items {
            'stackloop: for stack in stacks.clone().get(step.init_pos - 1) {
                for item in stack.iter().clone().rev() {
                    if !item.is_whitespace() {
                        stacks[step.init_pos - 1].pop();
                        stacks[step.final_pos - 1].push(*item);
                        break 'stackloop;
                    }
                }
            }
        }
    }

    for stack in stacks {
        res.push(*stack.last().unwrap());
    }

    res
}

#[aoc(day5, part2)]
pub fn part2(input: &(Vec<Vec<char>>, Vec<Step>)) -> String {
    let mut stacks = input.0.clone();

    let mut res = String::new();
    for step in &input.1 {
        'stackloop: for stack in input.0.clone().get(step.init_pos - 1) {
            for item in stack.iter().clone().rev() {
                if !item.is_whitespace() {
                    let mut pops_vec: Vec<char> = Vec::new();
                    let slen = stacks.clone()[step.init_pos - 1].len();
                    for popped in stacks[step.init_pos - 1].drain((slen - step.num_of_items)..) {
                        pops_vec.push(popped);
                    }
                    stacks[step.final_pos - 1].append(&mut pops_vec);
                    break 'stackloop;
                }
            }
        }
    }

    for stack in stacks {
        res.push(*stack.last().unwrap());
    }

    res

}
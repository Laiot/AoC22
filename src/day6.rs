use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;

pub fn unique_substring(v: &Vec<char>, sub_size: usize) -> usize {
    let mut counter = 0;
    let mut res = 0;
    for window in v.windows(sub_size) {
        let mut uniq = HashSet::new();
        if window.into_iter().all(|x| uniq.insert(x)) {
            res = counter + sub_size;
            break;
        }
        counter += 1;
    }
    res
}

#[aoc_generator(day6)]
pub fn day6_gen(input: &str) -> Vec<char> {
    input.chars().collect_vec()
}

#[aoc(day6, part1)]
pub fn part1(code: &Vec<char>) -> usize {
    unique_substring(code, 4)
}

#[aoc(day6, part2)]
pub fn part2(code: &Vec<char>) -> usize {
    unique_substring(code, 14)
}
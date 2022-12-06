use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::{Reverse, self};
use std::collections::BinaryHeap;

#[aoc_generator(day1)]
pub fn day1_gen(input: &str) -> Vec<u32> {
    let (mut vec, last) = input.lines()
    .fold((Vec::new(), 0), |mut a, x|{
        match x.parse::<u32>() {
            Ok(x) => {(a.0, a.1+x)}
            _ => {a.0.push(a.1); (a.0, 0)}
        }
    });
    vec.push(last); 
    vec
}

#[aoc(day1, part1)]
pub fn part1(calories: &Vec<u32>) -> u32 {
    *calories.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(calories: &Vec<u32>) -> u32 {
    calories.iter().fold(BinaryHeap::new(), |mut h, e|{
        if h.len() < 3 {
            h.push(Reverse(*e))
        } else {
            match h.pop() {
                Some(Reverse(x)) => {h.push(Reverse(cmp::max(x, *e)))}
                _ => {}
            }
        }
        h
     }).iter().map(|x| x.0).sum()
}
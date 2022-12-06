use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::{Reverse, self};
use std::collections::BinaryHeap;

// Day 1: https://adventofcode.com/2022/day/1

/*
Input handler for both parts of Day 1.
 
Every time a line can be correctly parsed as an integer (u8), the value is saved inside a 
counter. When a line cannot be correctly parsed as integer, the previous variable is pushed
into a vector.
 
The function returns the said vectors of integers.
TODO find a way to concatene the last push (something like pipe()).
*/
#[aoc_generator(day1)]
pub fn day1_gen(input: &str) -> Vec<u8> {
    let (mut vec, last) = input.lines()
    .fold((Vec::new(), 0), |mut a, x|{
        match x.parse::<u8>() {
            Ok(x) => {(a.0, a.1+x)}
            _ => {a.0.push(a.1); (a.0, 0)}
        }
    });
    vec.push(last); 
    vec
}

/*
Part 1

Returns the highest values between the integers inside the vector received as parameter.
*/
#[aoc(day1, part1)]
pub fn part1(calories: &Vec<u8>) -> u8 {
    *calories.iter().max().unwrap()
}

/*
Part 2

With the use of a BinaryHeap, the function takes the three highest
values and returns their sum.
*/
#[aoc(day1, part2)]
pub fn part2(calories: &Vec<u8>) -> u8 {
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
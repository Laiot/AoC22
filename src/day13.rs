use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

pub enum Packet {
    INTEGER(u32),
    LIST(Vec<Self>)
} 

pub struct Pair{
    first: Packet,
    second: Packet
}

#[aoc_generator(day13)]
pub fn day13_gen(input: &str) -> Vec<Pair> {
    let mut signal: Vec<Pair> = Vec::new();
    let mut signal_iter = input.lines();
    let mut pack_map: Vec<(u32, u32)> = Vec::new();

    while let Some(packet) = signal_iter.next() {
        let mut curr_depth: u32 = 0;
        for c in packet.chars() {
            match c {
                '[' => {
                    curr_depth += 1;
                },
                ']' => {
                    curr_depth -= 1;
                },
                ',' => {},
                _ => {
                    pack_map.push((c.to_digit(10).unwrap(), curr_depth));
                }
            }
        }

        for (digit, depth) in pack_map.clone() {
            
        }

        let snd_pack = signal_iter.next().unwrap();
    }

    signal
}

#[aoc(day13, part1)]
pub fn part1(signal: &Vec<Pair>) -> u32 {
    let mut res = 0;
    

    res
}

#[aoc(day13, part2)]
pub fn part2(signal: &Vec<Pair>) -> u32 {
    let mut res = 0;
    
    res
}
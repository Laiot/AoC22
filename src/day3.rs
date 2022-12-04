use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3, part1)]
pub fn part1_gen(input: &str) -> u32 {
    let mut res = 0;

    for rucksack in input.lines() {
        let val = rucksack;
        let half_len = val.chars().count() / 2;
        let (first, second) = val.split_at(half_len);

        for char1 in first.chars() {
            if second.contains(char1){
                let mut c = char1 as u32;
                if c < 96 {
                    c = c - (65-27);
                } else {
                    c = c - 96;
                }
                res += c;
                break;
            }
        }
    }

    res
}

#[aoc_generator(day3, part2)]
pub fn part2_gen(input: &str) -> u32 {
    let mut res = 0;

    let racksacks: Vec<&str> = input.lines().collect();

    for group in racksacks.chunks(3) {
        for char1 in group[0].chars() {
            if group[1].contains(char1) && group[2].contains(char1) {
                let mut c = char1 as u32;
                    if c < 96 {
						c = c - (65-27);
					} else {
						c = c - 96;
					}
                    res += c;
                    break;
            }
        }
    }
    
    res
}

#[aoc(day3, part1)]
pub fn part1(item: &u32) -> u32 {
    *item
}

#[aoc(day3, part2)]
pub fn part2(item: &u32) -> u32 {
    *item
}
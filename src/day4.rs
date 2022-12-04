use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4, part1)]
pub fn part1_gen(input: &str) -> u32 {
    let mut res = 0;

    for line in input.lines() {
        let line_val = line;
        let mut pair_iter = line_val.split([',','-']);

        let elfa_min = pair_iter.next().unwrap().parse::<u32>().unwrap();
        let elfa_max = pair_iter.next().unwrap().parse::<u32>().unwrap();
        let elfb_min = pair_iter.next().unwrap().parse::<u32>().unwrap();
        let elfb_max = pair_iter.next().unwrap().parse::<u32>().unwrap();

        if (elfa_min <= elfb_min && elfa_max >= elfb_max) || 
            (elfb_min <= elfa_min && elfb_max >= elfa_max) {
            res += 1;
        }
    }
    res
}

#[aoc_generator(day4, part2)]
pub fn part2_gen(input: &str) -> u32 {
    let mut res = 0;

    for line in input.lines() {
        let line_val = line;
        let mut pair_iter = line_val.split([',','-']);

        let elfa_min = pair_iter.next().unwrap().parse::<u32>().unwrap();
        let elfa_max = pair_iter.next().unwrap().parse::<u32>().unwrap();
        let elfb_min = pair_iter.next().unwrap().parse::<u32>().unwrap();
        let elfb_max = pair_iter.next().unwrap().parse::<u32>().unwrap();

        if (elfa_min <= elfb_min && elfa_max >= elfb_min) || 
            (elfb_min <= elfa_min && elfb_max >= elfa_min) {
            res += 1;
        }
    }
    res
}

#[aoc(day4, part1)]
pub fn part1(overlaps: &u32) -> u32 {
    *overlaps
}

#[aoc(day4, part2)]
pub fn part2(overlaps: &u32) -> u32 {
    *overlaps
}
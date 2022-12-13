use aoc_runner_derive::aoc;

pub fn check_cycle_count(cycle_count: i32) -> bool {
    cycle_count == 20 || cycle_count == 60 || cycle_count == 100 || cycle_count == 140 || cycle_count == 180 || cycle_count == 220 
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> i32 {
    let mut signal = 0;
    let mut cycle_count = 0;
    let mut saved_signals: Vec<i32> = Vec::new();

    for command in input.lines() {
        let command_split: Vec<&str> = command.split(" ").collect();
        let cmd = command_split[0];
        
        match cmd {
            "addx" => {
                let args = command_split[1].parse::<i32>().unwrap();
                cycle_count += 1;
                if check_cycle_count(cycle_count) {
                    saved_signals.push(signal * cycle_count);
                }
                signal += args;
                cycle_count += 1;
                if check_cycle_count(cycle_count) {
                    saved_signals.push(signal * cycle_count);
                }
            },
            _ => {
                cycle_count += 1;
                if check_cycle_count(cycle_count) {
                    saved_signals.push(signal * cycle_count);
                }
            }
        }
    }
    
    let res = saved_signals.iter().fold(0, |acc: i32, x: &i32| acc + x);
    res
}

#[aoc(day10, part2)]
pub fn part2(motions: &str) -> usize {
    let res = 0;

    res
}
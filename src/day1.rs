use aoc_runner_derive::{aoc, aoc_generator};

fn get_index_of_lowest(values: &[u32]) -> usize {
    let mut lowest = u32::MAX;
    let mut index = 0;
    for i in 0..values.len() {
        if values[i] < lowest {
            lowest = values[i];
            index = i;
        }
    }
    index
}

#[aoc_generator(day1)]
pub fn day1_gen(input: &str) -> Vec<u32> {
    let mut calories: Vec<u32> = Vec::new();
    let mut curr = 0;

    for line in input.lines() {
        let value = line.parse::<u32>();

        if value.is_ok(){
            curr += value.unwrap();
        } else {
            calories.push(curr);
            curr = 0;
        }
    }

    calories.push(curr);
    calories
}

#[aoc(day1, part1)]
pub fn part1(calories: &Vec<u32>) -> u32 {
    *calories.iter().max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(calories: &Vec<u32>) -> u32 {
    let mut result = vec![u32::MIN; 3];
    for cal in calories {
        let index = get_index_of_lowest(&result);
        if cal > &result[index] {
            result[index] = *cal;
        }
    }

    result.iter().sum()
}
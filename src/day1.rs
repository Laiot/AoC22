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
    let (mut vec, last) = input.lines()
    .fold((Vec::new(), 0), |mut a, x|{
        match x.parse::<u32>() {
            Ok(x) => {(a.0, a.1+x)}
            _ => {a.0.push(a.1); (a.0, 0)}
        }
    });
    vec.push(last); 

    // .fold(vec![0], |mut a, x|{
    //     match x.parse::<u32>() {
    //         Some(v) => {*a.last_mut().unwrap() += v;}
    //         None => {a.push(0);}
    //     };
    //     a
    // })
    vec
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
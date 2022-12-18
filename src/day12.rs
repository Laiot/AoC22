use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

pub fn get_neigh(map: HashMap<(usize, usize), u32>, point: ((usize, usize), u32), visited: &mut Vec<(usize, usize)>) -> Vec<((usize, usize), u32)> {
    let mut neighs: Vec<((usize, usize), u32)> = Vec::new();
    for (coord, height) in map {
        if !point.eq(&(coord, height)) && !visited.contains(&coord) {
            if (point.0.0.abs_diff(coord.0) <= 1 && point.0.1.abs_diff(coord.1) <= 1) && point.1.abs_diff(height) <= 1{
                neighs.push((coord, height));
                visited.push(coord);
            }
        } 
    }
    neighs
}

pub fn find_path(map: HashMap<(usize, usize), u32>, point: ((usize, usize), u32), end_point: ((usize, usize), u32), step_count: usize, visited: &mut Vec<(usize, usize)>) -> usize {
    let neighs = get_neigh(map.clone(), point, visited);
    
    if neighs.contains(&end_point) {
        return step_count + 1;
    } else if neighs.is_empty() {
        return usize::MAX;
    } else {
        return neighs.into_iter().map(|n| {
            find_path(map.clone(), n, end_point, step_count + 1, visited)
        }).min().unwrap();
    }
}

#[aoc_generator(day12)]
pub fn day12_gen(input: &str) -> HashMap<(usize, usize), u32> {
    let mut map: HashMap<(usize, usize), u32> = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height: u32;
            if c == 'S' {
                height = 0;
            } else if c == 'E' {
                height = 27;
            } else {
                height = c as u32 - 96;
            }
            map.insert((x, y), height);
        }
    }
    map
}

#[aoc(day12, part1)]
pub fn part1(map: &HashMap<(usize, usize), u32>) -> usize {    
    let start_point = map.iter()
        .find_map(|(key, &val)| if val == 0 { Some(key) } else { None }).unwrap();

    let end_point = map.iter()
        .find_map(|(key, &val)| if val == 26 { Some(key) } else { None }).unwrap();

    find_path(map.clone(), (*start_point, 0), (*end_point, 26), 0, &mut Vec::new())
}

#[aoc(day12, part2)]
pub fn part2(map: &HashMap<(usize, usize), u32>) -> u32 {
    let mut res = 0;
    


    res
}
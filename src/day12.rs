use aoc_runner_derive::aoc;
use std::collections::HashMap;

pub fn get_neigh(map: HashMap<(usize, usize), u32>, point: ((usize, usize), u32)) -> Vec<((usize, usize), u32)> {
    let mut neighs: Vec<((usize, usize), u32)> = Vec::new();
    for (coord, height) in map {
        if !point.eq(&(coord, height)) {
            if (point.0.0.abs_diff(coord.0) <= 1 && point.0.1.abs_diff(coord.1) <= 1) && point.1.abs_diff(height) <= 1{
                neighs.push((coord, height));
            }
        } 
    }
    neighs
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> usize {    
    let mut res: usize = 0;
    let mut map: HashMap<(usize, usize), u32> = HashMap::new();
    let mut start_point: (usize, usize) = (0,0);
    let mut end_point: (usize, usize) = (0,0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height: u32;
            if c == 'S' {
                height = 0;
                start_point = (x, y);
            } else if c == 'E' {
                height = 27;
                end_point = (x, y);
            } else {
                height = c as u32 - 96;
            }
            map.insert((x, y), height);
        }
    }

    let mut queue: Vec<((usize, usize), usize)> = Vec::new();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();

    queue.push((start_point, 0));
    visited.insert(start_point, 0);

    while let Some(next_vert) = queue.pop() {
        if next_vert.0 == end_point {
            res = next_vert.1 + 1;
        }

        for (coord, _) in get_neigh(map.clone(), (next_vert.0, *map.get(&next_vert.0).unwrap())) {
            if !visited.contains_key(&coord) {
                queue.push((coord, next_vert.1 + 1));
                visited.insert(coord, next_vert.1 + 1);
                res += 1;
            }
        }
    }
    res
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u32 {
    let mut res = 0;
    


    res
}
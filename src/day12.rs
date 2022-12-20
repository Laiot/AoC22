use aoc_runner_derive::aoc;
use std::collections::HashMap;

pub fn get_neigh(map: HashMap<(isize, isize), u32>, point: ((isize, isize), u32)) -> Vec<((isize, isize), u32)> {
    let mut neighs: Vec<((isize, isize), u32)> = Vec::new();
    if map.get(&(point.0.0 - 1, point.0.1)).is_some() && map.get(&(point.0.0 - 1, point.0.1)).unwrap() <= &(point.1 + 1) {
        neighs.push(((point.0.0 - 1, point.0.1), *map.get(&(point.0.0 - 1, point.0.1)).unwrap()));
    } 
    if map.get(&(point.0.0 + 1, point.0.1)).is_some() && map.get(&(point.0.0 + 1, point.0.1)).unwrap() <= &(point.1 + 1) {
        neighs.push(((point.0.0 + 1, point.0.1), *map.get(&(point.0.0 + 1, point.0.1)).unwrap()));
    }
    if map.get(&(point.0.0, point.0.1 - 1)).is_some() && map.get(&(point.0.0, point.0.1 - 1)).unwrap() <= &(point.1 + 1) {
        neighs.push(((point.0.0, point.0.1 - 1), *map.get(&(point.0.0, point.0.1 - 1)).unwrap()));
    }
    if map.get(&(point.0.0, point.0.1 + 1)).is_some() && map.get(&(point.0.0, point.0.1 + 1)).unwrap() <= &(point.1 + 1) {
        neighs.push(((point.0.0, point.0.1 + 1), *map.get(&(point.0.0, point.0.1 + 1)).unwrap()));
    }
    neighs
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> isize {    
    let mut res: isize = isize::MAX;
    let mut map: HashMap<(isize, isize), u32> = HashMap::new();
    let mut start_point: (isize, isize) = (0,0);
    let mut end_point: (isize, isize) = (0,0);

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let height: u32;
            if c == 'S' {
                height = 0;
                start_point = (x.try_into().unwrap(), y.try_into().unwrap());
            } else if c == 'E' {
                height = 27;
                end_point = (x.try_into().unwrap(), y.try_into().unwrap());
            } else {
                height = c as u32 - 96;
            }
            map.insert((x.try_into().unwrap(), y.try_into().unwrap()), height);
        }
    }

    let mut queue: Vec<((isize, isize), isize)> = Vec::new();
    let mut visited: HashMap<(isize, isize), isize> = HashMap::new();

    queue.push((start_point, 0));
    visited.insert(start_point, 0);

    let mut temp_res: isize = 0;

    while let Some(next_vert) = queue.pop() {
        if next_vert.0 == end_point {
            temp_res = next_vert.1 + 1;
            if temp_res < res {
                res = temp_res;
            }
            temp_res = 0;
        }

        for (coord, _) in get_neigh(map.clone(), (next_vert.0, *map.get(&next_vert.0).unwrap())) {
            if !visited.contains_key(&coord) {
                queue.push((coord, next_vert.1 + 1));
                visited.insert(coord, next_vert.1 + 1);
                temp_res += 1;
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
use std::collections::HashSet;
use aoc_runner_derive::aoc;

pub fn tail_follow(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    match (head.0 - tail.0, head.1 - tail.1) {
        (y, x) if (x.abs() > 1) || (y.abs() > 1) => (tail.0 + y.signum(), tail.1 + x.signum()),
        _ => (tail.0, tail.1)
    }
}

#[aoc(day9, part1)]
pub fn part1(motions: &str) -> usize {
    let mut head: (isize, isize) = (0, 0);
    let mut tail: (isize, isize) = (0, 0);
    let mut visited_pos: HashSet<(isize, isize)> = HashSet::new();
    visited_pos.insert((0, 0));

    for motion in motions.lines() {
        let inputs: Vec<&str> = motion.split(" ").collect();
        let direction = inputs[0];
        let reps = inputs[1].parse().unwrap();
        
        for _ in 0..reps {
            match direction {
                "R" => {
                    head.0 += 1
                },
                "L" => {
                    head.0 -= 1
                },
                "U" => {
                    head.1 += 1
                },
                "D" => {
                    head.1 -= 1
                },
                _ => {panic!()}
            };
            tail = tail_follow(head, tail);
            visited_pos.insert(tail);
        }
    }

    visited_pos.len()
}

#[aoc(day9, part2)]
pub fn part2(motions: &str) -> usize {
    let mut knots: Vec<(isize, isize)> = vec![(0, 0); 10];
    let mut visited_pos: HashSet<(isize, isize)> = HashSet::new();
    visited_pos.insert((0, 0));

    for motion in motions.lines() {
        let inputs: Vec<&str> = motion.split(" ").collect();
        let direction = inputs[0];
        let reps = inputs[1].parse().unwrap();
        
        for _ in 0..reps {
            match direction {
                "R" => {
                    knots[0].0 += 1
                },
                "L" => {
                    knots[0].0 -= 1
                },
                "U" => {
                    knots[0].1 += 1
                },
                "D" => {
                    knots[0].1 -= 1
                },
                _ => {panic!()}
            };
            for i in 0..9 {
                knots[i+1] = tail_follow(knots[i], knots[i+1]);
            }
            visited_pos.insert(knots[9]);
        }
    }

    visited_pos.len()
}
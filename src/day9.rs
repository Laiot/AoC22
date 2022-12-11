use aoc_runner_derive::{aoc};

#[aoc(day9, part1)]
pub fn part1(motions: &str) -> usize {
    let mut head: (u32, u32) = (0, 0);
    let mut tail: (u32, u32) = (0, 0);
    let mut visited_pos: Vec<(u32, u32)> = Vec::new();
    visited_pos.push((0, 0));

    for motion in motions.lines() {
        let direction = motion.chars().nth(0).unwrap();
        let reps = motion.chars().nth(2).unwrap().to_digit(10).unwrap();
        
        match direction {
            'R' => {
                for _ in 0..reps {
                    if head.0 <= tail.0 {
                        head.0 += 1;
                    } else if head.0 - tail.0 == 1 {
                        head.0 += 1;
                        tail.0 += 1;
                        tail.1 = head.1;
                    } else {
                        head.0 += 1;
                        tail.0 += 1;
                    }
                    if !visited_pos.contains(&(tail.0, tail.1)) {
                        visited_pos.push((tail.0, tail.1));
                    }
                }
            },
            'L' => {
                for _ in 0..reps {
                    if head.0 >= tail.0 {
                        head.0 -= 1;
                    } else if tail.0 - head.0 == 1 {
                        head.0 -= 1;
                        tail.0 -= 1;
                        tail.1 = head.1;
                    } else {
                        head.0 -= 1;
                        tail.0 -= 1;
                    }
                    if !visited_pos.contains(&(tail.0, tail.1)) {
                        visited_pos.push((tail.0, tail.1));
                    }
                }
            },
            'U' => {
                for _ in 0..reps {
                    if head.1 <= tail.1 {
                        head.1 += 1;
                    } else if head.1 - tail.1 == 1 {
                        head.1 += 1;
                        tail.1 += 1;
                        tail.0 = head.0;
                    } else {
                        head.1 += 1;
                        tail.1 += 1;
                    }
                    if !visited_pos.contains(&(tail.0, tail.1)) {
                        visited_pos.push((tail.0, tail.1));
                    }
                }
            },
            'D' => {
                for _ in 0..reps {
                    if head.1 >= tail.1 {
                        head.1 -= 1;
                    } else if tail.1 - head.1 == 1 {
                        head.1 -= 1;
                        tail.1 -= 1;
                        tail.0 = head.0;
                    } else {
                        head.1 -= 1;
                        tail.1 -= 1;
                    }
                    if !visited_pos.contains(&(tail.0, tail.1)) {
                        visited_pos.push((tail.0, tail.1));
                    }
                }
            },
            _ => println!("default")
        }
    }

    visited_pos.iter().count()
}
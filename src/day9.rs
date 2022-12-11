use std::collections::HashSet;

use aoc_runner_derive::aoc;

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
        
        match direction {
            "R" => {
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
                        visited_pos.insert((tail.0, tail.1));
                    }
                }
            },
            "L" => {
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
                        visited_pos.insert((tail.0, tail.1));
                    }
                }
            },
            "U" => {
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
                        visited_pos.insert((tail.0, tail.1));
                    }
                }
            },
            "D" => {
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
                        visited_pos.insert((tail.0, tail.1));
                    }
                }
            },
            _ => println!("default")
        }
    }

    visited_pos.len()
}

#[aoc(day9, part2)]
pub fn part2(motions: &str) -> usize {
    let mut knots: Vec<(isize, isize)> = vec![(0, 0); 9];
    let mut visited_pos: HashSet<(isize, isize)> = HashSet::new();
    visited_pos.insert((0, 0));

    for motion in motions.lines() {
        let inputs: Vec<&str> = motion.split(" ").collect();
        let direction = inputs[0];
        let reps = inputs[1].parse().unwrap();

        match direction {
            "R" => {
                for _ in 0..reps {
                    for i in 0..8 {
                        if i == 0 || knots[i-1] != knots[1] {
                            if knots[i].0 <= knots[i+1].0 {
                                knots[i].0 += 1;
                            } else if knots[i].0 - knots[i+1].0 == 1 {
                                knots[i].0 += 1;
                                knots[i+1].0 += 1;
                                knots[i+1].1 = knots[i].1;
                            } else {
                                knots[i].0 += 1;
                                knots[i+1].0 += 1;
                            }
                        }  
                    }  
                    if !visited_pos.contains(&(knots[8].0, knots[8].1)) {
                        visited_pos.insert((knots[8].0, knots[8].1));
                    }
                }
            },
            "L" => {
                for _ in 0..reps {
                    for i in 0..8 {
                        if i == 0 || knots[i-1] != knots[1] {
                            if knots[i].0 >= knots[i+1].0 {
                                knots[i].0 -= 1;
                            } else if knots[i+1].0 - knots[i].0 == 1 {
                                knots[i].0 -= 1;
                                knots[i+1].0 -= 1;
                                knots[i+1].1 = knots[i].1;
                            } else {
                                knots[i].0 -= 1;
                                knots[i+1].0 -= 1;
                            }
                        }
                    }
                    if !visited_pos.contains(&(knots[8].0, knots[8].1)) {
                        visited_pos.insert((knots[8].0, knots[8].1));
                    }
                }
            },
            "U" => {
                for _ in 0..reps {
                    for i in 0..8 {
                        if i == 0 || knots[i-1] != knots[1] {
                            if knots[i].1 <= knots[i+1].1 {
                                knots[i].1 += 1;
                            } else if knots[i].1 - knots[i+1].1 == 1 {
                                knots[i].1 += 1;
                                knots[i+1].1 += 1;
                                knots[i+1].0 = knots[i].0;
                            } else {
                                knots[i].1 += 1;
                                knots[i+1].1 += 1;
                            }
                        }
                    }
                    
                    if !visited_pos.contains(&(knots[8].0, knots[8].1)) {
                        visited_pos.insert((knots[8].0, knots[8].1));
                    }
                }
            },
            "D" => {
                for _ in 0..reps {
                    for i in 0..8 {
                            if i == 0 || knots[i-1] != knots[1] {
                            if knots[i].1 >= knots[i+1].1 {
                                knots[i].1 -= 1;
                            } else if knots[i+1].1 - knots[i].1 == 1 {
                                knots[i].1 -= 1;
                                knots[i+1].1 -= 1;
                                knots[i+1].0 = knots[i].0;
                            } else {
                                knots[i].1 -= 1;
                                knots[i+1].1 -= 1;
                            }
                        }
                    }
                    if !visited_pos.contains(&(knots[8].0, knots[8].1)) {
                        visited_pos.insert((knots[8].0, knots[8].1));
                    }
                }
            },
            _ => println!("default")
        }
    }

    visited_pos.len()
}
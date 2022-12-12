use std::collections::HashSet;
use aoc_runner_derive::aoc;

pub fn tail_follow(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    match (head.0 - tail.0, head.1 - tail.1) {
        (y, x) if (x.abs() > 1) || (y.abs() > 1) => (tail.0 + y.signum(), tail.1 + x.signum()),
        _ => (0,0)
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
        
        match direction {
            "R" => {
                for _ in 0..reps {
                    head.0 += 1;
                    tail = tail_follow(head, tail);
                    if !visited_pos.contains(&(tail.0, tail.1)) {
                        visited_pos.insert((tail.0, tail.1));
                    }
                }
            },
            "L" => {
                for _ in 0..reps {
                    head.0 -= 1;
                    tail = tail_follow(head, tail);
                    if !visited_pos.contains(&(tail.0, tail.1)) {
                        visited_pos.insert((tail.0, tail.1));
                    }
                }
            },
            "U" => {
                for _ in 0..reps {
                    head.1 += 1;
                    tail = tail_follow(head, tail);
                    if !visited_pos.contains(&(tail.0, tail.1)) {
                        visited_pos.insert((tail.0, tail.1));
                    }
                }
            },
            "D" => {
                for _ in 0..reps {
                    head.1 -= 1;
                    tail = tail_follow(head, tail);
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
    let mut knots: Vec<(isize, isize)> = vec![(0, 0); 10];
    let mut visited_pos: HashSet<(isize, isize)> = HashSet::new();
    visited_pos.insert((0, 0));

    for motion in motions.lines() {
        let inputs: Vec<&str> = motion.split(" ").collect();
        let direction = inputs[0];
        let reps = inputs[1].parse().unwrap();

        match direction {
            "R" => {
                for _ in 0..reps {
                    for i in 0..9 {
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
                    if !visited_pos.contains(&(knots[9].0, knots[9].1)) {
                        visited_pos.insert((knots[9].0, knots[9].1));
                    }
                }
            },
            "L" => {
                for _ in 0..reps {
                    for i in 0..9 {
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
                    if !visited_pos.contains(&(knots[9].0, knots[9].1)) {
                        visited_pos.insert((knots[9].0, knots[9].1));
                    }
                }
            },
            "U" => {
                for _ in 0..reps {
                    for i in 0..9 {
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
                    
                    if !visited_pos.contains(&(knots[9].0, knots[9].1)) {
                        visited_pos.insert((knots[9].0, knots[9].1));
                    }
                }
            },
            "D" => {
                for _ in 0..reps {
                    for i in 0..9 {
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
                    if !visited_pos.contains(&(knots[9].0, knots[9].1)) {
                        visited_pos.insert((knots[9].0, knots[9].1));
                    }
                }
            },
            _ => println!("default")
        }
    }

    visited_pos.len()
}
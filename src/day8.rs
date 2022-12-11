use std::cmp::min;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone)]
pub struct Tree {
    height: u32,
    pos_x: usize,
    pos_y: usize
}

#[aoc_generator(day8)]
pub fn day8_gen(input: &str) -> Vec<Tree> {
    let mut trees: Vec<Tree> = Vec::new();
    

    for (r_index, row) in input.lines().enumerate() {
        for (c_index, tree) in row.chars().enumerate() {
            trees.push(Tree{height: tree.to_digit(10).unwrap(), pos_x: c_index, pos_y: r_index});
        }
    }

    trees
}

#[aoc(day8, part1)]
pub fn part1(trees: &Vec<Tree>) -> usize {

    let row_len = 99;
    let edge_counter = (row_len * 4) - 4;
    let mut visible_trees: Vec<Tree> = Vec::new();
    let mut left: bool;
    let mut right: bool;
    let mut up: bool;
    let mut down: bool;

    for tree in trees {
        if tree.pos_x > 0 && tree.pos_x < row_len - 1 && tree.pos_y > 0 && tree.pos_y < row_len - 1 {
            left = true;
            right = true;
            up = true;
            down = true;
            for test_tree in trees {
                if test_tree.pos_x == tree.pos_x && test_tree.pos_y > tree.pos_y {
                    if test_tree.height >= tree.height {
                        down = false;
                    }
                } else if test_tree.pos_x == tree.pos_x && test_tree.pos_y < tree.pos_y {
                    if test_tree.height >= tree.height {
                        up = false;
                    }
                } else if test_tree.pos_y == tree.pos_y && test_tree.pos_x < tree.pos_x {
                    if test_tree.height >= tree.height {
                        right = false;
                    }
                } else if test_tree.pos_y == tree.pos_y && test_tree.pos_x > tree.pos_x {
                    if test_tree.height >= tree.height {
                        left = false;
                    }
                }
            }
            if down || up || right || left {
                visible_trees.push(*tree);
            }
        }
    }

    visible_trees.iter().count() + edge_counter
}

#[aoc(day8, part2)]
pub fn part2(trees: &Vec<Tree>) -> usize {
    let mut res = 0;
    let row_len = 99;

    for tree in trees {
        if tree.pos_x > 0 && tree.pos_x < row_len - 1 && tree.pos_y > 0 && tree.pos_y < row_len - 1 {
            let (mut up, mut down, mut left, mut right) = (tree.pos_y, 98 - tree.pos_y, tree.pos_x, 98 - tree.pos_x);
            for test_tree in trees {
                if test_tree.pos_x == tree.pos_x && test_tree.pos_y > tree.pos_y {
                    if test_tree.height >= tree.height {
                        down = min(down, test_tree.pos_y - tree.pos_y);
                    }
                } else if test_tree.pos_x == tree.pos_x && test_tree.pos_y < tree.pos_y {
                    if test_tree.height >= tree.height {
                        up = min(up, tree.pos_y - test_tree.pos_y);
                    }
                } else if test_tree.pos_y == tree.pos_y && test_tree.pos_x < tree.pos_x {
                    if test_tree.height >= tree.height {
                        right = min(right, tree.pos_x - test_tree.pos_x);
                    }
                } else if test_tree.pos_y == tree.pos_y && test_tree.pos_x > tree.pos_x {
                    if test_tree.height >= tree.height {
                        left = min(left, test_tree.pos_x - tree.pos_x);
                    }
                }
            }
            let new_max = down * up * right * left;
            if new_max > res {
                res = new_max;
            }
        }
    }

    res
}
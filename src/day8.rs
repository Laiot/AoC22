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

    let row_len = 5;
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
pub fn part2(trees: &Vec<Tree>) -> u32 {
    let mut res = 0;
    let row_len = 5;
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

    for visible_tree in visible_trees {
        let mut count = 0;
        let mut top = 1;
        for a in (0..visible_tree.pos_x).rev() {
            if trees.iter().find(|tree| tree.pos_y == visible_tree.pos_y && tree.pos_x == a && tree.height < visible_tree.height).is_some(){
                count += 1;
            } else {
                if count == 0 {count = 1}
                top *= count;
                count = 0;
                break;
            };
        }
        if count == 0 {count = 1}
        top *= count;
        count = 0;
        for b in visible_tree.pos_x..5 {
            if trees.iter().find(|tree| tree.pos_y == visible_tree.pos_y && tree.pos_x == b && tree.height < visible_tree.height).is_some(){
                count += 1;
            } else {
                if count == 0 {count = 1}

                top *= count;
                count = 0;
                break;
            };
        }
        if count == 0 {count = 1}
        top *= count;
        count = 0;
        for c in (0..visible_tree.pos_y).rev() {
            if trees.iter().find(|tree| tree.pos_y == c && tree.pos_x == visible_tree.pos_x && tree.height < visible_tree.height).is_some(){
                count += 1;
            } else {
                if count == 0 {count = 1}

                top *= count;
                count = 0;
                break;
            };
        }
        if count == 0 {count = 1}
        top *= count;
        count = 0;
        for d in visible_tree.pos_y..5 {
            if trees.iter().find(|tree| tree.pos_y == d && tree.pos_x == visible_tree.pos_x && tree.height < visible_tree.height).is_some(){
                count += 1;
            } else {
                if count == 0 {count = 1}

                top *= count;
                count = 0;
                break;
            };
        }
        if count == 0 {count = 1}
        top *= count;

        if res < top {
            res = top;
        }
    }

    res
}
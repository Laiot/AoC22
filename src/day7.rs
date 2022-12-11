use aoc_runner_derive::{aoc, aoc_generator};
#[derive(Debug, PartialEq)]
pub struct Dir {
    subdirs: Vec<Dir>,
    size: u32
}

impl Dir {
    pub fn new() -> Dir{
        Dir {subdirs: Vec::new(), size: 0}
    }
}

fn get_dir_size(dir: Dir) -> u32 {
    let mut res = 0;

    // for file in dir.files{
    //     res += file.0;
    // }

    // for subdir in dir.subdirs{
    //     get_dir_size(subdir);
    // }

    if res < 100000 {
        0
    } else {
        res
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> u32 {
    let mut res = 0;
    // let mut dir_path: Vec<Dir> = Vec::new();
    // let mut dir_tree: Vec<Dir> = Vec::new();

    // let mut lines = input.lines().peekable();

    // while lines.peek() != None {
    //     let line: Vec<&str> = lines.next().unwrap().split(" ").collect();

    //     if line[1] == "cd" {
    //         if line[2] == ".." {
    //             dir_path.pop();
    //         } else {
    //             dir_path.push(Dir::new());
    //             //dir_tree.push(Dir::new());
    //         }
    //     } else if line[1] == "ls" {
    //         while lines.peek() != None {
    //             let subline_peek: Vec<&str> = lines.peek().unwrap().split(" ").collect();

    //             if subline_peek[0] == "$" {
    //                 break;
    //             }

    //             let subline: Vec<&str> = lines.next().unwrap().split(" ").collect();

    //             if subline[0] == "dir" {
    //                 // dir_path.last_mut().unwrap().subdirs.push(Dir::new());
    //                 // dir_tree.last_mut().unwrap().subdirs.push(Dir::new());
    //                 break;
    //             } else {
    //                 for mut dir in dir_path {
    //                     if dir_path.contains(&dir){
    //                         dir.size += subline[0].parse::<u32>().unwrap();

    //                     }
    //                 }
    //                 dir_path.last_mut().unwrap().size += subline[0].parse::<u32>().unwrap();
    //                 dir_tree.last_mut().unwrap().size += subline[0].parse::<u32>().unwrap();
    //             }
    //         }
    //     }
    // }
   
    // println!("{:?}", dir_tree);

    // dir_tree.into_iter().map(|dir| {
    //     get_dir_size(dir)
    // }).fold(0, |acc, dir| {
    //     acc + dir
    // })
    res
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> u32 {
    let mut res = 0;

    res
}
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn day2_gen(input: &str) -> Vec<(char, char)> {
    let mut games: Vec<(char, char)> = Vec::new();

    for line in input.lines() {
        let string_game = line;

        games.push((string_game.chars().nth(0).unwrap(), string_game.chars().nth(2).unwrap()));
    }

    games
}

#[aoc(day2, part1)]
pub fn part1(games: &Vec<(char, char)>) -> u32 {
    let mut res = 0;
    
    for game in games {
        match game {
            ('A', 'X') => res += 4,
            ('A', 'Y') => res += 8,
            ('A', 'Z') => res += 3,
            ('B', 'X') => res += 1,
            ('B', 'Y') => res += 5,
            ('B', 'Z') => res += 9,
            ('C', 'X') => res += 7,
            ('C', 'Y') => res += 2,
            ('C', 'Z') => res += 6,
            _ => println!("error") 
        }
    }

    res
}

#[aoc(day2, part2)]
pub fn part2(games: &Vec<(char, char)>) -> u32 {
    let mut res = 0;
    
    for game in games {
        match game {
            ('A', 'X') => res += 3,
            ('A', 'Y') => res += 4,
            ('A', 'Z') => res += 8,
            ('B', 'X') => res += 1,
            ('B', 'Y') => res += 5,
            ('B', 'Z') => res += 9,
            ('C', 'X') => res += 2,
            ('C', 'Y') => res += 6,
            ('C', 'Z') => res += 7,
            _ => println!("error") 
        }
    }

    res
}
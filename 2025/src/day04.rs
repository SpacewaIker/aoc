use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

fn count_adjacent_rolls(grid: &[Vec<char>], x: usize, y: usize) -> u32 {
    let mut count = 0;
    if y > 0 {
        if grid[y - 1][x] == '@' {
            count += 1;
        }
        if x > 0 && grid[y - 1][x - 1] == '@' {
            count += 1;
        }
        if x < grid[0].len() - 1 && grid[y - 1][x + 1] == '@' {
            count += 1;
        }
    }
    if y < grid.len() - 1 {
        if grid[y + 1][x] == '@' {
            count += 1;
        }
        if x > 0 && grid[y + 1][x - 1] == '@' {
            count += 1;
        }
        if x < grid[0].len() - 1 && grid[y + 1][x + 1] == '@' {
            count += 1;
        }
    }
    if x > 0 && grid[y][x - 1] == '@' {
        count += 1;
    }
    if x < grid[0].len() - 1 && grid[y][x + 1] == '@' {
        count += 1;
    }
    count
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[aoc(day4, part1, simple)]
fn solve_part1_simple(input: &[Vec<char>]) -> usize {
    (0..input.len())
        .cartesian_product(0..input[0].len())
        .filter(|&(x, y)| input[y][x] == '@' && count_adjacent_rolls(input, x, y) < 4)
        .count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Vec<char>]) -> u32 {
    let mut input = input.to_owned();
    let mut rolls = Vec::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == '@' {
                rolls.push((x, y));
            }
        }
    }
    let mut count = 0;
    loop {
        let mut idx = 0;
        let mut removed = false;
        while idx < rolls.len() {
            let (x, y) = rolls[idx];
            let neighbors = count_adjacent_rolls(&input, x, y);
            if neighbors < 4 {
                input[y][x] = '.';
                rolls.remove(idx);
                count += 1;
                removed = true;
            } else {
                idx += 1;
            }
        }
        if !removed {
            break;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4_part1_simple() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let parsed = input_generator(input);
        assert_eq!(solve_part1_simple(&parsed), 13);
    }

    #[test]
    fn test_day4_part2() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let parsed = input_generator(input);
        assert_eq!(solve_part2(&parsed), 43);
    }
}

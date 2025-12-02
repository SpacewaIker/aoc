use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| {
            (if line.starts_with('L') { -1 } else { 1 }) * line[1..].parse::<i32>().unwrap()
        })
        .collect()
}

#[aoc(day1, part1, decl)]
fn solve_part1(input: &[i32]) -> u32 {
    let mut position = 50;
    let mut count = 0;
    for &val in input {
        position = (position + val).rem_euclid(100);
        if position == 0 {
            count += 1;
        }
    }
    count
}

#[aoc(day1, part1, iterator)]
fn solve_part1_it(input: &[i32]) -> u32 {
    input
        .iter()
        .fold((50, 0), |(mut position, mut count), &val| {
            position = (position + val).rem_euclid(100);
            count += u32::from(position == 0);
            (position, count)
        })
        .1
}

#[aoc(day1, part2, smart)]
fn solve_part2(input: &[i32]) -> u32 {
    let mut position = 50;
    let mut count = 0;
    for &val in input {
        count += val.unsigned_abs() / 100;
        let new = position + val % 100;
        count += u32::from(position != 0 && (new <= 0 || new >= 100));
        position = new.rem_euclid(100);
    }
    count
}

#[aoc(day1, part2, bruteforce)]
fn solve_part2_bf(input: &[i32]) -> u32 {
    let mut position = 50;
    let mut count = 0;
    for &val in input {
        for _ in 0..val.unsigned_abs() {
            position = (position + val.signum()) % 100;

            if position == 0 {
                count += 1;
            }
        }
    }
    count
}

#[aoc(day1, part2, iterator)]
fn solve_part2_it(input: &[i32]) -> u32 {
    input
        .iter()
        .fold((50, 0), |(mut position, mut count), &val| {
            count += val.unsigned_abs() / 100;
            let new = position + val % 100;
            count += u32::from(position != 0 && (new <= 0 || new >= 100));
            position = new.rem_euclid(100);
            (position, count)
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1_part1() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(solve_part1(&input_generator(input)), 3);
    }

    #[test]
    fn test_day1_part2() {
        let input = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(solve_part2(&input_generator(input)), 6);
    }

    #[test]
    fn test_day1_part2_2() {
        let input = "R1000";
        assert_eq!(solve_part2(&input_generator(input)), 10);
    }

    #[test]
    fn test_day1_part2_long() {
        let input = "R100000000
R100000000
R100000000
R100000000";
        assert_eq!(solve_part2_bf(&input_generator(input)), 4_000_000);
        assert_eq!(solve_part2(&input_generator(input)), 4_000_000);
    }

    #[test]
    fn test_day1_part2_3() {
        let input = "L50
L100";
        let parsed = input_generator(input);
        assert_eq!(solve_part2(&parsed), solve_part2_bf(&parsed));
    }
}

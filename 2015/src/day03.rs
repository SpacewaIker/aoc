use std::collections::HashSet;

use aoc_runner_derive::aoc;

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> usize {
    let (mut x, mut y) = (0, 0);
    let mut houses = HashSet::from([(0, 0)]);

    for c in input.chars() {
        let movement = match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => panic!("unknown character"),
        };
        x += movement.0;
        y += movement.1;
        houses.insert((x, y));
    }

    houses.len()
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> usize {
    let mut santa = (0, 0);
    let mut robo_santa = (0, 0);
    let mut houses = HashSet::from([(0, 0)]);
    let mut turn = true;

    for c in input.chars() {
        let movement = match c {
            '^' => (0, 1),
            'v' => (0, -1),
            '>' => (1, 0),
            '<' => (-1, 0),
            _ => panic!("unknown character"),
        };
        if turn {
            santa.0 += movement.0;
            santa.1 += movement.1;
            houses.insert(santa);
        } else {
            robo_santa.0 += movement.0;
            robo_santa.1 += movement.1;
            houses.insert(robo_santa);
        }
        turn = !turn;
    }

    houses.len()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(">", 2)]
    #[case("^>v<", 4)]
    #[case("<><><><><><", 2)]
    fn test_day3_part1(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part1(input), expected);
    }

    #[rstest]
    #[case("^v", 3)]
    #[case("^>v<", 3)]
    #[case("<><><><><><", 12)]
    fn test_day3_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part2(input), expected);
    }
}

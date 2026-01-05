use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn solve_part1(input: &str) -> i32 {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("unknown character"),
        })
        .sum()
}

#[aoc(day1, part2)]
fn solve_part2(input: &str) -> usize {
    let mut floor = 0;
    for (idx, c) in input.chars().enumerate() {
        floor += match c {
            '(' => 1,
            ')' => -1,
            _ => panic!("unknown character"),
        };
        if floor == -1 {
            return idx + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("(())", 0)]
    #[case("()()", 0)]
    #[case("(((", 3)]
    #[case("(()(()(", 3)]
    #[case("))(((((", 3)]
    #[case("())", -1)]
    #[case("))(", -1)]
    #[case(")))", -3)]
    #[case(")())())", -3)]
    fn test_day1_part1(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(solve_part1(input), expected);
    }

    #[rstest]
    #[case(")", 1)]
    #[case("()())", 5)]
    fn test_day1_part2(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(solve_part2(input), expected);
    }
}

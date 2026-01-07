use aoc_runner_derive::aoc;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[aoc(day4, part1)]
fn solve_part1(input: &str) -> u32 {
    (0..10_000_000)
        .into_par_iter()
        .find_first(|number| {
            let string = input.to_string() + &number.to_string();
            let digest = md5::compute(string);
            format!("{digest:?}").starts_with("00000")
        })
        .unwrap_or(0)
}

#[aoc(day4, part2)]
fn solve_part2(input: &str) -> u32 {
    (0..10_000_000)
        .into_par_iter()
        .find_first(|number| {
            let string = input.to_string() + &number.to_string();
            let digest = md5::compute(string);
            format!("{digest:?}").starts_with("000000")
        })
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("abcdef", 609_043)]
    #[case("pqrstuv", 1_048_970)]
    fn test_day4_part1(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(solve_part1(input), expected);
    }
}

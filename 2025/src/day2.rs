use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::num_traits::Euclid;
use winnow::{
    Parser, Result,
    ascii::dec_uint,
    combinator::{separated, separated_pair},
};

fn range(input: &mut &str) -> Result<(u64, u64)> {
    separated_pair(dec_uint, '-', dec_uint).parse_next(input)
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<(u64, u64)> {
    let mut parser = separated(1.., range, ',');

    parser.parse(input).expect("Could not parse input")
}

#[aoc(day2, part1)]
fn solve_part1(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .map(|&(start, end)| (start..=end).filter(|&id| !is_valid_id_1(id)).sum::<u64>())
        .sum()
}

fn is_valid_id_1(id: u64) -> bool {
    let digits = 1 + id.ilog10();
    if digits % 2 == 0 {
        let divider = 10u64.pow(digits / 2);
        let (first, second) = id.div_rem_euclid(&divider);
        first != second
    } else {
        true
    }
}

fn is_valid_id_2(id: u64) -> bool {
    let id = id.to_string();
    if id.len() == 1 {
        return true;
    }
    let mut dividers = Vec::new();
    for divider in 2..=(id.len() / 2).max(2) {
        if id.len() % divider == 0 {
            dividers.push(divider);
        }
    }
    dividers.push(id.len());

    let invalid = dividers.into_iter().any(|divider| {
        let subs_len = id.len() / divider;
        let substring = &id[0..subs_len];
        (1..divider).all(|slice_idx| {
            let start = slice_idx * subs_len;
            *substring == id[start..start + subs_len]
        })
    });
    !invalid
}

#[aoc(day2, part2)]
fn solve_part2(input: &[(u64, u64)]) -> u64 {
    input
        .iter()
        .map(|&(start, end)| (start..=end).filter(|&id| !is_valid_id_2(id)).sum::<u64>())
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(11, false)]
    #[case(123, true)]
    #[case(111, true)]
    #[case(1212, false)]
    #[case(1000, true)]
    #[case(1010, false)]
    fn test_day2_is_valid_id_1(#[case] id: u64, #[case] valid: bool) {
        assert_eq!(is_valid_id_1(id), valid);
    }

    #[rstest]
    #[case(11, false)]
    #[case(123, true)]
    #[case(111, false)]
    #[case(1212, false)]
    #[case(1000, true)]
    #[case(1010, false)]
    #[case(123_123_123, false)]
    #[case(12_12_12_12_12, false)]
    #[case(12121, true)]
    fn test_day2_is_valid_id_2(#[case] id: u64, #[case] valid: bool) {
        assert_eq!(is_valid_id_2(id), valid);
    }

    #[test]
    fn test_day2_part1() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let parsed = input_generator(input);
        assert_eq!(solve_part1(&parsed), 1_227_775_554);
    }

    #[test]
    fn test_day2_part2() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let parsed = input_generator(input);
        assert_eq!(solve_part1(&parsed), 44_487_518_055);
    }
}

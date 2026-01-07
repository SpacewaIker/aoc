use std::collections::HashMap;

use aoc_runner_derive::aoc;

fn is_nice_part1(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();

    let mut vowels = u32::from(['a', 'e', 'i', 'o', 'u'].contains(&chars[0]));
    let mut doubles = false;

    for i in 0..chars.len() - 1 {
        let c1 = chars[i];
        let c2 = chars[i + 1];

        if [('a', 'b'), ('c', 'd'), ('p', 'q'), ('x', 'y')].contains(&(c1, c2)) {
            return false;
        }
        doubles |= c1 == c2;
        vowels += u32::from(['a', 'e', 'i', 'o', 'u'].contains(&c2));
    }

    vowels >= 3 && doubles
}

#[aoc(day5, part1)]
fn solve_part1(input: &str) -> usize {
    input.lines().filter(|s| is_nice_part1(s)).count()
}

fn is_nice_part2(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>();

    let mut pairs = HashMap::new();
    let mut letters = HashMap::from([(chars[0], 0)]);

    let mut pair = false;
    let mut letter = false;

    for i in 0..chars.len() - 1 {
        let c1 = chars[i];
        let c2 = chars[i + 1];

        if !pair {
            if let Some(&j) = pairs.get(&(c1, c2)) {
                pair |= j < i - 1;
            } else {
                pairs.insert((c1, c2), i);
            }
        }

        if !letter {
            if let Some(&j) = letters.get(&c2) {
                letter |= i - j == 1;
            }
            letters.insert(c2, i + 1);
        }

        if letter && pair {
            return true;
        }
    }

    pair && letter
}

#[aoc(day5, part2)]
fn solve_part2(input: &str) -> usize {
    input.lines().filter(|s| is_nice_part2(s)).count()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("ugknbfddgicrmopn", true)]
    #[case("aaa", true)]
    #[case("jchzalrnumimnmhp", false)]
    #[case("haegwjzuvuyypxyu", false)]
    #[case("dvszwmarrgswjxmb", false)]
    fn test_day5_is_nice_part1(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_nice_part1(input), expected);
    }

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", true)]
    #[case("xxyxx", true)]
    #[case("uurcxstgmygtbstg", false)]
    #[case("ieodomkazucvgmuy", false)]
    fn test_day5_is_nice_part2(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(is_nice_part2(input), expected);
    }
}

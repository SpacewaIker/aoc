use aoc_runner_derive::aoc;

fn first_max(input: &str) -> (usize, char) {
    let mut max_idx = 0;
    let mut max = char::MIN;
    for (idx, c) in input.chars().enumerate() {
        if c > max {
            max = c;
            max_idx = idx;
        }
    }
    (max_idx, max)
}

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (idx, first) = first_max(&line[0..line.len() - 1]);
            let (_, second) = first_max(&line[idx + 1..]);
            10 * first.to_digit(10).unwrap() + second.to_digit(10).unwrap()
        })
        .sum()
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut value = 0;
            let mut start = usize::MAX; // wrap around on purpose

            for step in 0..12 {
                start = start.wrapping_add(1);
                let (idx, digit) = first_max(&line[start..line.len() - 11 + step]);
                start += idx;
                value += 10u64.pow(11 - step as u32) * u64::from(digit.to_digit(10).unwrap());
            }
            value
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day3_part1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve_part1(input), 357);
    }

    #[test]
    fn test_day3_part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";
        assert_eq!(solve_part2(input), 3_121_910_778_619);
    }
}

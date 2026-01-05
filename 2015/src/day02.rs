use aoc_runner_derive::{aoc, aoc_generator};

type ParsedInput = Vec<(u32, u32, u32)>;

#[aoc_generator(day2)]
fn input_generator(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            let mut spl = line.split('x');
            (
                spl.next().unwrap().parse().unwrap(),
                spl.next().unwrap().parse().unwrap(),
                spl.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &ParsedInput) -> u32 {
    input
        .iter()
        .map(|(l, w, h)| {
            let area1 = l * w;
            let area2 = w * h;
            let area3 = l * h;
            let min = area1.min(area2).min(area3);
            2 * (area1 + area2 + area3) + min
        })
        .sum()
}

#[aoc(day2, part2)]
fn solve_part2(input: &ParsedInput) -> u32 {
    input
        .iter()
        .map(|(l, w, h)| {
            let max = l.max(w).max(h);
            2 * (l + w + h - max) + l * w * h
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2_part1() {
        let input = "2x3x4
1x1x10";
        let parsed = input_generator(input);
        assert_eq!(solve_part1(&parsed), 101);
    }

    #[test]
    fn test_day2_part2() {
        let input = "2x3x4
1x1x10";
        let parsed = input_generator(input);
        assert_eq!(solve_part2(&parsed), 48);
    }
}

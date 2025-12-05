use std::{
    iter::Sum,
    ops::{Add, RangeInclusive, Sub},
};

use aoc_runner_derive::{aoc, aoc_generator};
use winnow::{
    Parser, Result,
    ascii::{dec_uint, line_ending},
    combinator::{separated, separated_pair},
};

struct ListOfRanges<T> {
    ranges: Vec<RangeInclusive<T>>,
}

fn disjoint<T: PartialOrd>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> bool {
    r1.end() < r2.start() || r1.start() > r2.end()
}

fn merge<T: Ord + Copy>(r1: &RangeInclusive<T>, r2: &RangeInclusive<T>) -> RangeInclusive<T> {
    let start = *r1.start().min(r2.start());
    let end = *r1.end().max(r2.end());
    start..=end
}

impl<T> ListOfRanges<T>
where
    T: Ord + Copy + Sub<Output = T> + Add<Output = T> + Default + Sum + From<u8>,
{
    fn add(&mut self, range: &RangeInclusive<T>) {
        let mut idx = 0;
        while idx < self.ranges.len() {
            let existing = &self.ranges[idx];

            if !disjoint(range, existing) {
                self.ranges[idx] = merge(range, existing);

                if idx < self.ranges.len() - 1
                    && !disjoint(&self.ranges[idx], &self.ranges[idx + 1])
                {
                    self.ranges[idx] = merge(&self.ranges[idx], &self.ranges[idx + 1]);
                    self.ranges.remove(idx + 1);
                }
                return;
            }
            if range.start() < existing.start() {
                self.ranges.insert(idx, range.clone());
                return;
            }

            idx += 1;
        }
        self.ranges.push(range.clone());
    }

    fn contains(&self, val: T) -> bool {
        for range in &self.ranges {
            if range.contains(&val) {
                return true;
            }
        }
        false
    }

    fn count_contained_values(&self) -> T {
        self.ranges
            .iter()
            .map(|range| T::from(1) + *range.end() - *range.start())
            .sum::<T>()
    }
}

fn parse_range(input: &mut &str) -> Result<(u64, u64)> {
    separated_pair(dec_uint, '-', dec_uint).parse_next(input)
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> (ListOfRanges<u64>, Vec<u64>) {
    let (ranges, ids): (Vec<(u64, u64)>, Vec<u64>) = separated_pair(
        separated(1.., parse_range, line_ending),
        (line_ending, line_ending),
        separated(1.., dec_uint::<_, u64, _>, line_ending),
    )
    .parse(input)
    .unwrap();

    let mut lor = ListOfRanges { ranges: Vec::new() };
    for (start, end) in ranges {
        lor.add(&(start..=end));
    }

    (lor, ids)
}

#[aoc(day5, part1)]
fn solve_part1((lor, ids): &(ListOfRanges<u64>, Vec<u64>)) -> usize {
    ids.iter().filter(|&&id| lor.contains(id)).count()
}

#[aoc(day5, part2)]
fn solve_part2((lor, _): &(ListOfRanges<u64>, Vec<u64>)) -> u64 {
    lor.count_contained_values()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5_part1() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let parsed = input_generator(input);
        assert_eq!(solve_part1(&parsed), 3);
    }

    #[test]
    fn test_day5_part2() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        let parsed = input_generator(input);
        assert_eq!(solve_part2(&parsed), 14);
    }
}

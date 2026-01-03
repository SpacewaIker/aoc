use aoc_runner_derive::{aoc, aoc_generator};

struct Grid {
    width: usize,
    height: usize,
    gifts: Vec<usize>,
}

type ParsedInput = Vec<Grid>;

#[aoc_generator(day12)]
fn input_generator(input: &str) -> ParsedInput {
    input
        .lines()
        .filter_map(|line| {
            if line.contains('x') {
                let mut spl = line.split(':');
                let mut size = spl.next().unwrap().split('x');
                let width = size.next().unwrap().parse().unwrap();
                let height = size.next().unwrap().parse().unwrap();
                let gifts = spl
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .filter_map(|s| {
                        if s.is_empty() {
                            None
                        } else {
                            Some(s.parse().unwrap())
                        }
                    })
                    .collect();
                Some(Grid {
                    width,
                    height,
                    gifts,
                })
            } else {
                None
            }
        })
        .collect()
}

#[aoc(day12, part1, stupid)]
fn solve_part1_stupid(input: &ParsedInput) -> usize {
    input
        .iter()
        .filter(|&grid| {
            let req = grid.gifts.iter().sum::<usize>() * 9;
            let actual = grid.width * grid.height;
            actual >= req
        })
        .count()
}

#[aoc(day12, part1, full)]
fn solve_part1(input: &ParsedInput) -> usize {
    todo!()
}

#[aoc(day12, part2)]
const fn solve_part2(_input: &ParsedInput) -> u32 {
    0 // no part 2!
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12_part1() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";
        let parsed = input_generator(input);
        assert_eq!(solve_part1(&parsed), 2);
    }
}

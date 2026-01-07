use aoc_runner_derive::{aoc, aoc_generator};
use euclid::default::{Box2D, Point2D};
use winnow::{
    Parser, Result,
    ascii::{dec_uint, newline, space1},
    combinator::{alt, separated, separated_pair},
};

#[derive(Debug, PartialEq)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

impl Instruction {
    const fn apply_part1(&self, input: bool) -> bool {
        match self {
            Self::TurnOn => true,
            Self::TurnOff => false,
            Self::Toggle => !input,
        }
    }

    const fn apply_part2(&self, input: u32) -> u32 {
        match self {
            Self::TurnOn => input + 1,
            Self::TurnOff => input.saturating_sub(1),
            Self::Toggle => input + 2,
        }
    }
}

type ParsedInput = Vec<(Instruction, Box2D<usize>)>;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> ParsedInput {
    fn instruction(input: &mut &str) -> Result<Instruction> {
        alt(("turn on", "turn off", "toggle"))
            .parse_next(input)
            .map(|str| match str {
                "turn on" => Instruction::TurnOn,
                "turn off" => Instruction::TurnOff,
                "toggle" => Instruction::Toggle,
                _ => unreachable!(),
            })
    }

    fn coord(input: &mut &str) -> Result<Point2D<usize>> {
        separated_pair(dec_uint, ',', dec_uint)
            .parse_next(input)
            .map(|(x, y)| Point2D::new(x, y))
    }

    fn rect(input: &mut &str) -> Result<Box2D<usize>> {
        separated_pair(coord, (space1, "through", space1), coord)
            .parse_next(input)
            .map(|(p1, p2)| Box2D::new(p1, p2))
    }

    separated(1.., separated_pair(instruction, space1, rect), newline)
        .parse(input)
        .unwrap()
}

#[aoc(day6, part1)]
#[allow(clippy::needless_range_loop)]
fn solve_part1(input: &ParsedInput) -> usize {
    let mut lights = vec![vec![false; 1000]; 1000];

    for (instruction, rect) in input {
        for y in rect.min.y..=rect.max.y {
            for x in rect.min.x..=rect.max.x {
                lights[y][x] = instruction.apply_part1(lights[y][x]);
            }
        }
    }

    lights.into_iter().flatten().filter(|b| *b).count()
}

#[aoc(day6, part2)]
#[allow(clippy::needless_range_loop)]
fn solve_part2(input: &ParsedInput) -> u32 {
    let mut lights = vec![vec![0; 1000]; 1000];

    for (instruction, rect) in input {
        for y in rect.min.y..=rect.max.y {
            for x in rect.min.x..=rect.max.x {
                lights[y][x] = instruction.apply_part2(lights[y][x]);
            }
        }
    }

    lights.into_iter().flatten().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_parser() {
        let input = "turn on 0,0 through 999,999
toggle 0,0 through 999,0
turn off 499,499 through 500,500";
        let expected = vec![
            (
                Instruction::TurnOn,
                Box2D::new(Point2D::new(0, 0), Point2D::new(999, 999)),
            ),
            (
                Instruction::Toggle,
                Box2D::new(Point2D::new(0, 0), Point2D::new(999, 0)),
            ),
            (
                Instruction::TurnOff,
                Box2D::new(Point2D::new(499, 499), Point2D::new(500, 500)),
            ),
        ];

        assert_eq!(input_generator(input), expected);
    }
}

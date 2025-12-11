use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use winnow::{
    Parser,
    ascii::{dec_uint, space0},
    combinator::{delimited, repeat, separated, terminated},
    token::one_of,
};

#[derive(Debug)]
struct Machine {
    target_lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u32>,
}

impl Machine {
    fn parse(input: &str) -> winnow::Result<Self> {
        fn lights(input: &mut &str) -> winnow::Result<Vec<bool>> {
            delimited('[', repeat(1.., one_of(['.', '#'])), ']')
                .parse_next(input)
                .map(|it: Vec<char>| it.iter().map(|c| *c == '#').collect())
        }

        fn button(input: &mut &str) -> winnow::Result<Vec<usize>> {
            terminated(
                delimited(
                    '(',
                    separated(1.., dec_uint::<_, usize, _>, (',', space0)),
                    ')',
                ),
                space0,
            )
            .parse_next(input)
        }

        fn joltage(input: &mut &str) -> winnow::Result<Vec<u32>> {
            delimited(
                '{',
                separated(1.., dec_uint::<_, u32, _>, (',', space0)),
                '}',
            )
            .parse_next(input)
        }

        let (target_lights, _, buttons, _, joltage) =
            (lights, space0, repeat(1.., button), space0, joltage)
                .parse_peek(input)
                .map(|(_, result)| result)?;

        Ok(Self {
            target_lights,
            buttons,
            joltage,
        })
    }

    fn lights_fewest_presses(&self) -> usize {
        self.buttons
            .iter()
            .powerset()
            .filter_map(|buttons| {
                let mut lights = vec![false; self.target_lights.len()];
                for button in &buttons {
                    for idx in *button {
                        lights[*idx] = !lights[*idx];
                    }
                }

                if lights == self.target_lights {
                    Some(buttons.len())
                } else {
                    None
                }
            })
            .min()
            .unwrap()
    }

    fn joltage_fewest_presses(&self) -> usize {
        todo!("Linear programming!")
    }
}

type ParsedInput = Vec<Machine>;

#[aoc_generator(day10)]
fn input_generator(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| Machine::parse(line).unwrap())
        .collect()
}

#[aoc(day10, part1)]
fn solve_part1(input: &ParsedInput) -> usize {
    input.iter().map(Machine::lights_fewest_presses).sum()
}

#[aoc(day10, part2)]
fn solve_part2(input: &ParsedInput) -> usize {
    input.iter().map(Machine::joltage_fewest_presses).sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 2)]
    #[case("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 3)]
    #[case("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}", 2)]
    fn test_day10_machine_lights_fewest_presses(#[case] input: &str, #[case] expected: usize) {
        let machine = Machine::parse(input).unwrap();
        dbg!(&machine);
        assert_eq!(machine.lights_fewest_presses(), expected);
    }

    #[rstest]
    #[case("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}", 10)]
    #[case("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}", 12)]
    #[case("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}", 11)]
    fn test_day10_machine_joltage_fewest_presses(#[case] input: &str, #[case] expected: usize) {
        let machine = Machine::parse(input).unwrap();
        dbg!(&machine);
        assert_eq!(machine.lights_fewest_presses(), expected);
    }

    #[test]
    fn test_day10_part1() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let parsed = input_generator(input);
        assert_eq!(solve_part1(&parsed), 7);
    }

    #[test]
    fn test_day10_part2() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        let parsed = input_generator(input);
        assert_eq!(solve_part2(&parsed), 33);
    }
}

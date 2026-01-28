use std::{collections::HashMap, fmt::Display};

use aoc_runner_derive::aoc;
use cached::{Cached, proc_macro::cached};

type Reg = String;
type Imm = u16;

#[derive(Debug, PartialEq, Eq, Hash)]
enum Input {
    Register(Reg),
    Immediate(Imm),
}

#[cached(key = "String", convert = r#"{ format!("{input}") }"#)]
fn evaluate(input: &Input, registers: &HashMap<Reg, Operation>) -> u16 {
    match input {
        Input::Immediate(imm) => *imm,
        Input::Register(reg) => registers.get(reg).unwrap().solve(registers),
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Register(reg) => write!(f, "{reg}"),
            Self::Immediate(imm) => write!(f, "{imm}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Mov(Input),
    And(Input, Input),
    Or(Input, Input),
    LShift(Input, u16),
    RShift(Input, u16),
    Not(Input),
}

impl Operation {
    fn solve(&self, registers: &HashMap<Reg, Self>) -> u16 {
        match self {
            Self::Mov(input) => evaluate(input, registers),
            Self::And(i1, i2) => evaluate(i1, registers) & evaluate(i2, registers),
            Self::Or(i1, i2) => evaluate(i1, registers) | evaluate(i2, registers),
            Self::LShift(i1, am) => evaluate(i1, registers) << am,
            Self::RShift(i1, am) => evaluate(i1, registers) >> am,
            Self::Not(input) => !evaluate(input, registers),
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mov(reg) => write!(f, "Mov({reg})"),
            Self::And(r1, r2) => write!(f, "And({r1}, {r2})"),
            Self::Or(r1, r2) => write!(f, "Or({r1}, {r2})"),
            Self::LShift(reg, am) => write!(f, "LShift({reg}, {am})"),
            Self::RShift(reg, am) => write!(f, "RShift({reg}, {am})"),
            Self::Not(reg) => write!(f, "Not({reg})"),
        }
    }
}

fn parse_input(input: &str) -> Input {
    if input.chars().next().unwrap().is_alphabetic() {
        Input::Register(input.to_owned())
    } else {
        Input::Immediate(input.parse().unwrap())
    }
}

fn parse_operation(input: &str) -> (Reg, Operation) {
    let mut split = input.split(" -> ");
    let inputs = split.next().unwrap();
    let output = split.next().unwrap().to_owned();

    let op = if inputs.starts_with("NOT") {
        let input = parse_input(inputs.trim_start_matches("NOT "));
        Operation::Not(input)
    } else if inputs.contains("AND") {
        let mut split = inputs.split(" AND ");
        let first = parse_input(split.next().unwrap());
        let second = parse_input(split.next().unwrap());
        Operation::And(first, second)
    } else if inputs.contains("OR") {
        let mut split = inputs.split(" OR ");
        let first = parse_input(split.next().unwrap());
        let second = parse_input(split.next().unwrap());
        Operation::Or(first, second)
    } else if inputs.contains("LSHIFT") {
        let mut split = inputs.split(" LSHIFT ");
        let input = parse_input(split.next().unwrap());
        let amount = split.next().unwrap().parse::<u16>().unwrap();
        Operation::LShift(input, amount)
    } else if inputs.contains("RSHIFT") {
        let mut split = inputs.split(" RSHIFT ");
        let input = parse_input(split.next().unwrap());
        let amount = split.next().unwrap().parse::<u16>().unwrap();
        Operation::RShift(input, amount)
    } else {
        let input = parse_input(inputs);
        Operation::Mov(input)
    };

    (output, op)
}

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> u16 {
    let registers = input
        .lines()
        .map(parse_operation)
        .collect::<HashMap<Reg, Operation>>();

    registers
        .get("a")
        .map_or(u16::MAX, |op| op.solve(&registers))
}

#[aoc(day7, part2)]
fn solve_part2(input: &str) -> u16 {
    let mut registers = input
        .lines()
        .map(parse_operation)
        .collect::<HashMap<Reg, Operation>>();

    let a_val = registers.get("a").unwrap().solve(&registers);

    EVALUATE.lock().unwrap().cache_clear();

    registers
        .entry(String::from("b"))
        .and_modify(|v| *v = Operation::Mov(Input::Immediate(a_val)));

    registers.get("a").unwrap().solve(&registers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1() {
        let input = "123 -> x
456 -> y
i -> a
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";
        let expected = 65079;

        assert_eq!(solve_part1(input), expected);
    }

    #[test]
    fn test_day7_part2() {
        let input = "";
        let expected = 0;

        assert_eq!(solve_part2(input), expected);
    }
}

use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type ParsedInput = Vec<(u64, u64)>;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            (
                split.next().unwrap().parse::<u64>().unwrap(),
                split.next().unwrap().parse::<u64>().unwrap(),
            )
        })
        .collect()
}

#[aoc(day9, part1)]
fn solve_part1(input: &ParsedInput) -> u64 {
    input
        .iter()
        .cartesian_product(input)
        .map(|(r1, r2)| (1 + r1.0.abs_diff(r2.0)) * (1 + r1.1.abs_diff(r2.1)))
        .max()
        .unwrap()
}

#[aoc(day9, part2)]
fn solve_part2(input: &ParsedInput) -> u64 {
    // compress coordinates
    let xs = input.iter().map(|(x, _)| *x).collect::<HashSet<_>>();
    let xs = xs
        .iter()
        .sorted_unstable()
        .enumerate()
        .map(|(idx, x)| (x, idx))
        .collect::<HashMap<_, _>>();
    let ys = input.iter().map(|(_, y)| *y).collect::<HashSet<_>>();
    let ys = ys
        .iter()
        .sorted_unstable()
        .enumerate()
        .map(|(idx, y)| (y, idx))
        .collect::<HashMap<_, _>>();

    let tiles = input
        .iter()
        .map(|(x, y)| (*x, *y, xs[x], ys[y]))
        .collect::<Vec<_>>();

    let mut scale_map =
        vec![vec![false; *xs.values().max().unwrap() + 1]; *ys.values().max().unwrap() + 1];

    // outline
    for (&(_, _, x1, y1), &(_, _, x2, y2)) in tiles.iter().tuple_windows() {
        if x1 == x2 {
            // vertical
            let y_min = y1.min(y2);
            let y_max = y1.max(y2);
            for y in y_min..=y_max {
                scale_map[y][x1] = true;
            }
        } else {
            // horizontal
            let x_min = x1.min(x2);
            let x_max = x1.max(x2);
            for x in x_min..=x_max {
                scale_map[y1][x] = true;
            }
        }
    }
    // fill
    for (y, row) in scale_map.iter_mut().enumerate() {
        for (x, b) in row.iter_mut().enumerate() {
            if *b {
                continue;
            }
            let mut inside = false;
            for (&(_, _, x1, y1), &(_, _, _x2, y2)) in tiles.iter().tuple_windows() {
                if (y1 > y) != (y2 > y) && x < x1 {
                    inside = !inside;
                }
            }
            *b = inside;
        }
    }

    tiles
        .iter()
        .cartesian_product(&tiles)
        .filter_map(|(r1, r2)| {
            let x_min = r1.2.min(r2.2);
            let x_max = r1.2.max(r2.2);
            let y_min = r1.3.min(r2.3);
            let y_max = r1.3.max(r2.3);

            let valid = (x_min..=x_max)
                .cartesian_product(y_min..=y_max)
                .all(|(x, y)| scale_map[y][x]);
            if valid {
                Some((1 + r1.0.abs_diff(r2.0)) * (1 + r1.1.abs_diff(r2.1)))
            } else {
                None
            }
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_part1() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let parsed = input_generator(input);
        assert_eq!(solve_part1(&parsed), 50);
    }

    #[test]
    fn test_day9_part2() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let parsed = input_generator(input);
        assert_eq!(solve_part2(&parsed), 24);
    }
}

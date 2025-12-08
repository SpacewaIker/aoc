use aoc_runner_derive::aoc;

#[aoc(day7, part1)]
fn solve_part1(input: &str) -> u32 {
    let mut rows = input.lines();
    let mut beams = rows
        .next()
        .unwrap()
        .chars()
        .map(|c| c == 'S')
        .collect::<Vec<_>>();

    rows.next(); // second line is only empty space

    let mut splits = 0;
    for row in rows {
        for (i, c) in row.chars().enumerate() {
            if c == '^' && beams[i] {
                beams[i - 1] = true;
                beams[i] = false;
                beams[i + 1] = true;
                splits += 1;
            }
        }
    }

    splits
}

#[aoc(day7, part2)]
fn solve_part2(input: &str) -> u64 {
    let mut rows = input.lines();
    let mut beams = rows
        .next()
        .unwrap()
        .chars()
        .map(|c| u64::from(c == 'S'))
        .collect::<Vec<_>>();

    rows.next(); // second line is only empty space

    for row in rows {
        for (i, c) in row.chars().enumerate() {
            if c == '^' && beams[i] > 0 {
                beams[i - 1] += beams[i];
                beams[i + 1] += beams[i];
                beams[i] = 0;
            }
        }
    }

    beams.into_iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(solve_part1(input), 21);
    }

    #[test]
    fn test_day7_part2() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(solve_part2(input), 40);
    }
}

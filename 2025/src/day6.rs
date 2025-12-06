use aoc_runner_derive::aoc;

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> u64 {
    let input = input.lines().collect::<Vec<_>>();

    let mut probs = input[input.len() - 1]
        .split_whitespace()
        .filter_map(|op| {
            if op.is_empty() {
                None
            } else if op.starts_with('+') {
                Some(('+', 0))
            } else {
                Some(('*', 1))
            }
        })
        .collect::<Vec<_>>();

    for row in &input[..input.len() - 1] {
        for (idx, num) in row.split_whitespace().enumerate() {
            let num = num.parse::<u64>().unwrap();
            probs[idx].1 = match probs[idx].0 {
                '+' => probs[idx].1 + num,
                '*' => probs[idx].1 * num,
                _ => unreachable!(),
            };
        }
    }

    probs.iter().map(|(_, answer)| *answer).sum()
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> u64 {
    let mut input = input
        .lines()
        .map(|line| {
            let mut chars = line.chars().collect::<Vec<_>>();
            chars.push(' ');
            chars
        })
        .collect::<Vec<_>>();

    let num_cols = input[0].len();
    let num_rows = input.len();

    let mut total = 0;
    let mut numbers = Vec::new();
    let mut op = '+';

    for col_idx in 0..num_cols {
        let mut empty_col = true;
        for row in &mut input[..num_rows - 1] {
            if !row[col_idx].is_whitespace() {
                empty_col = false;
                break;
            }
        }
        if !input[num_rows - 1][col_idx].is_whitespace() {
            op = input[num_rows - 1][col_idx];
        }
        if empty_col {
            total += match op {
                '+' => numbers.iter().sum::<u64>(),
                '*' => numbers.iter().product::<u64>(),
                _ => panic!(),
            };
            numbers.clear();
        } else {
            let num = (0..num_rows - 1)
                .map(|row_idx| input[row_idx][col_idx])
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .unwrap();
            numbers.push(num);
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6_part1() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(solve_part1(input), 4_277_556);
    }

    #[test]
    fn test_day6_part2() {
        let input = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
        assert_eq!(solve_part2(input), 3_263_827);
    }
}

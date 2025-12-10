use aoc_runner_derive::{aoc, aoc_generator};
use euclid::default::Point3D;
use indicatif::ProgressIterator;
use itertools::Itertools;

type ParsedInput = Vec<Point3D<f32>>;

#[aoc_generator(day8)]
fn input_generator(input: &str) -> ParsedInput {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            Point3D::new(
                split.next().unwrap().parse::<f32>().unwrap(),
                split.next().unwrap().parse::<f32>().unwrap(),
                split.next().unwrap().parse::<f32>().unwrap(),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
fn solve_part1(input: &ParsedInput) -> u32 {
    solve_part1_inner(input, 1000)
}

fn solve_part1_inner(input: &ParsedInput, num_connections: u32) -> u32 {
    let mut junctions = input.iter().enumerate().collect::<Vec<_>>();
    let mut connected_pairs = std::collections::HashSet::new();

    for _ in (0..num_connections).progress() {
        let mut min_pair = (0, 0);
        let mut min_dist = f32::MAX;
        for (i, (_, p1)) in junctions.iter().enumerate() {
            for (j, (_, p2)) in junctions.iter().enumerate() {
                if i != j
                    && !connected_pairs.contains(&(i, j))
                    && !connected_pairs.contains(&(j, i))
                {
                    let dist = p1.distance_to(**p2);
                    if dist < min_dist {
                        min_dist = dist;
                        min_pair = (i, j);
                    }
                }
            }
        }

        connected_pairs.insert(min_pair);

        let old_group = junctions[min_pair.1].0;
        let new_group = junctions[min_pair.0].0;

        for (circuit, _) in &mut junctions {
            if *circuit == old_group {
                *circuit = new_group;
            }
        }
    }

    let mut group_size = std::collections::HashMap::<usize, u32>::new();
    for (group, _) in junctions {
        group_size
            .entry(group)
            .and_modify(|size| *size += 1)
            .or_insert(1);
    }
    group_size
        .values()
        .sorted_unstable()
        .rev()
        .take(3)
        .product()
}

#[aoc(day8, part2)]
fn solve_part2(input: &ParsedInput) -> u64 {
    let mut junctions = input.iter().enumerate().collect::<Vec<_>>();

    let mut min_pair = (0, 0);
    loop {
        let mut min_dist = f32::MAX;
        for (i, (g1, p1)) in junctions.iter().enumerate() {
            for (j, (g2, p2)) in junctions.iter().enumerate() {
                if i != j && g1 != g2 {
                    let dist = p1.distance_to(**p2);
                    if dist < min_dist {
                        min_dist = dist;
                        min_pair = (i, j);
                    }
                }
            }
        }

        let old_group = junctions[min_pair.1].0;
        let new_group = junctions[min_pair.0].0;

        for (circuit, _) in &mut junctions {
            if *circuit == old_group {
                *circuit = new_group;
            }
        }
        let group = junctions[0].0;
        let all_same_group = junctions.iter().all(|(g, _)| *g == group);
        if all_same_group {
            break;
        }
    }

    junctions[min_pair.0].1.x as u64 * junctions[min_pair.1].1.x as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_part1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let parsed = input_generator(input);
        assert_eq!(solve_part1_inner(&parsed, 10), 40);
    }

    #[test]
    fn test_day8_part2() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let parsed = input_generator(input);
        assert_eq!(solve_part2(&parsed), 25272);
    }
}

use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};
use cached::proc_macro::cached;
use petgraph::graph::NodeIndex;

type Graph = petgraph::graph::Graph<String, ()>;

struct ParsedInput {
    graph: Graph,
    you: Option<NodeIndex>,
    out: Option<NodeIndex>,
    svr: Option<NodeIndex>,
    dac: Option<NodeIndex>,
    fft: Option<NodeIndex>,
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> ParsedInput {
    let mut graph = Graph::new();
    let mut nodes = HashMap::new();
    let mut you = None;
    let mut out = None;
    let mut svr = None;
    let mut dac = None;
    let mut fft = None;

    for line in input.lines() {
        let mut split = line.split(": ");
        let node = split.next().unwrap();
        let node_idx = *nodes
            .entry(node)
            .or_insert_with(|| graph.add_node(node.to_owned()));

        if node == "you" && you.is_none() {
            you = Some(node_idx);
        }
        if node == "svr" && svr.is_none() {
            svr = Some(node_idx);
        }
        if node == "dac" && dac.is_none() {
            dac = Some(node_idx);
        }
        if node == "fft" && fft.is_none() {
            fft = Some(node_idx);
        }

        for other in split.next().unwrap().split_whitespace() {
            let other_idx = *nodes
                .entry(other)
                .or_insert_with(|| graph.add_node(other.to_owned()));

            if other == "out" && out.is_none() {
                out = Some(other_idx);
            }

            graph.add_edge(node_idx, other_idx, ());
        }
    }

    assert!(
        !petgraph::algo::is_cyclic_directed(&graph),
        "Graph is cyclic!"
    );

    ParsedInput {
        graph,
        you,
        out,
        svr,
        dac,
        fft,
    }
}

#[aoc(day11, part1, rec)]
fn solve_part1_rec(pi: &ParsedInput) -> u32 {
    fn paths_to(graph: &Graph, node: NodeIndex, target: NodeIndex) -> u32 {
        let mut count = 0;
        for neighbour in graph.neighbors(node) {
            if neighbour == target {
                count += 1;
            } else {
                count += paths_to(graph, neighbour, target);
            }
        }
        count
    }

    let graph = &pi.graph;
    let start = pi.you.unwrap();
    let target = pi.out.unwrap();

    paths_to(graph, start, target)
}

#[aoc(day11, part1, rec_cached)]
fn solve_part1_rec_cached(pi: &ParsedInput) -> u32 {
    #[cached(key = "(NodeIndex)", convert = "{ (node) }")]
    fn paths_to(graph: &Graph, node: NodeIndex, target: NodeIndex) -> u32 {
        let mut count = 0;
        for neighbour in graph.neighbors(node) {
            if neighbour == target {
                count += 1;
            } else {
                count += paths_to(graph, neighbour, target);
            }
        }
        count
    }

    let graph = &pi.graph;
    let start = pi.you.unwrap();
    let target = pi.out.unwrap();

    paths_to(graph, start, target)
}

#[aoc(day11, part1, basic)]
fn solve_part1_basic(pi: &ParsedInput) -> u32 {
    let graph = &pi.graph;
    let start = pi.you.unwrap();
    let target = pi.out.unwrap();

    let mut paths = 0;
    let mut stack = vec![start];

    while let Some(node) = stack.pop() {
        for neighbour in graph.neighbors(node) {
            if neighbour == target {
                paths += 1;
            } else {
                stack.push(neighbour);
            }
        }
    }

    paths
}

#[aoc(day11, part2)]
fn solve_part2(pi: &ParsedInput) -> u64 {
    #[rustfmt::skip]
    #[cached(key = "(NodeIndex, bool, bool)", convert = "{ (node, visited_dac, visited_fft) }")]
    fn paths_to(graph: &Graph, node: NodeIndex, target: NodeIndex, dac: NodeIndex, fft: NodeIndex, visited_dac: bool, visited_fft: bool) -> u64 {
        let mut count = 0;
        for neighbour in graph.neighbors(node) {
            if neighbour == target {
                if visited_dac && visited_fft {
                    count += 1;
                }
            } else {
                count += paths_to(graph, neighbour, target, dac, fft, visited_dac || neighbour == dac, visited_fft || neighbour == fft);
            }
        }
        count
    }

    let graph = &pi.graph;
    let svr = pi.svr.unwrap();
    let dac = pi.dac.unwrap();
    let fft = pi.fft.unwrap();
    let target = pi.out.unwrap();

    paths_to(graph, svr, target, dac, fft, false, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11_part1_rec() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let parsed = input_generator(input);
        assert_eq!(solve_part1_rec(&parsed), 5);
    }

    #[test]
    fn test_day11_part1_basic() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let parsed = input_generator(input);
        assert_eq!(solve_part1_basic(&parsed), 5);
    }

    #[test]
    fn test_day11_part2() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let parsed = input_generator(input);
        assert_eq!(solve_part2(&parsed), 2);
    }
}

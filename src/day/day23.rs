use std::collections::HashSet;
use std::ops::RangeInclusive;
use fast_graph::{Graph, GraphInterface};
use fxhash::{FxBuildHasher, FxHashMap, FxHashSet};
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending};
use nom::character::complete::alpha1;
use nom::IResult;
use nom::multi::{fold_many1, separated_list1};
use nom::sequence::{separated_pair, terminated};
use crate::*;
use crate::collections::ArrayBag;
use crate::day::nom_parsed_bytes;


#[derive(Debug, Default)]
struct WorkingData<'a> {
    edges: FxHashMap<&'a str, FxHashSet<&'a str>>,
    three_cliques: FxHashSet<[&'a str; 3]>,
}

fn parse_input(input: &str) -> IResult<&str, WorkingData> {
    let parse_single_line = separated_pair(alpha1, tag("-"), alpha1);
    fold_many1(terminated(parse_single_line, line_ending), WorkingData::default, |mut result, (a, b)| {
        result.edges.entry(a).or_default().insert(b);
        result.edges.entry(b).or_default().insert(a);
        result
    })(input)
}

fn part1(input: &mut WorkingData) -> usize {
    for (first, neighbours) in input.edges.iter() {
        if first.starts_with('t') {
            for second in neighbours {
                let neighbours_of_neighbour = input.edges.get(second).unwrap();
                for third in neighbours.intersection(neighbours_of_neighbour) {
                    let mut clique_code = [*first, *second, *third];
                    clique_code.sort();
                    input.three_cliques.insert(clique_code);
                }
            }
        }
    }

    input.three_cliques.len()
}

fn find_extension_node<'a, 'b>(clique: &'b FxHashSet<&'a str>, edges: &FxHashMap<&'a str, FxHashSet<&'a str>>) -> Option<&'a str> {
    let one_node = clique.iter().next().unwrap();
    let potential_members = edges.get(one_node).unwrap();
    for grow_candidate in potential_members {
        if clique.iter().copied().all(|node| {
            edges.get(node).unwrap().contains(grow_candidate)
        }) {
            // all current clique members contain the node, add it
            return Some(grow_candidate);
        }
    }

    None
}

fn part2(input: WorkingData) -> String {
    let mut maximal_cliques = Vec::<FxHashSet<&str>>::default();
    let mut largest_set = FxHashSet::default();

    for root in input.three_cliques {
        let mut clique_set = FxHashSet::from_iter(root.iter().copied());
        if maximal_cliques.iter().any(|maximum_clique| maximum_clique.is_superset(&clique_set)) {
            continue;
        }

        while let Some(extension) = find_extension_node(&clique_set, &input.edges) {
            clique_set.insert(extension);
        }

        if clique_set.len() > largest_set.len() {
            largest_set = clique_set.clone();
        }
        maximal_cliques.push(clique_set);
    }

    let mut sorted: Vec<_> = largest_set.iter().copied().collect();
    sorted.sort();
    sorted.join(",")
}

fn solve(input: &[u8]) -> String {
    let input_string = String::from_utf8_lossy(input);
    let mut parsed = parse_input(input_string.as_ref()).unwrap().1;
    let part1 = part1(&mut parsed);
    let part2 = part2(parsed);

    format!("Part 1: {part1}, Part 2: {part2}")
}

simple_day!(|x|solve(x));
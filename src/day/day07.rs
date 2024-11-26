use std::collections::HashMap;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending};
use nom::combinator::{map, opt};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use crate::*;
use crate::day::{nom_parsed};

parsed_day!(nom_parsed(parse), part1, part2);


fn find_mismatch_in<'input, 'nodes>(nodes: &'nodes Vec<Day7Node<'input>>, mut path_accu: Vec<&'nodes Day7Node<'input>>) -> Vec<&'nodes Day7Node<'input>> {
    let mismatched_node = nodes.as_slice().windows(3).find_map(|w| {
        if let [a, b, c] = w {
            if a.cumulative_weight != b.cumulative_weight {
                if b.cumulative_weight != c.cumulative_weight {
                    Some(b)
                } else {
                    Some(a)
                }
            } else if b.cumulative_weight != c.cumulative_weight {
                Some(c)
            } else {
                None
            }
        } else {
            panic!("Pattern match failed for {:?}", w)
        }
    });

    if let Some(mismatched_node) = mismatched_node {
        path_accu.push(mismatched_node);
        path_accu = find_mismatch_in(&mismatched_node.children, path_accu)
    }

    path_accu
}

fn part2(root: Day7) -> usize {
    let root = root.root.unwrap();
    let mismatch_node_path = find_mismatch_in(&root.children, vec![&root]);
    let wrong_weight_node = mismatch_node_path[mismatch_node_path.len() - 1];
    let child_sum = wrong_weight_node.children.iter().map(|c|c.cumulative_weight).sum::<usize>();
    let wrong_weight_sibling = mismatch_node_path[mismatch_node_path.len() - 2].children.iter().find(|it|it.name != wrong_weight_node.name).unwrap();
    let reference_weight = wrong_weight_sibling.cumulative_weight;
    let required_self_weight = reference_weight - child_sum;
    required_self_weight
}

fn part1<'day, 'input>(input: &'day mut Day7<'input>) -> &'input str {
    let mut unresolved = HashMap::new();
    let mut next_spec = input.node_specs.iter().map(|(name, _)| *name).next();
    while let Some(name) = next_spec {
        let next = input.node_specs.remove(name).unwrap();
        let next_node = next.resolve(&mut input.node_specs, &mut unresolved);
        unresolved.insert(next_node.name, next_node);
        next_spec = input.node_specs.iter().map(|(name, _)| *name).next();
    }

    let mut iter = unresolved.into_iter();
    input.root = iter.next().map(|(_, d)| d);
    assert!(iter.next().is_none());
    input.root.as_ref().unwrap().name
}

#[derive(Debug)]
struct NodeSpec<'a> {
    name: &'a str,
    weight: usize,
    unresolved_child_names: Vec<&'a str>,
}

impl<'a> NodeSpec<'a> {
    fn resolve(self, other_specs: &mut HashMap<&'a str, NodeSpec<'a>>, unresolved_nodes: &mut HashMap<&'a str, Day7Node<'a>>) -> Day7Node<'a> {
        let NodeSpec { name, weight, mut unresolved_child_names } = self;
        let mut children = Vec::new();
        let mut cumulative_weight = weight;
        while let Some(child) = unresolved_child_names.pop() {
            if let Some(dangling_node) = unresolved_nodes.remove(child) {
                cumulative_weight += dangling_node.cumulative_weight;
                children.push(dangling_node)
            } else {
                let recurse_spec = other_specs.remove(child).expect("Referenced node exists");
                let child = recurse_spec.resolve(other_specs, unresolved_nodes);
                cumulative_weight += child.cumulative_weight;
                children.push(child)
            }
        }

        Day7Node { name, cumulative_weight, children }
    }
}

#[derive(Debug)]
struct Day7Node<'a> {
    name: &'a str,
    cumulative_weight: usize,
    children: Vec<Day7Node<'a>>,
}

#[derive(Debug)]
struct Day7<'a> {
    node_specs: HashMap<&'a str, NodeSpec<'a>>,
    root: Option<Day7Node<'a>>,
}

fn child_list(input: &str) -> IResult<&str, Vec<&str>> {
    preceded(tag(" -> "), separated_list1(tag(", "), alpha1))(input)
}

fn node_spec(input: &str) -> IResult<&str, NodeSpec> {
    map(tuple((
        alpha1::<&str, _>,
        tag(" ("),
        digit1,
        tag(")"),
        opt(child_list)
    )), |(name, _, weight, _, children)| {
        let weight = weight.parse().unwrap();
        let unresolved_child_names = children.unwrap_or_default();
        NodeSpec { name, weight, unresolved_child_names }
    })(input)
}

fn parse(input: &str) -> IResult<&str, Day7> {
    let (rest, node_specs) = separated_list1(line_ending, node_spec)(input)?;
    let node_specs = node_specs.into_iter()
        .map(|spec| (spec.name, spec))
        .collect();
    Ok((rest, Day7 {
        node_specs,
        root: None,
    }))
}
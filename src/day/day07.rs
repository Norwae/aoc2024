use std::collections::HashMap;
use std::time::Instant;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, line_ending};
use nom::combinator::{map, opt};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use crate::*;
use crate::day::nom_parsed;

simple_day!(solve);

fn solve<T: UIWrite>(input: &str, out: &mut T) -> String {
    let start_parse = Instant::now();
    let (_, mut parsed) = parse(input).unwrap();
    let start_part_1 = Instant::now();
    out.info(format_args!("Parse: {:?}", start_part_1 - start_parse));
    part1(&mut parsed);
    let root = parsed.root.unwrap();
    let start_part_2 = Instant::now();
    out.info(format_args!("Part 1 -> {}: {:?}", root.name, start_part_2 - start_part_1));


    out.info(format_args!("Done: {:?}", start_part_2 - start_parse));
    format!("{}", root.name)

}

fn part1<'a>(input: &'a mut Day7) {
    let mut unresolved = HashMap::new();
    let mut next_spec = input.node_specs.iter().map(|(name, _)|*name).next();
    while let Some(name) = next_spec {
        let next = input.node_specs.remove(name).unwrap();
        let next_node = next.resolve(&mut input.node_specs, &mut unresolved);
        unresolved.insert(next_node.name, next_node);
        next_spec = input.node_specs.iter().map(|(name, _)|*name).next();
    }

    let mut iter = unresolved.into_iter();
    input.root = iter.next().map(|(_, d)|d);
    assert!(iter.next().is_none());
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
        while let Some(child) = unresolved_child_names.pop() {
            if let Some(dangling_node) = unresolved_nodes.remove(child){
                children.push(dangling_node)
            } else {
                let recurse_spec = other_specs.remove(child).expect("Referenced node exists");
                children.push(recurse_spec.resolve(other_specs, unresolved_nodes))
            }
        }

        Day7Node { name, weight, children }
    }
}

#[derive(Debug)]
struct Day7Node<'a> {
    name: &'a str,
    weight: usize,
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
        let children = children.unwrap_or_default();
        NodeSpec { name, weight, unresolved_child_names: children }
    })(input)
}

fn parse(input: &str) -> IResult<&str, Day7> {
    let (rest, node_specs) = separated_list1(line_ending, node_spec)(input)?;
    let node_specs = node_specs.into_iter()
        .map(|spec|(spec.name, spec))
        .collect();
    Ok((rest, Day7 {
        node_specs,
        root: None,
    }))
}
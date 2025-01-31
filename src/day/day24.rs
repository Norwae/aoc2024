use crate::*;
use std::borrow::Cow;
use std::fmt::{format, Debug, Formatter};
use fxhash::FxHashMap;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{alphanumeric1, line_ending, one_of};
use nom::combinator::{map, value};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use crate::*;
use crate::day::nom_parsed_bytes;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Operation {
    AND, OR, XOR
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Label<'a>(&'a[u8]);
impl Debug for Label<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(String::from_utf8_lossy(self.0).as_ref())
    }
}

#[derive(Debug)]
struct Gate<'a> {
    operation: Operation,
    input_1: Label<'a>,
    input_2: Label<'a>,
    output: Label<'a>
}

#[derive(Debug)]
struct Input<'a> {
    label: Label<'a>,
    value: bool
}

fn parse_label(input: &[u8]) -> IResult<&[u8], Label> {
    map(alphanumeric1, |bytes| Label(bytes))(input)
}

fn parse_input(input: &[u8]) -> IResult<&[u8], Input> {
    let (rest, (label, value)) = separated_pair(parse_label, tag(": "), alt((value(true, tag(b"1")), value(false, tag(b"0")))))(input)?;

    Ok((rest, Input { label, value }))
}

fn parse_gate(input: &[u8]) -> IResult<&[u8], Gate> {
    let mut parse_op = alt((
        value(Operation::AND, tag(" AND ")),
        value(Operation::OR, tag(" OR ")),
        value(Operation::XOR, tag(" XOR "))
    ));

    let (rest, (input_1, operation, input_2, _, output)) = tuple((parse_label, parse_op, parse_label, tag(" -> "), parse_label))(input)?;

    Ok((rest, Gate { operation, input_1, input_2, output, }))
}

fn parse(input: &[u8]) -> IResult<&[u8], (Vec<Input>, Vec<Gate>)> {
    separated_pair(separated_list1(line_ending, parse_input), tuple((line_ending, line_ending)), separated_list1(line_ending, parse_gate))(input)
}

fn solve((inputs, gates): (Vec<Input>, Vec<Gate>)) -> usize {
    let gate_by_output_label: FxHashMap<_, _> = gates.into_iter().map(|g| {
        (g.output, g)
    }).collect();

    let input_by_label: FxHashMap<_, _> = inputs.into_iter().map(|input| (input.label, input)).collect();

    let mut result = 0;
    let mut shift = 0;
    let mut key = format!("z{shift:>02}");
    while let Some(gate) = gate_by_output_label.get(&Label(key.as_bytes())) {
        let bit = calculate(&gate_by_output_label, &input_by_label, gate);
        if bit {
            let set = 1usize << shift;
            result |= set;
        }
        shift += 1;
        key = format!("z{shift:>02}");
    }

    result
}

fn calculate<'a, 'input>(gates: &'a FxHashMap<Label<'input>, Gate<'input>>, inputs: &'a FxHashMap<Label<'input>, Input<'input>>, cursor: &'a Gate<'input>) -> bool {
    let input_by_label = |label: &Label<'input>| if let Some(input) = inputs.get(label) {
        input.value
    } else {
        calculate(gates, inputs, gates.get(label).unwrap())
    };

    let input_1 = input_by_label(&cursor.input_1);
    let input_2 = input_by_label(&cursor.input_2);

    match cursor.operation {
        Operation::AND => { input_1 & input_2 },
        Operation::OR => { input_1 | input_2 },
        Operation::XOR => { input_1 ^ input_2 },
    }
}

parsed_day!(nom_parsed_bytes(parse), solve);
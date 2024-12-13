use crate::day::nom_parsed_bytes;
use crate::parse_helpers::parse_unsigned_nr_bytes;
use crate::*;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;
/*
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

in other words:
A * 94 + B * 22 = 8400
A * 34 + B * 67 = 5400

Solve for A, B
Integer solutions only
*/

#[derive(Debug)]
struct Linear2x2System {
    a: [[f64; 2]; 2],
    b: [f64; 2],
}

fn solve_2x2_linear_system(linear_2x2: &Linear2x2System) -> Option<(u64, u64)> {
    let Linear2x2System { a, b } = linear_2x2;
    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
    if det.abs() < f64::EPSILON {
        return None; // Degenerate matrix, no unique solution
    }

    let x = (b[0] * a[1][1] - b[1] * a[0][1]) / det;
    let y = (a[0][0] * b[1] - a[1][0] * b[0]) / det;

    if x != x.floor()
        || y != y.floor()
        || x < 0.0
        || y < 0.0
    {
        None
    } else {
        Some((x as u64, y as u64))
    }
}

parsed_day!(nom_parsed_bytes(parse), |systems| {
    let mut sum = 0;
    for sys in systems {
        if let Some((a, b)) = solve_2x2_linear_system(sys) {
            sum += 3 * a + b
        }
    }
    sum
}, |systems|{
    let mut sum = 0u64;
    for mut sys in systems {
        sys.b[0] += 10000000000000.;
        sys.b[1] += 10000000000000.;
        if let Some((a, b)) = solve_2x2_linear_system(&sys) {
            sum += 3 * a + b
        }
    }
    sum
});

fn parse(input: &[u8]) -> IResult<&[u8], Vec<Linear2x2System>> {
    let parse_xy = |prefix: &'static [u8], interstice: &'static [u8]| {
        preceded(
            tag(prefix),
            separated_pair(
                parse_unsigned_nr_bytes,
                tag(interstice),
                parse_unsigned_nr_bytes,
            ),
        )
    };

    let parse_single_system = map(
        tuple((
            terminated(parse_xy(b"Button A: X+", b", Y+"), line_ending),
            terminated(parse_xy(b"Button B: X+", b", Y+"), line_ending),
            terminated(parse_xy(b"Prize: X=", b", Y="), line_ending),
        )),
        |(a, b, c)| Linear2x2System {
            a: [[a.0, b.0], [a.1, b.1]],
            b: [c.0, c.1],
        },
    );
    separated_list1(line_ending, parse_single_system)(input)
}

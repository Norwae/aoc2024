use std::fmt::{Display, Formatter, Write};
use crate::collections::{Index2D, Vec2D};
use crate::parse_helpers::parse_signed_nr_bytes;
use crate::*;
use nom::bytes::complete::tag;
use nom::character::complete::line_ending;
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use nom::multi::{separated_list0};
use crate::day::{nom_parsed_bytes, visual_inspection};

#[derive(Debug, Clone)]
struct Robot {
    position: Index2D,
    velocity: Index2D,
}

const HEIGHT: usize = 103;
const WIDTH: usize = 101;

impl Robot {
    fn step(&mut self) {
        self.position.row += self.velocity.row;
        self.position.column += self.velocity.column;
        self.position.row %= HEIGHT;
        self.position.column %= WIDTH;
    }

    fn assign_to_quadrant(&self, quadrants: &mut [usize]) {
        if self.position.row == (HEIGHT / 2) || self.position.column == (WIDTH / 2) {
            return;
        }

        if self.position.row < HEIGHT / 2 {
            if self.position.column < WIDTH / 2 {
                quadrants[0] += 1;
            } else {
                quadrants[1] += 1;
            }
        } else {
            if self.position.column < WIDTH / 2 {
                quadrants[2] += 1;
            } else {
                quadrants[3] += 1;
            }
        }
    }
}

fn parse_location(input: &[u8]) -> IResult<&[u8], Index2D> {
    map(
        separated_pair(parse_signed_nr_bytes::<i64>, tag(b","), parse_signed_nr_bytes::<i64>),
        |(mut x, mut y)| {
            if x < 0 {
                x += WIDTH as i64;
            };
            if y < 0 {
                y += HEIGHT as i64;
            }
            Index2D { row: y as usize, column: x as usize }
        },
    )(input)
}
fn parse_line(input: &[u8]) -> IResult<&[u8], Robot> {
    map(preceded(
        tag(b"p="),
        separated_pair(parse_location, tag(b" v="), parse_location),
    ), |(position, velocity)| Robot { position, velocity })(input)
}

fn p1(robots: &mut Vec<Robot>) -> usize {
    let mut robots = robots.clone();
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.step();
        }
    }

    let mut quadrants = [0usize; 4];
    for robot in robots {
        robot.assign_to_quadrant(&mut quadrants);
    }

    quadrants.iter().product()
}

struct DisplayType<'a>(&'a [Robot]);

impl Display for DisplayType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..HEIGHT {
            for column in 0..WIDTH {
                let char =
                    if self.0.iter().any(|robot| robot.position == Index2D{ row, column }) {
                        '#'
                    } else {
                        '.'
                    };
                f.write_char(char)?;
            }

            f.write_char('\n')?;
        }

        Ok(())
    }
}
fn p2(mut robots: Vec<Robot>) -> usize {
    let mut i = 0;
    loop {
        let mut counts = Vec2D::new_from_flat(vec![0i32; (HEIGHT * WIDTH) as usize], WIDTH as usize);

        i += 1;
        for robot in robots.iter_mut() {
            robot.step();
            counts[robot.position]+=1;
        }
        let mut quadrants = [0usize; 4];
        for robot in robots.iter() {
            robot.assign_to_quadrant(&mut quadrants);
        }

        if !counts.as_slice().iter().any(|x| *x > 1) {
            if visual_inspection(DisplayType(&robots), i) {
                return i
            }
        }
    }
}

parsed_day!(nom_parsed_bytes(separated_list0(line_ending, parse_line)), p1, p2);

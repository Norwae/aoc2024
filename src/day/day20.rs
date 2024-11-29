use std::collections::{HashMap, HashSet};
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0};
use nom::IResult;
use nom::multi::separated_list1;
use nom::sequence::{tuple};
use crate::*;
use crate::day::nom_parsed;
use crate::parse_helpers::parse_signed_nr;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Particle {
    position: IntVector3D,
    velocity: IntVector3D,
    acceleration: IntVector3D,
}

impl Particle {
    fn update(&mut self) {
        self.velocity.x += self.acceleration.x;
        self.velocity.y += self.acceleration.y;
        self.velocity.z += self.acceleration.z;
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Default, Clone)]
struct IntVector3D {
    x: i64,
    y: i64,
    z: i64,
}

fn parse_int_vector_3d(input: &str) -> IResult<&str, IntVector3D> {
    let (rest, (_, _, x, _, _, y, _, _, z, _)) = tuple((
        tag("<"),
        space0,
        parse_signed_nr,
        tag(","),
        space0,
        parse_signed_nr,
        tag(","),
        space0,
        parse_signed_nr,
        tag(">")
    ))(input)?;

    Ok((rest, IntVector3D { x, y, z }))
}

fn parse_particle(input: &str) -> IResult<&str, Particle> {
    let (rest, (_, position, _, velocity, _, acceleration)) = tuple((
        tag("p="),
        parse_int_vector_3d,
        tag(", v="),
        parse_int_vector_3d,
        tag(", a="),
        parse_int_vector_3d
    ))(input)?;
    Ok((rest, Particle { position, velocity, acceleration }))
}

parsed_day!(nom_parsed(separated_list1(line_ending, parse_particle)), p1, p2);

fn p1(input: &mut Vec<Particle>) -> usize {
    let mut best = i64::MAX;
    let mut best_index = usize::MAX;

    for (i, particle) in input.iter().enumerate() {
        let delta_v = particle.acceleration.x.abs() + particle.acceleration.y.abs() + particle.acceleration.z.abs();
        if delta_v < best {
            best_index = i;
            best = delta_v;
        }
    }

    best_index
}

fn p2(mut input: Vec<Particle>) -> String {
    const NO_COLLISION_THRESHOLD: usize = 100;
    let mut i = 0;
    let mut no_collision_count = 0;
    while no_collision_count < NO_COLLISION_THRESHOLD {
        let mut position_to_particle = HashMap::<IntVector3D, Vec<Particle>>::new();
        let mut clean = true;

        for mut particle in input {
            particle.update();
            let vec = position_to_particle.entry(particle.position.clone()).or_default();
            vec.push(particle);
        }

        input = position_to_particle.into_iter().filter_map(|(_, mut ps)| {
            if ps.len() == 1 {
                ps.pop()
            } else {
                None
            }
        }).collect();
        if clean {
            no_collision_count += 1
        } else {
            no_collision_count = 0
        }
        i += 1;
    }

    format!("{} (in {} iterations)", input.len(), i)
}
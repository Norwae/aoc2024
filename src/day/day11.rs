use std::ops::Add;
use crate::*;

#[derive(Debug, Copy, Clone)]
enum HexDirection {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Default)]
struct CubicHexCoordinate {
    q: i32,
    r: i32,
    s: i32
}

impl CubicHexCoordinate {
    const ORIGIN: Self  = const { Self { q: 0, r: 0, s: 0 }};
    fn origin_distance(self) -> usize {
        (self.q.abs() + self.r.abs() + self.s.abs()) as usize / 2
    }
}

impl Add<HexDirection> for CubicHexCoordinate {
    type Output = CubicHexCoordinate;

    fn add(self, rhs: HexDirection) -> Self::Output {
        match rhs {
            HexDirection::North => Self {
                q: self.q,
                r: self.r - 1,
                s: self.s + 1,
            },
            HexDirection::NorthEast => Self {
                q: self.q + 1,
                r: self.r - 1,
                s: self.s,
            },
            HexDirection::SouthEast => Self {
                q: self.q + 1,
                r: self.r,
                s: self.s - 1,
            },
            HexDirection::South => Self {
                q: self.q,
                r: self.r + 1,
                s: self.s - 1,
            },
            HexDirection::SouthWest => Self {
                q: self.q - 1,
                r: self.r + 1,
                s: self.s
            },
            HexDirection::NorthWest => Self {
                q: self.q - 1,
                r: self.r,
                s: self.s + 1
            },
        }
    }
}

simple_day!(|input|{
   use HexDirection::*;

   let moves = input.split(",")
    .map(str::trim)
    .filter(|f|  !f.is_empty())
    .map(|fragment|match fragment {
        "n" => North,
        "ne" => NorthEast,
        "se" => SouthEast,
        "s" => South,
        "sw" => SouthWest,
        "nw" => NorthWest,
        _ => panic!("Bad input: {}", fragment)
    }).collect::<Vec<HexDirection>>();

    let mut cursor = CubicHexCoordinate::ORIGIN;
    let mut max_distance = 0;
    for m in moves {
        cursor = cursor + m;
        max_distance = max_distance.max(cursor.origin_distance())
    }

    format!("Part1: {}, Part2: {}", cursor.origin_distance(), max_distance)
});
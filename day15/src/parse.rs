#![allow(dead_code, unused_imports)]

use std::{collections::HashSet, fmt};

#[derive(Hash)]
pub struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}

// #[derive(Debug)]
pub struct SensorBeaconPair {
    sensor: Coord,
    beacon: Coord,
}
impl fmt::Debug for SensorBeaconPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sensor({:?}), Beacon({:?})", self.sensor, self.beacon)
    }
}

struct CaveMap {
    parded_pairs: Vec<SensorBeaconPair>,
    exhaustive_map: HashSet<Coord>,
}

use nom::{
    bytes::complete::tag, character::complete as cc, multi::separated_list1, sequence::tuple,
    Finish, IResult,
};

fn parse_line(i: &str) -> IResult<&str, SensorBeaconPair> {
    let (i, (_, sx, _, sy, _, bx, _, by)) = tuple((
        tag("Sensor at x="),
        cc::i32,
        tag(", y="),
        cc::i32,
        tag(": closest beacon is at x="),
        cc::i32,
        tag(", y="),
        cc::i32,
    ))(i)?;

    Ok((
        i,
        SensorBeaconPair {
            sensor: Coord::from(sx, sy),
            beacon: Coord::from(bx, by),
        },
    ))
}
pub fn parse_cave(i: &str) -> IResult<&str, Vec<SensorBeaconPair>> {
    separated_list1(tag("\n"), parse_line)(i)
}

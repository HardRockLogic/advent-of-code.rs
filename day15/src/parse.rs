#![allow(dead_code, unused_imports)]

use std::{collections::HashSet, fmt};

#[derive(Hash, PartialEq, Eq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl Coord {
    pub fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl fmt::Debug for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {} y: {}", self.x, self.y)
    }
}

pub struct SensorBeaconPair {
    pub sensor: Coord,
    pub beacon: Coord,
    pub delta: i32,
}

impl fmt::Debug for SensorBeaconPair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Sensor({:?}), Beacon({:?}), d{}",
            self.sensor, self.beacon, self.delta
        )
    }
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

    let delta = (sx - bx).abs() + (sy - by).abs();

    Ok((
        i,
        SensorBeaconPair {
            sensor: Coord::from(sx, sy),
            beacon: Coord::from(bx, by),
            delta,
        },
    ))
}
pub fn parse_cave(i: &str) -> IResult<&str, Vec<SensorBeaconPair>> {
    separated_list1(tag("\n"), parse_line)(i)
}

#![allow(dead_code, unused_imports)]

pub struct Name([u8; 2]);

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a as char, b as char)
    }
}
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Name {
    fn scrap_name(i: &str) -> IResult<&str, Self> {
        map(take(2usize), |slice: &str| {
            Self(slice.as_bytes().try_into().unwrap())
        })(i)
    }
}

#[derive(Debug)]
pub struct Valve {
    name: Name,
    flow: u32,
    adjecents: Vec<Name>,
}

use core::fmt;
use std::write;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete as cc,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

impl Valve {
    pub fn scrap_valve(i: &str) -> IResult<&str, Self> {
        let (i, (name, flow, adjecents)) = tuple((
            preceded(tag("Valve "), Name::scrap_name),
            preceded(tag(" has flow rate="), cc::u32),
            preceded(
                alt((
                    tag("; tunnel leads to valve "),
                    tag("; tunnels lead to valves "),
                )),
                separated_list1(tag(", "), Name::scrap_name),
            ),
        ))(i)?;

        Ok((
            i,
            Self {
                name,
                flow,
                adjecents,
            },
        ))
    }
}

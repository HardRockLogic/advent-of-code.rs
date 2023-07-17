#![allow(dead_code, unused_imports)]

pub struct Name([u8; 2]);

pub struct Valve {
    name: Name,
    flow: u32,
    adjecents: Vec<Name>,
}

use nom::{
    bytes::complete::tag, combinator::map, multi::separated_list1, sequence::tuple, IResult,
};

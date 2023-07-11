#![allow(dead_code)]

mod parse;

use nom::Finish;
use parse::{parse_cave, Coord, SensorBeaconPair};
use std::{collections::HashSet, println};

#[derive(Debug)]
struct CaveMap {
    parsed_pairs: Vec<SensorBeaconPair>,
    exhaustive_map: HashSet<Coord>,
}

impl CaveMap {
    fn parse(i: &str) -> Self {
        let parsed_pairs = parse_cave(i).finish().unwrap().1;
        let exhaustive_map = HashSet::new();

        Self {
            parsed_pairs,
            exhaustive_map,
        }
    }

    fn find_all(&mut self) -> u32 {
        let mut output = 0;

        let max = self
            .parsed_pairs
            .iter()
            .max_by_key(|&pair| pair.sensor.x)
            .unwrap();
        let min = self
            .parsed_pairs
            .iter()
            .min_by_key(|&pair| pair.sensor.x)
            .unwrap();

        // println!("max -> {max:?}\nmin -> {min:?}");

        dbg!((max.sensor.x + max.delta) - (min.sensor.x - min.delta));

        'outer: for i in min.sensor.x - min.delta..=max.sensor.x + max.delta {
            let target = Coord::from(i, 2_000_000);

            for pair in self.parsed_pairs.iter() {
                // in this particular case, this control-flow eliminates only 1 point,
                // and final result will be 5394423 instead of 5394424
                if target == pair.beacon {
                    continue;
                }

                let target_delta =
                    (target.x - pair.sensor.x).abs() + (target.y - pair.sensor.y).abs();

                if target_delta <= pair.delta {
                    output += 1;
                    continue 'outer;
                }
            }
        }
        output
    }
}

fn main() {
    let file = std::fs::read_to_string("day15.txt").unwrap();

    let mut res = CaveMap::parse(&file);

    dbg!(res.find_all());

    // let count = res
    //     .exhaustive_map
    //     .iter()
    //     .filter(|elem| elem.y == 10)
    //     .count();
    //
    // dbg!(count);
}

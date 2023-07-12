#![allow(dead_code)]

mod parse;

use itertools::Itertools;
use nom::Finish;
use parse::{parse_cave, Coord, SensorBeaconPair};
use std::{collections::HashSet, ops::RangeInclusive};

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

    fn brute_search(&mut self) -> u32 {
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

    fn segment_intersection(&self, ohter: i32) -> impl Iterator<Item = RangeInclusive<i32>> {
        let mut segments = Vec::new();

        for pair in self.parsed_pairs.iter() {
            let abs_diff = (pair.sensor.y - ohter).abs();

            if abs_diff > pair.delta {
                continue;
            }

            let start = pair.sensor.x - pair.delta + abs_diff;
            let end = pair.sensor.x + pair.delta - abs_diff;

            segments.push(start..=end);
        }
        segments.sort_by_key(|r| *r.start());

        segments.into_iter().coalesce(|a, b| {
            if *a.end() >= b.start() - 1 {
                Ok(*a.start()..=*b.end().max(a.end()))
            } else {
                Err((a, b))
            }
        })
    }
    fn find_all(&mut self) {
        for pair in self.parsed_pairs.iter() {
            let delta =
                (pair.sensor.x - pair.beacon.x).abs() + (pair.sensor.y - pair.beacon.y).abs();

            let mut start = pair.sensor.x - delta + 1;
            let mut end = pair.sensor.x + delta - 1;

            for y in pair.sensor.y + 1..=pair.sensor.y + delta {
                for x in start..=end {
                    self.exhaustive_map.insert(Coord::from(x, y));

                    if pair.sensor == Coord::from(8, 7) {}
                }
                start += 1;
                end -= 1;
            }

            start = pair.sensor.x - delta;
            end = pair.sensor.x + delta;

            for y in (pair.sensor.y - delta..=pair.sensor.y).rev() {
                for x in start..=end {
                    self.exhaustive_map.insert(Coord::from(x, y));
                    if pair.sensor == Coord::from(8, 7) {}
                }
                start += 1;
                end -= 1;
            }
        }
    }
}

fn main() {
    let file = std::fs::read_to_string("day15.txt").unwrap();

    let res = CaveMap::parse(&file);

    'outer: for i in 0..4_000_000 {
        let mut items = res.segment_intersection(i);

        loop {
            match items.next() {
                Some(range) => {
                    if *range.end() > 0 && *range.end() < 4_000_000 {
                        let freq: i128 = ((*range.end() as i128 + 1) * 4_000_000) + i as i128;
                        println!("tuning frequency is {freq}");
                        break 'outer;
                    }
                }
                None => break,
            }
        }
    }
}

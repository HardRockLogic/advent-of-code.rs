#![allow(dead_code, unused_imports)]

use std::{dbg, fs, println};

#[derive(Debug, Clone, Copy)]
struct Coords {
    x: u8,
    y: u8,
    z: u8,
}

impl Coords {
    fn parse(i: &str) -> Self {
        let x = i.split(",").nth(0).unwrap().parse::<u8>().unwrap();
        let y = i.split(",").nth(1).unwrap().parse::<u8>().unwrap();
        let z = i.split(",").nth(2).unwrap().parse::<u8>().unwrap();

        Self { x, y, z }
    }

    fn deltas(&self, other: Self) -> [i8; 3] {
        let dx = self.x as i8 - other.x as i8;
        let dy = self.y as i8 - other.y as i8;
        let dz = self.z as i8 - other.z as i8;

        [dx, dy, dz]
    }
}

#[derive(Debug, Default)]
struct Cube {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
    face: bool,
    back: bool,
}

impl Cube {
    fn count_vacant_sides(&self) -> u32 {
        let mut cout = 0;

        if !self.left {
            cout += 1;
        }
        if !self.right {
            cout += 1;
        }
        if !self.top {
            cout += 1;
        }
        if !self.bottom {
            cout += 1;
        }
        if !self.face {
            cout += 1;
        }
        if !self.back {
            cout += 1;
        }

        cout
    }
}

fn main() {
    let file = fs::read_to_string("day18.txt").unwrap();
    let cube_coords = file
        .lines()
        .map(|sub_line| Coords::parse(sub_line))
        .collect::<Vec<Coords>>();

    let mut poli_cube: Vec<Cube> = Vec::new();
    // println!("{cube_coords:#?}");

    for i in 0..cube_coords.len() {
        let mut cube = Cube::default();

        for j in 0..cube_coords.len() {
            let prime = cube_coords[i];
            let secondary = cube_coords[j];

            let deltas = prime.deltas(secondary);

            if deltas[0] == 1 && deltas[1] == 0 && deltas[2] == 0 {
                cube.left = true;
            } else if deltas[0] == -1 && deltas[1] == 0 && deltas[2] == 0 {
                cube.right = true;
            } else if deltas[0] == 0 && deltas[1] == 1 && deltas[2] == 0 {
                cube.bottom = true;
            } else if deltas[0] == 0 && deltas[1] == -1 && deltas[2] == 0 {
                cube.top = true;
            } else if deltas[0] == 0 && deltas[1] == 0 && deltas[2] == 1 {
                cube.back = true;
            } else if deltas[0] == 0 && deltas[1] == 0 && deltas[2] == -1 {
                cube.face = true;
            }
        }
        poli_cube.push(cube);
    }

    let mut counter: u32 = 0;

    for cube in poli_cube.into_iter() {
        counter += cube.count_vacant_sides();
    }

    println!("open sides: {counter}");
}

#![allow(dead_code, unused_imports)]

use std::{collections::VecDeque, dbg, fs, println};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
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

    fn from(x: u8, y: u8, z: u8) -> Self {
        Self { x, y, z }
    }

    fn get_neighbours_3d(&self) -> impl Iterator<Item = Coords> {
        let right_x = self.x + 1;
        let left_x: Option<u8> = if self.x != 0 { Some(self.x - 1) } else { None };
        let up_y = self.y + 1;
        let down_y: Option<u8> = if self.y != 0 { Some(self.y - 1) } else { None };
        let fornt_z = self.z + 1;
        let back_z: Option<u8> = if self.z != 0 { Some(self.z - 1) } else { None };

        let a = Self {
            x: right_x,
            y: self.y,
            z: self.z,
        };
        let b = Self {
            x: self.x,
            y: up_y,
            z: self.z,
        };
        let c = Self {
            x: self.x,
            y: self.y,
            z: fornt_z,
        };

        let mut output = vec![a, b, c];

        if let Some(coord) = left_x {
            let coords = Self {
                x: coord,
                y: self.y,
                z: self.z,
            };
            output.push(coords);
        }
        if let Some(coord) = down_y {
            let coords = Self {
                x: self.x,
                y: coord,
                z: self.z,
            };
            output.push(coords);
        }
        if let Some(coord) = back_z {
            let coords = Self {
                x: self.x,
                y: self.y,
                z: coord,
            };
            output.push(coords);
        }

        output.into_iter()
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Cube {
    left: bool,
    right: bool,
    top: bool,
    bottom: bool,
    face: bool,
    back: bool,
    is_surface_available: bool,
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
    let mut surface_only_coords: Vec<Coords> = Vec::new();

    for i in 0..cube_coords.len() {
        let mut cube = Cube::default();
        let prime = cube_coords[i];

        for j in 0..cube_coords.len() {
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
        if cube.count_vacant_sides() > 0 {
            surface_only_coords.push(prime);
        }
        poli_cube.push(cube);
    }

    let mut counter: u32 = 0;

    for cube in poli_cube.into_iter() {
        let tempo = cube.count_vacant_sides();
        counter += tempo;
    }

    println!("open sides: {counter}");
}

// fn flood_fill(surface: &[Coords]) -> u32 {
//     let mut max_x: u8 = 0;
//     let mut max_y: u8 = 0;
//     let mut max_z: u8 = 0;
//     let mut queue: VecDeque<Coords> = VecDeque::new();
//
//     for coord in surface.iter() {
//         if coord.x > max_x {
//             max_x = coord.x;
//         }
//         if coord.y > max_y {
//             max_y = coord.y;
//         }
//         if coord.z > max_z {
//             max_z = coord.z;
//         }
//     }
//
//     queue.push_back(Coords::from(0, 0, 0));
//
//     while !queue.is_empty() {
//         let liquid = queue.pop_front().unwrap();
//
//         for coord in liquid.get_neighbours_3d() {
//             if !queue.contains(&coord) {
//                 if let Some(cube) = surface.iter().find(|&&item| item == coord) {
//                     cube.is_surface_available = true;
//                 }
//             }
//         }
//     }
//
//     0
// }

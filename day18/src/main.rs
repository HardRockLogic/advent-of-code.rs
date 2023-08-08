#![allow(dead_code, unused_imports)]

use std::{collections::VecDeque, dbg, fs, println};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Coords {
    x: i8,
    y: i8,
    z: i8,
}

impl Coords {
    fn parse(i: &str) -> Self {
        let x = i.split(",").nth(0).unwrap().parse::<i8>().unwrap();
        let y = i.split(",").nth(1).unwrap().parse::<i8>().unwrap();
        let z = i.split(",").nth(2).unwrap().parse::<i8>().unwrap();

        Self { x, y, z }
    }

    fn deltas(&self, other: Self) -> [i8; 3] {
        let dx = self.x as i8 - other.x as i8;
        let dy = self.y as i8 - other.y as i8;
        let dz = self.z as i8 - other.z as i8;

        [dx, dy, dz]
    }

    fn from(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }

    fn get_neighbours_3d(&self, max_x: i8, max_y: i8, max_z: i8) -> impl Iterator<Item = Self> {
        let right_x: Option<i8> = if self.x < max_x + 1 {
            Some(self.x + 1)
        } else {
            None
        };
        let left_x: Option<i8> = if self.x > -1 { Some(self.x - 1) } else { None };

        let up_y: Option<i8> = if self.y < max_y + 1 {
            Some(self.y + 1)
        } else {
            None
        };
        let down_y: Option<i8> = if self.y > -1 { Some(self.y - 1) } else { None };

        let fornt_z: Option<i8> = if self.z < max_z + 1 {
            Some(self.z + 1)
        } else {
            None
        };
        let back_z: Option<i8> = if self.z > -1 { Some(self.z - 1) } else { None };

        let mut output: Vec<Self> = Vec::new();

        if let Some(coord) = right_x {
            let coords = Self {
                x: coord,
                y: self.y,
                z: self.z,
            };
            output.push(coords);
        }

        if let Some(coord) = up_y {
            let coords = Self {
                x: self.x,
                y: coord,
                z: self.z,
            };
            output.push(coords);
        }

        if let Some(coord) = fornt_z {
            let coords = Self {
                x: self.x,
                y: self.y,
                z: coord,
            };
            output.push(coords);
        }

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
    coordinates: Coords,
}

impl Cube {
    fn revert_to_default(&mut self) {
        self.left = false;
        self.right = false;
        self.top = false;
        self.bottom = false;
        self.face = false;
        self.back = false;
    }

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
    fn count_vacant_sides_positive(&self) -> u32 {
        let mut cout = 0;

        if self.left {
            cout += 1;
        }
        if self.right {
            cout += 1;
        }
        if self.top {
            cout += 1;
        }
        if self.bottom {
            cout += 1;
        }
        if self.face {
            cout += 1;
        }
        if self.back {
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
    let mut surface_only_coords: Vec<Cube> = Vec::new();

    for i in 0..cube_coords.len() {
        let mut cube = Cube::default();
        let prime = cube_coords[i];
        cube.coordinates = prime;

        for j in 0..cube_coords.len() {
            let secondary = cube_coords[j];

            let deltas = prime.deltas(secondary);

            // if deltas[0] == 1 && deltas[1] == 0 && deltas[2] == 0 {
            //     cube.left = true;
            // } else if deltas[0] == -1 && deltas[1] == 0 && deltas[2] == 0 {
            //     cube.right = true;
            // } else if deltas[0] == 0 && deltas[1] == 1 && deltas[2] == 0 {
            //     cube.bottom = true;
            // } else if deltas[0] == 0 && deltas[1] == -1 && deltas[2] == 0 {
            //     cube.top = true;
            // } else if deltas[0] == 0 && deltas[1] == 0 && deltas[2] == 1 {
            //     cube.back = true;
            // } else if deltas[0] == 0 && deltas[1] == 0 && deltas[2] == -1 {
            //     cube.face = true;
            // }
            check_adjecents(&mut cube, deltas);
        }
        if cube.count_vacant_sides() > 0 {
            surface_only_coords.push(cube);
        }
        poli_cube.push(cube);
    }

    let mut counter: u32 = 0;

    for item in poli_cube.iter_mut() {
        item.revert_to_default();
    }
    flood_fill(&mut poli_cube);

    for cube in poli_cube.into_iter() {
        counter += cube.count_vacant_sides_positive();
    }

    println!("surface available: {counter}");

    // for cube in poli_cube.into_iter() {
    //     let tempo = cube.count_vacant_sides();
    //     counter += tempo;
    // }
    //
    // println!("open sides: {counter}");
}

fn check_adjecents(cube: &mut Cube, deltas: [i8; 3]) {
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

fn flood_fill(surface: &mut [Cube]) {
    let mut max_x: i8 = 0;
    let mut max_y: i8 = 0;
    let mut max_z: i8 = 0;
    let mut queue: VecDeque<Coords> = VecDeque::new();
    let mut enqueued: Vec<Coords> = Vec::new();

    for coord in surface.iter() {
        if coord.coordinates.x > max_x {
            max_x = coord.coordinates.x;
        }
        if coord.coordinates.y > max_y {
            max_y = coord.coordinates.y;
        }
        if coord.coordinates.z > max_z {
            max_z = coord.coordinates.z;
        }
    }

    queue.push_back(Coords::from(0, 0, 0));
    enqueued.push(Coords::from(0, 0, 0));

    while !queue.is_empty() {
        let liquid = queue.pop_front().unwrap();

        for coord in liquid.get_neighbours_3d(max_x, max_y, max_z) {
            let deltas = coord.deltas(liquid);

            if !enqueued.contains(&coord) {
                if let Some(cube) = surface.iter_mut().find(|item| item.coordinates == coord) {
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
                } else {
                    queue.push_back(coord);
                    enqueued.push(coord);
                }
            }
        }
    }
}

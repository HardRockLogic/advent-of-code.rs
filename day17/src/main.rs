#![allow(dead_code, unused_imports)]
use std::{collections::HashSet, dbg, fmt::write, fs, panic, todo, unimplemented, unreachable};

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: u32,
    y: u32,
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}; y: {}", self.x, self.y)
    }
}

impl Coord {
    fn from(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Shapes {
    Start,
    Horizontal([Coord; 4]),
    Plus([Coord; 5]),
    ReversedL([Coord; 5]),
    Vertical([Coord; 4]),
    Cube([Coord; 4]),
}

impl Shapes {
    fn unwrap_data(&self) -> impl Iterator<Item = Coord> + '_ {
        match self {
            Self::Horizontal(arr) => arr.iter().copied(),
            Self::Plus(arr) => arr.iter().copied(),
            Self::ReversedL(arr) => arr.iter().copied(),
            Self::Vertical(arr) => arr.iter().copied(),
            Self::Cube(arr) => arr.iter().copied(),
            Self::Start => unimplemented!(),
        }
    }
    fn fill(&self, highest_y: u32) -> Self {
        match self {
            Self::Horizontal(_) => {
                let a = Coord::from(3, highest_y + 4);
                let b = Coord::from(4, highest_y + 4);
                let c = Coord::from(5, highest_y + 4);
                let d = Coord::from(6, highest_y + 4);

                Self::Horizontal([a, b, c, d])
            }
            Self::Plus(_) => {
                let a = Coord::from(4, highest_y + 6);
                let b = Coord::from(3, highest_y + 5);
                let c = Coord::from(4, highest_y + 5);
                let d = Coord::from(5, highest_y + 5);
                let e = Coord::from(4, highest_y + 4);

                Self::Plus([a, b, c, d, e])
            }
            Self::ReversedL(_) => {
                let a = Coord::from(5, highest_y + 6);
                let b = Coord::from(5, highest_y + 5);
                let c = Coord::from(3, highest_y + 4);
                let d = Coord::from(4, highest_y + 4);
                let e = Coord::from(5, highest_y + 4);

                Self::ReversedL([a, b, c, d, e])
            }
            Self::Vertical(_) => {
                let a = Coord::from(3, highest_y + 7);
                let b = Coord::from(3, highest_y + 6);
                let c = Coord::from(3, highest_y + 5);
                let d = Coord::from(3, highest_y + 4);

                Self::Vertical([a, b, c, d])
            }
            Self::Cube(_) => {
                let a = Coord::from(3, highest_y + 5);
                let b = Coord::from(4, highest_y + 5);
                let c = Coord::from(3, highest_y + 4);
                let d = Coord::from(4, highest_y + 4);

                Self::Cube([a, b, c, d])
            }
            Self::Start => unimplemented!(),
        }
    }

    fn left(&self) -> Self {
        match self {
            Self::Horizontal(arr) => {
                let mut new_position = [Coord::from(0, 0); 4];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x - 1;
                    new_position[i].y = arr[i].y;
                }
                Self::Horizontal(new_position)
            }
            Self::Plus(arr) => {
                let mut new_position = [Coord::from(0, 0); 5];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x - 1;
                    new_position[i].y = arr[i].y;
                }
                Self::Plus(new_position)
            }
            Self::ReversedL(arr) => {
                let mut new_position = [Coord::from(0, 0); 5];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x - 1;
                    new_position[i].y = arr[i].y;
                }
                Self::ReversedL(new_position)
            }
            Self::Vertical(arr) => {
                let mut new_position = [Coord::from(0, 0); 4];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x - 1;
                    new_position[i].y = arr[i].y;
                }
                Self::Vertical(new_position)
            }
            Self::Cube(arr) => {
                let mut new_position = [Coord::from(0, 0); 4];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x - 1;
                    new_position[i].y = arr[i].y;
                }
                Self::Cube(new_position)
            }
            Self::Start => unimplemented!(),
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::Horizontal(arr) => {
                let mut new_position = [Coord::from(0, 0); 4];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x + 1;
                    new_position[i].y = arr[i].y;
                }
                Self::Horizontal(new_position)
            }
            Self::Plus(arr) => {
                let mut new_position = [Coord::from(0, 0); 5];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x + 1;
                    new_position[i].y = arr[i].y;
                }
                Self::Plus(new_position)
            }
            Self::ReversedL(arr) => {
                let mut new_position = [Coord::from(0, 0); 5];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x + 1;
                    new_position[i].y = arr[i].y;
                }
                Self::ReversedL(new_position)
            }
            Self::Vertical(arr) => {
                let mut new_position = [Coord::from(0, 0); 4];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x + 1;
                    new_position[i].y = arr[i].y;
                }
                Self::Vertical(new_position)
            }
            Self::Cube(arr) => {
                let mut new_position = [Coord::from(0, 0); 4];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x + 1;
                    new_position[i].y = arr[i].y;
                }
                Self::Cube(new_position)
            }
            Self::Start => unimplemented!(),
        }
    }

    fn down(&self) -> Self {
        match self {
            Self::Horizontal(arr) => {
                let mut new_position = [Coord::from(0, 0); 4];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x;
                    new_position[i].y = arr[i].y - 1;
                }
                Self::Horizontal(new_position)
            }
            Self::Plus(arr) => {
                let mut new_position = [Coord::from(0, 0); 5];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x;
                    new_position[i].y = arr[i].y - 1;
                }
                Self::Plus(new_position)
            }
            Self::ReversedL(arr) => {
                let mut new_position = [Coord::from(0, 0); 5];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x;
                    new_position[i].y = arr[i].y - 1;
                }
                Self::ReversedL(new_position)
            }
            Self::Vertical(arr) => {
                let mut new_position = [Coord::from(0, 0); 4];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x;
                    new_position[i].y = arr[i].y - 1;
                }
                Self::Vertical(new_position)
            }
            Self::Cube(arr) => {
                let mut new_position = [Coord::from(0, 0); 4];
                for i in 0..arr.len() {
                    new_position[i].x = arr[i].x;
                    new_position[i].y = arr[i].y - 1;
                }
                Self::Cube(new_position)
            }
            Self::Start => unimplemented!(),
        }
    }
}

#[derive(PartialEq, Eq)]
struct Game {
    coord_storage: HashSet<Coord>,
    shape_state: Shapes,
    jets: Vec<char>,
    jet_state: usize,
    shape_coords: Shapes,
    next_step: Shapes,
}

impl Game {
    fn init(i: &str) -> Self {
        let jets = i.strip_suffix("\n").unwrap().chars().collect::<Vec<_>>();
        let coord_storage = HashSet::new();
        let shape_state = Shapes::Start;
        let shape_coords = Shapes::Start;
        let next_step = Shapes::Start;
        let jet_state = 0;

        Self {
            coord_storage,
            shape_state,
            jets,
            jet_state,
            shape_coords,
            next_step,
        }
    }

    fn spawn_shape(&mut self, highest_y: u32) {
        let coords_4 = [Coord::from(0, 0); 4];
        let coords_5 = [Coord::from(0, 0); 5];

        self.shape_state = match self.shape_state {
            Shapes::Start => Shapes::Horizontal(coords_4),
            Shapes::Horizontal(_) => Shapes::Plus(coords_5),
            Shapes::Plus(_) => Shapes::ReversedL(coords_5),
            Shapes::ReversedL(_) => Shapes::Vertical(coords_4),
            Shapes::Vertical(_) => Shapes::Cube(coords_4),
            Shapes::Cube(_) => Shapes::Horizontal(coords_4),
        };

        self.shape_coords = self.shape_state.fill(highest_y);
    }

    fn is_valid(&self) -> bool {
        for coord in self.next_step.unwrap_data() {
            if coord.x == 0 || coord.x == 8 || coord.y == 0 || self.coord_storage.contains(&coord) {
                return false;
            }
        }
        true
    }

    fn solidify(&mut self) -> u32 {
        for coord in self.shape_coords.unwrap_data() {
            self.coord_storage.insert(coord);
        }

        self.coord_storage
            .iter()
            .max_by_key(|coord| coord.y)
            .unwrap()
            .y
    }
}

impl Iterator for Game {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.jet_state < self.jets.len() {
            let item = self.jets[self.jet_state];
            self.jet_state += 1;
            Some(item)
        } else {
            self.jet_state = 1;
            if !self.jets.is_empty() {
                Some(self.jets[0])
            } else {
                None
            }
        }
    }
}

fn main() {
    let line = fs::read_to_string("day17.txt").unwrap();

    let mut tetris = Game::init(&line);
    tetris.spawn_shape(0);
    let mut counter = 0;

    loop {
        let turn = tetris.next().unwrap();
        tetris.next_step = match turn {
            '<' => tetris.shape_coords.left(),
            '>' => tetris.shape_coords.right(),
            _ => unreachable!(),
        };

        if tetris.is_valid() {
            tetris.shape_coords = tetris.next_step;
        }

        tetris.next_step = tetris.shape_coords.down();

        if tetris.is_valid() {
            tetris.shape_coords = tetris.next_step;
        } else {
            let highest = tetris.solidify();
            tetris.spawn_shape(highest);
            counter += 1;
            if counter == 2022 {
                println!("tower is {} units high", highest);
                break;
            }
        }
    }

    // let coords = [Coord::from(0, 0); 4];
    // let shape = Shapes::Cube(coords).fill(0);
    // dbg!(shape);

    // line.strip_suffix("\n")
    //     .unwrap()
    //     .chars()
    //     .for_each(|l| match l {
    //         '<' => println!("left jet"),
    //         '>' => println!("right jet"),
    //         _ => unreachable!(),
    //     })
}

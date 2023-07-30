#![allow(dead_code, unused_imports)]
use std::{collections::HashSet, dbg, fmt::write, fs, panic, todo, unimplemented};

#[derive(Hash, Clone, Copy)]
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

#[derive(Debug)]
enum Shapes {
    Start,
    Horizontal([Coord; 4]),
    Plus([Coord; 5]),
    ReversedL([Coord; 5]),
    Vertical([Coord; 4]),
    Cube([Coord; 4]),
}

impl Shapes {
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
}

struct Game {
    coord_storage: HashSet<Coord>,
    shape_state: Shapes,
    jets: Vec<char>,
    jet_state: usize,
}

impl Game {
    fn spawn_shape(&mut self, highest_y: u32) {
        let coords = [Coord::from(0, 0); 4];
        self.shape_state = match self.shape_state {
            Shapes::Start => Shapes::Horizontal(coords),
            _ => todo!(),
        }
    }
}

fn main() {
    let line = fs::read_to_string("test.txt").unwrap();

    let coords = [Coord::from(0, 0); 4];
    let shape = Shapes::Cube(coords).fill(0);
    dbg!(shape);

    // line.strip_suffix("\n")
    //     .unwrap()
    //     .chars()
    //     .for_each(|l| match l {
    //         '<' => println!("left jet"),
    //         '>' => println!("right jet"),
    //         _ => unreachable!(),
    //     })
}

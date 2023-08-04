#![allow(dead_code, unused_imports)]
use std::{
    collections::{HashMap, HashSet, VecDeque},
    dbg,
    fmt::write,
    fs, panic, println, todo, unimplemented, unreachable,
};

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: u64,
    y: u64,
}

impl std::fmt::Debug for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}; y: {}", self.x, self.y)
    }
}

impl Coord {
    fn from(x: u64, y: u64) -> Self {
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
    fn create(&self, highest_y: u64) -> Self {
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
    coord_storage: VecDeque<Coord>,
    shape_state: Shapes,
    shape_state_index: u64,
    jets: Vec<char>,
    jet_state: usize,
    shape_coords: Shapes,
    next_step: Shapes,
    move_window: bool,
}

impl Game {
    fn init(i: &str) -> Self {
        let jets = i.strip_suffix("\n").unwrap().chars().collect::<Vec<_>>();
        let coord_storage = VecDeque::new();
        let shape_state = Shapes::Start;
        let shape_state_index = 0;
        let shape_coords = Shapes::Start;
        let next_step = Shapes::Start;
        let jet_state = 0;
        let move_window = false;

        Self {
            coord_storage,
            shape_state,
            shape_state_index,
            jets,
            jet_state,
            shape_coords,
            next_step,
            move_window,
        }
    }

    fn spawn_shape(&mut self, highest_y: u64) {
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

        if self.shape_state_index < 5 {
            self.shape_state_index += 1;
        } else {
            self.shape_state_index = 1;
        }

        self.shape_coords = self.shape_state.create(highest_y);
    }

    fn left(&mut self) {
        self.next_step = self.shape_coords.left();
    }

    fn right(&mut self) {
        self.next_step = self.shape_coords.right();
    }

    fn down(&mut self) {
        self.next_step = self.shape_coords.down();
    }

    fn is_valid(&self) -> bool {
        for coord in self.next_step.unwrap_data() {
            if coord.x == 0 || coord.x == 8 || coord.y == 0 || self.coord_storage.contains(&coord) {
                return false;
            }
        }
        true
    }

    fn solidify(&mut self) -> u64 {
        if self.coord_storage.len() > 200 {
            self.move_window = true;
        }

        if self.move_window {
            for coord in self.shape_coords.unwrap_data() {
                self.coord_storage.push_back(coord);
                self.coord_storage.pop_front();
            }
        } else {
            for coord in self.shape_coords.unwrap_data() {
                self.coord_storage.push_back(coord);
            }
        }
        // for coord in self.shape_coords.unwrap_data() {
        //     self.coord_storage.push_back(coord);
        // }

        let highest = self
            .coord_storage
            .iter()
            .max_by_key(|coord| coord.y)
            .unwrap()
            .y;

        // let mut floor: Option<u64> = None;
        // let mut is_exist = true;
        //
        // 'outer: for y in (0..=highest).rev() {
        //     for x in 1..=7 {
        //         let tempo_coord = Coord::from(x, y);
        //         if !self.coord_storage.contains(&tempo_coord) {
        //             is_exist = false;
        //         }
        //         if x == 7 && is_exist {
        //             floor = Some(y);
        //             break 'outer;
        //         }
        //     }
        // }
        //
        // if let Some(floor_y) = floor {
        //     self.coord_storage.retain(|coord| coord.y >= floor_y);
        // }

        highest
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

    // let mut counter: u64 = 0;
    #[allow(unused_variables)]
    let part1_goal: u64 = 2022;
    let part2_goal: u64 = 1_000_000_000_000;

    let mut edge_shapes = HashMap::<[u64; 9], [u64; 2]>::new();
    let mut cycle_detected = false;
    let mut total_pieces: u64 = 0;
    let mut cycle_height: u64 = 0;
    let mut skipped_cycles: u64 = 0;

    while total_pieces < part2_goal {
        let turn = tetris.next().unwrap();
        // if total_pieces == 2048 {
        //     let one = tetris.coord_storage.contains(&Coord::from(2, 3179));
        //     println!("{one}");
        // }
        match turn {
            '<' => tetris.left(),
            '>' => tetris.right(),
            _ => unreachable!(),
        };

        if tetris.is_valid() {
            tetris.shape_coords = tetris.next_step;
        }

        tetris.down();

        // if total_pieces < 2080 && total_pieces > 1000 && total_pieces != 2048 {
        //     println!("{total_pieces}");
        // }
        // if total_pieces == 2048 {
        //     println!(
        //         "current: {:?} - next: {:?}",
        //         tetris.next_step, tetris.shape_coords
        //     );
        // }
        if tetris.is_valid() {
            tetris.shape_coords = tetris.next_step;
        } else {
            let highest = tetris.solidify();
            // if total_pieces == 2048 {
            //     println!(
            //         "faulted: {:?}\nsolidified: {:?}\nhighest: {highest}\n edge: {edge_state:?}",
            //         tetris.next_step, tetris.shape_coords
            //     );
            // }
            tetris.spawn_shape(highest);
            total_pieces += 1;

            if total_pieces == part2_goal {
                let answer = highest + (skipped_cycles * cycle_height);
                println!("tower is {} units high", answer);
                break;
            }

            // Detecting cycle
            if !cycle_detected {
                // 8th and 9th values are adtitional;
                // let mut edge_state: Vec<u64> = Vec::new();
                let mut edge_state: [u64; 9] = [0; 9];
                for i in 0..7 {
                    // edge_state.push(
                    //     tetris
                    //         .coord_storage
                    //         .iter()
                    //         .filter(|coord| coord.x as usize == i + 1)
                    //         .map(|coord| coord.y)
                    //         .max()
                    //         .unwrap_or_default(),
                    // )
                    edge_state[i] = tetris
                        .coord_storage
                        .iter()
                        .filter(|coord| coord.x as usize == i + 1)
                        .map(|coord| coord.y)
                        .max()
                        .unwrap_or_default();
                }

                let lowest = edge_state.iter().take(7).copied().min().unwrap();
                edge_state
                    .iter_mut()
                    .take(7)
                    .for_each(|edge| *edge -= lowest);
                // edge_state.extend([tetris.shape_state_index, tetris.jet_state as u64].into_iter());
                edge_state[7] = tetris.shape_state_index;
                edge_state[8] = tetris.jet_state as u64;

                if let Some(stored_data) = edge_shapes.get(&edge_state) {
                    cycle_height = highest - stored_data[0];
                    let pieces_in_cycle = total_pieces - stored_data[1];

                    println!("cycle detected on: {total_pieces} with {pieces_in_cycle} pieces");

                    skipped_cycles = (part2_goal - total_pieces) / pieces_in_cycle;
                    total_pieces += skipped_cycles * pieces_in_cycle;
                    cycle_detected = true;
                } else {
                    edge_shapes.insert(edge_state, [highest, total_pieces]);
                }
            }
        }
    }
}

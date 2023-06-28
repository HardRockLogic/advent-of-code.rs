#[derive(Debug)]
enum CellType {
    Start,
    End,
    Height(u8),
}

#[derive(Debug)]
struct Coord {
    // stands for (h)orisontal and (v)ertical
    h: usize,
    v: usize,
}

impl Coord {
    fn from(h: usize, v: usize) -> Self {
        Self { h, v }
    }
}

#[derive(Debug)]
struct Element {
    visited: bool,
    ancestor: Option<Vec<Coord>>,
    position: Coord,
    cell_type: CellType,
}

#[derive(Debug)]
struct Grid {
    m: usize,
    n: usize,
    start_coord: Coord,
    data: Vec<Vec<Element>>,
}

impl Grid {
    fn build(source: &str) -> Self {
        let m = source.lines().count();
        let n = source.lines().next().unwrap().len();
        let mut start_coord: Option<Coord> = None;
        let mut data = Vec::new();
        let mut h = 0;

        for line in source.lines() {
            let mut tempo = Vec::new();
            let mut v = 0;
            for chr in line.chars() {
                let cell_type = match chr {
                    'S' => {
                        start_coord = Some(Coord::from(h, v));
                        CellType::Start
                    }
                    'E' => CellType::End,
                    'a'..='z' => CellType::Height((chr as u8) - b'a'),
                    _ => panic!("unexpected input"),
                };
                let element = Element {
                    visited: false,
                    ancestor: None,
                    position: Coord::from(h, v),
                    cell_type: cell_type,
                };
                tempo.push(element);
                v += 1;
            }
            data.push(tempo);
            h += 1;
        }
        let start_coord = start_coord.unwrap();
        Self {
            m,
            n,
            start_coord,
            data,
        }
    }
}

use std::fs;

fn main() {
    let file = fs::read_to_string("test.txt").unwrap();

    let grid = Grid::build(&file);

    dbg!(grid);
}

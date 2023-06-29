#[derive(Debug)]
enum CellType {
    Start,
    End,
    Height(u8),
}

#[derive(Debug, Clone, Copy)]
struct Coord {
    // stands for (h)orisontal and (v)ertical
    h: usize,
    v: usize,
}

impl Coord {
    fn from(h: usize, v: usize) -> Self {
        Self { h, v }
    }
    fn h_v(&self) -> (usize, usize) {
        (self.h, self.v)
    }
}

#[derive(Debug)]
struct Element {
    visited: bool,
    enqueued: bool,
    ancestor: Option<Coord>,
    position: Coord,
    cell_type: CellType,
}

#[derive(Debug)]
struct Grid {
    m: usize,
    n: usize,
    start_coord: Coord,
    end_coord: Coord,
    data: Vec<Vec<Element>>,
}

impl Grid {
    fn build(source: &str) -> Self {
        let m = source.lines().count();
        let n = source.lines().next().unwrap().len();
        let mut start_coord: Option<Coord> = None;
        let mut end_coord: Option<Coord> = None;
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
                    'E' => {
                        end_coord = Some(Coord::from(h, v));
                        CellType::End
                    }
                    'a'..='z' => CellType::Height((chr as u8) - b'a'),
                    _ => panic!("unexpected input"),
                };
                let element = Element {
                    visited: false,
                    enqueued: false,
                    ancestor: None,
                    position: Coord::from(h, v),
                    cell_type,
                };
                tempo.push(element);
                v += 1;
            }
            data.push(tempo);
            h += 1;
        }
        let start_coord = start_coord.unwrap();
        let end_coord = end_coord.unwrap();
        Self {
            m,
            n,
            start_coord,
            end_coord,
            data,
        }
    }
}

use std::{collections::VecDeque, fs, println};

fn main() {
    let file = fs::read_to_string("day12.txt").unwrap();

    let grid = Grid::build(&file);

    // dbg!(grid);

    println!("{}", bfs(grid));
}

fn bfs(mut grid: Grid) -> u32 {
    let mut set = VecDeque::new();
    set.push_back(grid.start_coord);
    let mut cntr = 0;

    'outer: loop {
        cntr += 1;

        let err_mes = format!("iteration {}", cntr);
        let node = set.pop_front().expect(&err_mes);
        println!("iteration {cntr} on node ({}; {})", node.h, node.v);
        {
            grid.data[node.h][node.v].visited = true;
        }
        let node_height: u8 = match grid.data[node.h][node.v].cell_type {
            CellType::Start => 0,
            CellType::End => panic!("unexpected behaviour"),
            CellType::Height(n) => n,
        };
        'inner: for neighbour in neighbours(&node, grid.m, grid.n).iter() {
            // println!("n - {:?}", neighbour);
            let (h, v) = neighbour.h_v();

            let target = &mut grid.data[h][v];

            println!("({h}; {v}) before validation");

            match target.cell_type {
                CellType::Start => continue 'inner,
                CellType::End => {
                    if node_height + 1 >= 25 {
                        println!("gocha");
                        target.ancestor = Some(Coord::from(node.h, node.v));
                        break 'outer;
                    } else {
                        continue 'inner;
                    }
                }
                CellType::Height(altitude) => {
                    let diff: i8 = (node_height as i8) - (altitude as i8);
                    if diff >= -1 && !target.visited && !target.enqueued {
                        println!("({h}; {v}) after validation");
                        set.push_back(Coord::from(h, v));
                        target.enqueued = true;
                        target.ancestor = Some(Coord::from(node.h, node.v));
                    }
                }
            }
        }
    }
    let mut searcher = grid.end_coord;
    let mut counter: u32 = 0;
    loop {
        let (h, v) = searcher.h_v();
        counter += 1;
        // let parent = grid.data[h][v].ancestor.unwrap();

        match grid.data[h][v].cell_type {
            CellType::End => {
                searcher = grid.data[h][v].ancestor.unwrap();
            }
            CellType::Start => break,
            CellType::Height(_) => {
                searcher = grid.data[h][v].ancestor.unwrap();
            }
        }
    }
    counter - 1
}

fn neighbours(coord: &Coord, m: usize, n: usize) -> Vec<Coord> {
    let (h, v) = coord.h_v();
    let mut vec = Vec::new();

    if h != 0 {
        vec.push(Coord::from(h - 1, v));
    }
    if h < m - 1 {
        vec.push(Coord::from(h + 1, v));
    }
    if v != 0 {
        vec.push(Coord::from(h, v - 1));
    }
    if v < n - 1 {
        vec.push(Coord::from(h, v + 1));
    }
    vec
}

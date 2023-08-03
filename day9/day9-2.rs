use std::collections::HashSet;
use std::fs;

#[derive(Default, Debug, Eq, Hash, PartialEq)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn from(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Default, Debug)]
struct Rope {
    knots: [Coords; 10],
}

impl Rope {
    fn set_head(&mut self, coords: Coords) {
        self.knots[0] = coords;
    }

    fn head(&self) -> Coords {
        Coords {
            x: self.knots[0].x,
            y: self.knots[0].y,
        }
    }

    fn tail(&self) -> Coords {
        Coords {
            x: self.knots[self.knots.len() - 1].x,
            y: self.knots[self.knots.len() - 1].y,
        }
    }

    fn diff(&self, i: usize) -> [i32; 2] {
        let x = self.knots[i - 1].x - self.knots[i].x;
        let y = self.knots[i - 1].y - self.knots[i].y;

        [x, y]
    }

    fn chain_reaction(&mut self) {
        for i in 1..self.knots.len() {
            let [x, y]: [i32; 2] = self.diff(i);

            if x.abs() + y.abs() == 3 {
                self.knots[i].x += sgn(x);
                self.knots[i].y += sgn(y);
            } else {
                self.knots[i].x += transform(x);
                self.knots[i].y += transform(y);
            }
        }
    }
}

fn main() {
    let file = fs::read_to_string("day9.txt").expect("no such file");
    let mut set = HashSet::new();

    let mut rope = Rope::default();

    for line in file.lines() {
        let direction = line.split_whitespace().nth(0).unwrap();
        let steps = line
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i32>()
            .unwrap();

        match direction {
            "R" => {
                for _ in 0..steps {
                    rope.set_head(Coords::from(rope.head().x + 1, rope.head().y));
                    rope.chain_reaction();
                    set.insert(rope.tail());
                }
            }
            "L" => {
                for _ in 0..steps {
                    rope.set_head(Coords::from(rope.head().x - 1, rope.head().y));
                    rope.chain_reaction();
                    set.insert(rope.tail());
                }
            }
            "U" => {
                for _ in 0..steps {
                    rope.set_head(Coords::from(rope.head().x, rope.head().y + 1));
                    rope.chain_reaction();
                    set.insert(rope.tail());
                }
            }
            "D" => {
                for _ in 0..steps {
                    rope.set_head(Coords::from(rope.head().x, rope.head().y - 1));
                    rope.chain_reaction();
                    set.insert(rope.tail());
                }
            }
            _ => {}
        }
    }

    println!("the len is: {}", set.len());
}
fn transform(x: i32) -> i32 {
    sgn(x) * (x.abs() - 1)
}

fn sgn(x: i32) -> i32 {
    if x < 0 {
        -1
    } else if x > 0 {
        1
    } else {
        0
    }
}

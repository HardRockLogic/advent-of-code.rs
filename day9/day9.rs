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
    head: Coords,
    tail: Coords,
}

impl Rope {
    fn set_head(&mut self, coords: Coords) {
        self.head = coords;
    }

    fn adjust_tail(&mut self) {
        let diff: [i32; 2] = self.diff();

        match diff {
            // Moves right
            [2, 0] => {
                self.tail.x += 1;
            }
            // Moves left
            [-2, 0] => {
                self.tail.x -= 1;
            }
            // Moves up
            [0, 2] => {
                self.tail.y += 1;
            }
            // Moves down
            [0, -2] => {
                self.tail.y -= 1;
            }
            // Moves up and right
            [1, 2] | [2, 1] => {
                self.tail.x += 1;
                self.tail.y += 1;
            }
            // Moves down an left
            [-1, -2] | [-2, -1] => {
                self.tail.x -= 1;
                self.tail.y -= 1;
            }
            // Moves left and up
            [-1, 2] | [-2, 1] => {
                self.tail.x -= 1;
                self.tail.y += 1;
            }
            // Moves right and down
            [1, -2] | [2, -1] => {
                self.tail.x += 1;
                self.tail.y -= 1;
            }
            _ => {}
        }
    }

    fn head(&self) -> Coords {
        Coords {
            x: self.head.x,
            y: self.head.y,
        }
    }

    fn tail(&self) -> Coords {
        Coords {
            x: self.tail.x,
            y: self.tail.y,
        }
    }

    fn diff(&self) -> [i32; 2] {
        let x = self.head.x - self.tail.x;
        let y = self.head.y - self.tail.y;

        [x, y]
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
                    rope.adjust_tail();
                    set.insert(rope.tail());
                }
            }
            "L" => {
                for _ in 0..steps {
                    rope.set_head(Coords::from(rope.head().x - 1, rope.head().y));
                    rope.adjust_tail();
                    set.insert(rope.tail());
                }
            }
            "U" => {
                for _ in 0..steps {
                    rope.set_head(Coords::from(rope.head().x, rope.head().y + 1));
                    rope.adjust_tail();
                    set.insert(rope.tail());
                }
            }
            "D" => {
                for _ in 0..steps {
                    rope.set_head(Coords::from(rope.head().x, rope.head().y - 1));
                    rope.adjust_tail();
                    set.insert(rope.tail());
                }
            }
            _ => {}
        }
    }

    println!("the len is: {}", set.len());
}

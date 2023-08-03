use std::{dbg, fmt, vec};
#[derive(Clone, PartialEq, Eq)]
enum Packet {
    Integer(u8),
    Vector(Vec<Packet>),
}

impl Packet {
    fn into_slice<T>(&self, f: impl FnOnce(&[Packet]) -> T) -> T {
        match self {
            Self::Integer(u) => f(&[Self::Integer(*u)]),
            Self::Vector(v) => f(&v[..]),
        }
    }
} // find firs occurance

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Integer(a), Packet::Integer(b)) => a.partial_cmp(b),
            (left, right) => {
                left.into_slice(|left| right.into_slice(|right| left.partial_cmp(right)))
            }
        }
    }
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Debug for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Packet::Integer(i) => write!(f, "{}", i),
            Packet::Vector(v) => {
                write!(f, "[")?;
                for (i, packet) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", packet)?;
                }
                write!(f, "]")
            }
        }
    }
}

use std::fs;

fn main() {
    let file = fs::read_to_string("day13.txt").unwrap();

    let deviders = vec![
        vec![Packet::Vector(vec![Packet::Integer(2)])],
        vec![Packet::Vector(vec![Packet::Integer(6)])],
    ];

    let mut distress_packets = file
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| deserialize(&line[1..]))
        .chain(deviders.iter().cloned())
        .collect::<Vec<_>>();

    distress_packets.sort();

    let decoder_key = deviders
        .iter()
        .map(|d| distress_packets.binary_search(d).unwrap() + 1)
        .product::<usize>();

    dbg!(decoder_key);
}

fn deserialize(input: &str) -> Vec<Packet> {
    let mut output: Vec<Packet> = Vec::new();
    let mut issued = true;
    let mut o_br = 1; // stands for open bracket
    let mut c_br = 0; // stands for closed bracket
    let mut allowed_counter = 0;
    let mut acc: Option<char> = None;

    for (i, chr) in input.chars().enumerate() {
        if allowed_counter == 0 {
            o_br = 1;
            c_br = 0;
            issued = true;
        }

        match chr {
            '[' => {
                if issued {
                    'inner: for bracket in input[i + 1..].chars() {
                        if bracket == '[' {
                            o_br += 1;
                        } else if bracket == ']' {
                            c_br += 1;
                        } else {
                            continue 'inner;
                        }
                        if o_br == c_br {
                            allowed_counter = o_br;
                            // println!("it calls! {}", allowed_counter);
                            break 'inner;
                        }
                    }
                    issued = false;
                    output.push(Packet::Vector(deserialize(&input[i + 1..])));
                }
            }
            '0'..='9' => {
                if issued {
                    let mut s = String::new();
                    match acc {
                        Some(d) => {
                            s.push(d);
                            output.pop();
                        }
                        None => acc = Some(chr),
                    }
                    s.push(chr);
                    output.push(Packet::Integer(s.parse::<u8>().unwrap()));
                }
            }
            ',' => {
                acc = None;
                continue;
            }
            ']' => {
                if issued {
                    return output;
                }
                allowed_counter -= 1;
            }
            _ => unreachable!(),
        }
    }
    output
}

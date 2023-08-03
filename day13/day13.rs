use std::{fmt, vec};
#[derive(Clone, Eq, PartialEq)]
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
    // find firs occurance
    fn ffo(&self, u: u8, len: usize) -> Option<bool> {
        match self {
            Self::Integer(i) => {
                if u < *i {
                    return Some(true);
                } else if u > *i {
                    return Some(false);
                } else {
                    if len > 1 {
                        return Some(true);
                    } else {
                        return None;
                    }
                }
            }
            Self::Vector(v) => {
                if v.is_empty() {
                    return Some(false);
                } else {
                    return v[0].ffo(u, v.len());
                }
            }
        }
    }
}

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

use std::borrow::Cow;
struct Compare<'a> {
    left: Cow<'a, Vec<Packet>>,
    right: Cow<'a, Vec<Packet>>,
}

impl<'a> Compare<'a> {
    fn compare(&self) -> Option<bool> {
        let mut return_value = Some(true);
        let right_len = self.right.len();
        let left_len = self.left.len();

        if left_len == right_len {
            return_value = None;
        }

        for idx in 0..left_len {
            if right_len == 0 && left_len > 0 {
                return Some(false);
            }
            if idx > right_len - 1 {
                return Some(false);
            }
            match &self.left[idx] {
                Packet::Integer(left_u) => match &self.right[idx] {
                    Packet::Integer(right_u) => {
                        if left_u < right_u {
                            return Some(true);
                        } else if left_u > right_u {
                            return Some(false);
                        } else {
                            continue;
                        }
                    }
                    Packet::Vector(right_v) => match self.right[idx].ffo(*left_u, right_v.len()) {
                        Some(bolean) => return Some(bolean),
                        None => continue,
                    },
                },
                Packet::Vector(left_v) => match &self.right[idx] {
                    Packet::Integer(right_u) => match self.left[idx].ffo(*right_u, left_v.len()) {
                        Some(bolean) => return Some(!bolean),
                        None => continue,
                    },

                    Packet::Vector(right_v) => {
                        let recursive_cmp = Compare {
                            left: Cow::Borrowed(left_v),
                            right: Cow::Borrowed(right_v),
                        };
                        match recursive_cmp.compare() {
                            Some(bolean) => return Some(bolean),
                            None => continue,
                        }
                    }
                },
            }
        }
        // println!("reaches");
        return_value
    }
}

use std::fs;

fn main() {
    let start = std::time::Instant::now();

    let file = fs::read_to_string("day13.txt").unwrap();
    let mut counter = 1;
    let mut index: usize = 1;
    let mut ind_list: Vec<usize> = Vec::new();
    let mut cmp = Compare {
        left: Cow::Owned(vec![]),
        right: Cow::Owned(vec![]),
    };

    for line in file.lines() {
        if counter == 1 {
            cmp.left = Cow::Owned(deserialize(&line[1..]));
        } else if counter == 2 {
            cmp.right = Cow::Owned(deserialize(&line[1..]));
        }
        counter += 1;

        if line.is_empty() {
            counter = 1;
            if cmp.compare().unwrap() {
                ind_list.push(index);
            }
            index += 1;
        }
    }
    let total: usize = ind_list.iter().sum();
    let elapsed = start.elapsed();

    println!("{} - in {:?}", total, elapsed);
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

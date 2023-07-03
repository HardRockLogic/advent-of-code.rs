use std::fmt;
enum Packet {
    Integer(u8),
    Vector(Vec<Packet>),
}

impl Packet {
    fn transform(&self) -> Self {
        match self {
            Self::Integer(u) => Self::Vector(vec![Self::Integer(*u)]),
            Self::Vector(_) => panic!("Shouldnt encounter Vector"),
        }
    }
    // find firs occurance
    fn ffo(&self, u: u8, len: usize) -> Option<bool> {
        // println!("left {}u8", u);
        match self {
            Self::Integer(i) => {
                // println!("right {}u8", i);
                if u < *i {
                    return Some(true);
                } else if u > *i {
                    // println!("should work");
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

    fn vector_cmp(&self, other: &Vec<Packet>) -> Option<bool> {
        match self {
            Self::Integer(_) => panic!("for Self::Vector comparison only"),
            Self::Vector(left_vec) => {
                for idx in 0..left_vec.len() {
                    match &left_vec[idx] {
                        Packet::Integer(left_u) => match &other[idx] {
                            Packet::Integer(right_u) => {
                                if *left_u < *right_u {
                                    return Some(true);
                                } else if *left_u > *right_u {
                                    return Some(false);
                                } else {
                                    continue;
                                };
                            }
                            Packet::Vector(right_v) => {
                                match other[idx].ffo(*left_u, right_v.len()) {
                                    Some(bolean) => return Some(bolean),
                                    None => continue,
                                }
                            }
                        },
                        Packet::Vector(left_v) => match &other[idx] {
                            Packet::Integer(right_u) => {
                                match left_vec[idx].ffo(*right_u, left_v.len()) {
                                    Some(bolean) => return Some(!bolean),
                                    None => continue,
                                }
                            }
                            Packet::Vector(right_v) => match left_vec[idx].vector_cmp(right_v) {
                                Some(bolean) => return Some(bolean),
                                None => continue,
                            },
                        },
                    }
                }
            }
        }
        None
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

struct Compare {
    left: Vec<Packet>,
    right: Vec<Packet>,
}

impl Compare {
    fn compare(&self) -> bool {
        let right_len = self.right.len();

        for idx in 0..self.left.len() {
            if idx > right_len - 1 {
                return false;
            }
            match &self.left[idx] {
                Packet::Integer(left_u) => match &self.right[idx] {
                    Packet::Integer(right_u) => {
                        if left_u < right_u {
                            return true;
                        } else if left_u > right_u {
                            return false;
                        } else {
                            continue;
                        }
                    }
                    Packet::Vector(right_v) => match self.right[idx].ffo(*left_u, right_v.len()) {
                        Some(bolean) => return bolean,
                        None => continue,
                    },
                },
                Packet::Vector(left_v) => match &self.right[idx] {
                    Packet::Integer(right_u) => match self.left[idx].ffo(*right_u, left_v.len()) {
                        Some(bolean) => return !bolean,
                        None => continue,
                    },

                    Packet::Vector(right_v) => match self.left[idx].vector_cmp(right_v) {
                        Some(bolean) => return bolean,
                        None => continue,
                    },
                },
            }
        }
        // println!("reaches");
        true
    }
}

use std::fs;
use std::println;
use std::todo;

fn main() {
    let s1 = "[9]";
    let s2 = "[[[9,7,6]]]";

    let a1 = "[]";
    let a2 = "[3]";

    let cmp = Compare {
        left: filling(&s2[1..]),
        right: filling(&s1[1..]),
    };
    println!("{}", cmp.compare());

    // let file = fs::read_to_string("test.txt").unwrap();
    //
    // for line in file.lines() {
    //     if !line.is_empty() {
    //         println!("{:?}", filling(&line[1..]));
    //     } else {
    //         continue;
    //     }
    // }
}

fn filling(input: &str) -> Vec<Packet> {
    let mut output: Vec<Packet> = Vec::new();
    let mut truncate_str = input.to_string();
    let mut issued = true;
    let mut o_br = 1; // stands for open bracket
    let mut c_br = 0; // stands for closed bracket
    let mut allowed_counter = 0;
    let mut acc: Option<char> = None;

    for chr in input.chars() {
        truncate_str.remove(0);

        if allowed_counter == 0 {
            o_br = 1;
            c_br = 0;
            issued = true;
        }

        match chr {
            '[' => {
                if issued {
                    'inner: for bracket in truncate_str.chars() {
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
                    // println!("{truncate_str}");
                    output.push(Packet::Vector(filling(&truncate_str)));
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
                // issued = true;
                allowed_counter -= 1;
            }
            _ => unreachable!(),
        }
    }
    output
}

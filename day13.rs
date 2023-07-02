use std::fmt;
enum Packet {
    Integer(u8),
    Vector(Vec<Packet>),
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

fn main() {
    let s1 = "1,[2,[3,[4,[5,6,7]]]],8,9]";
    let s4 = "[[[3,9,5],[4,0,3],[1,2,0],0],9],[[],[[0,0,5]]],[2,[[6,3,4],[10,9]],3,4,8],[]]";
    let res1 = filling(s1);
    let res4 = filling(s4);
    println!("{:?}", res1);
    println!("{:?}", res4);
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

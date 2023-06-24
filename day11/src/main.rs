use nom::Finish;

mod parser;

#[allow(unused_imports)]
use parser::{parse_all_monkeys, parse_monkey, Monkey};

fn main() {
    let monkeys = parse_all_monkeys(concat!(include_str!("../../day11.txt"), "\n"))
        .finish()
        .unwrap()
        .1;

    // match monkeys {
    //     Ok((_, res)) => println!("{res:?}"),
    //     Err(err) => println!("{err:?}"),
    // }

    dbg!(monkeys);
    // for monkey in &monkeys {
    //     println!("{:?}", monkey);
    // }
}

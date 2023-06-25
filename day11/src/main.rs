use nom::Finish;

mod parser;

#[allow(unused_imports)]
use parser::{parse_all_monkeys, parse_monkey, Monkey};

fn main() {
    let monkeys = parse_all_monkeys(concat!(include_str!("../../day11.txt"), "\n"))
        .finish()
        .unwrap()
        .1;

    // dbg!(monkeys);
}

fn one_round(entity: &mut Vec<Monkey>) {
    let number_monkeys = entity.len();

    for i in 0..number_monkeys {
        let mc;

        {
            let monkey = &mut entity[i];
            mc = monkey.clone();
            monkey.inspected_items += mc.items_vec.len() as u64;
        }

        for mut item in mc.items_vec.iter().copied() {
            item = mc.operation.eval(item);
            item /= 3;
            if item % mc.divisor == 0 {
                entity[mc.pass_if_true].items_vec.push(item);
            } else {
                entity[mc.pass_if_false].items_vec.push(item);
            }
        }
        entity[i].items_vec.clear();
    }
}

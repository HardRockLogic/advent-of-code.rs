use nom::Finish;

mod parser;

use parser::{parse_all_monkeys, Monkey};

fn main() {
    let mut monkeys = parse_all_monkeys(concat!(include_str!("../../day11.txt"), "\n"))
        .finish()
        .unwrap()
        .1;

    for _ in 0..10_000 {
        one_round(&mut monkeys);
    }

    let mut top_collectors = monkeys
        .iter()
        .map(|m| m.inspected_items)
        .collect::<Vec<_>>();
    top_collectors.sort_by_key(|&m| std::cmp::Reverse(m));

    let total = top_collectors.into_iter().take(2).product::<u64>();
    dbg!(total);
}

fn one_round(entity: &mut Vec<Monkey>) {
    let number_monkeys = entity.len();
    let divisor_prod = entity.iter().map(|m| m.divisor).product::<u64>();

    for i in 0..number_monkeys {
        let mc;

        {
            let monkey = &mut entity[i];
            mc = monkey.clone();
            monkey.inspected_items += mc.items_vec.len() as u64;
        }

        for mut item in mc.items_vec.iter().copied() {
            item %= divisor_prod;
            item = mc.operation.eval(item);
            // item /= 3; // for the first part only
            if item % mc.divisor == 0 {
                entity[mc.pass_if_true].items_vec.push(item);
            } else {
                entity[mc.pass_if_false].items_vec.push(item);
            }
        }
        entity[i].items_vec.clear();
    }
}

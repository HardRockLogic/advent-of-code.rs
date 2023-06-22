#[derive(Debug)]
pub struct Monkey {
    items: Vec<u64>,
    inspected_items: u64,
    operation: Operation,
    divisor: u64,
    pass_if_true: usize,
    pass_if_false: usize,
}

#[derive(Debug)]
pub enum Operation {
    Add(Statemant, Statemant),
    Mul(Statemant, Statemant),
}

impl Operation {
    pub fn eval_operation(self, old: u64) -> u64 {
        match self {
            Operation::Add(left, right) => left.eval_state(old) + right.eval_state(old),
            Operation::Mul(left, right) => left.eval_state(old) * right.eval_state(old),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Statemant {
    Old,
    Constant(u64),
}

impl Statemant {
    pub fn eval_state(self, old: u64) -> u64 {
        match self {
            Statemant::Old => old,
            Statemant::Constant(c) => c,
        }
    }
}

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete as cmplt;
use nom::character::complete::{one_of, space1};
use nom::combinator::{map, value};
use nom::multi::separated_list0;
use nom::sequence::{preceded, tuple};
use nom::IResult;

fn parse(input: &str) -> IResult<&str, &str> {
    preceded(tag("Starting items: "), take_till(|c| c == '\n'))(input)
}

pub fn list(input: &str) -> IResult<&str, Vec<u64>> {
    let (_, result) = parse(input)?;
    separated_list0(tag(", "), cmplt::u64)(result)
}

pub fn parse_statemant(i: &str) -> IResult<&str, Statemant> {
    alt((
        value(Statemant::Old, tag("old")),
        map(cmplt::u64, Statemant::Constant),
    ))(i)
}

pub fn parse_operation(i: &str) -> IResult<&str, Operation> {
    let (i, (left, _, operand, _, right)) = preceded(
        tag("new = "),
        tuple((
            parse_statemant,
            space1,
            one_of("*+"),
            space1,
            parse_statemant,
        )),
    )(i)?;
    let operand = match operand {
        '*' => Operation::Mul(left, right),
        '+' => Operation::Add(left, right),
        _ => unreachable!(),
    };
    Ok((i, operand))
}

pub fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
    unimplemented!()
}

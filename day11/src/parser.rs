#[allow(dead_code)]
#[derive(Debug)]
pub struct Monkey {
    items_vec: Vec<u64>,
    inspected_items: u64,
    operation: Operation,
    divisor: u64,
    pass_if_true: usize,
    pass_if_false: usize,
}

#[derive(Debug)]
#[allow(dead_code)]
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
#[allow(dead_code)]
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
#[allow(unused_imports)]
use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::character::complete as cmplt;
use nom::character::complete::{multispace1, one_of, space1};
use nom::combinator::{eof, map, value};
use nom::error::{ParseError, VerboseError};
use nom::multi::separated_list1;
use nom::sequence::{preceded, tuple};
use nom::IResult;
use nom_supreme::tag::complete::tag;

use nom_supreme::error::ErrorTree;

pub fn parse_statemant(i: &str) -> IResult<&str, Statemant, ErrorTree<&str>> {
    alt((
        value(Statemant::Old, tag("old")),
        map(cmplt::u64, Statemant::Constant),
    ))(i)
}

pub fn parse_operation(i: &str) -> IResult<&str, Operation, ErrorTree<&str>> {
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

pub fn parse_monkey(i: &str) -> IResult<&str, Monkey, ErrorTree<&str>> {
    let (i, _) = tuple((tag("Monkey "), cmplt::u64, tag(":\n")))(i)?;

    let (i, (_, _, items_vec, _)) = tuple((
        space1,
        tag("Starting items: "),
        separated_list1(tag(", "), cmplt::u64),
        tag("\n"),
    ))(i)?;

    let (i, (_, _, operation, _)) =
        tuple((space1, tag("Operation: "), parse_operation, tag("\n")))(i)?;

    let (i, (_, _, divisor, _)) =
        tuple((space1, tag("Test: divisible by "), cmplt::u64, tag("\n")))(i)?;

    let (i, (_, _, pass_if_true, _)) = tuple((
        space1,
        tag("If true: throw to monkey "),
        map(cmplt::u64, |x| x as usize),
        tag("\n"),
    ))(i)?;

    let (i, (_, _, pass_if_false, _)) = tuple((
        space1,
        tag("If false: throw to monkey "),
        map(cmplt::u64, |x| x as usize),
        tag("\n"),
    ))(i)?;

    Ok((
        i,
        Monkey {
            inspected_items: 0,
            items_vec,
            operation,
            divisor,
            pass_if_true,
            pass_if_false,
        },
    ))
}

pub fn parse_all_monkeys(i: &str) -> IResult<&str, Vec<Monkey>, ErrorTree<&str>> {
    separated_list1(tag("\n"), parse_monkey)(i)
}

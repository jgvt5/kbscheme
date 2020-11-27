use std::fmt::Debug;
use std::num::ParseIntError;

use nom::{
  branch::{alt, permutation},
  bytes::complete::{tag, take},
  character::complete::{digit1, multispace0, one_of},
  combinator::{map, map_res, opt, value},
  sequence::{delimited, preceded, separated_pair},
  IResult, Parser,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Const {
  Integer(i32),
  Boolean(bool),
  Nil,
}

pub enum Expr {
  //Const(Box<Const>),
}

fn boolean(input: &str) -> IResult<&str, bool> {
  alt((value(false, tag("#f")), value(true, tag("#t"))))(input)
}

fn integer(input: &str) -> IResult<&str, i32> {
  map_res(
    permutation((opt(one_of("+-")), digit1)),
    |(sign, int): (Option<char>, &str)| -> Result<i32, ParseIntError> {
      let sign: i32 = if sign.unwrap_or('+') == '+' { 1 } else { -1 };
      let int: i32 = int.parse::<i32>()?;
      Ok(sign * int)
    },
  )(input)
}

fn const_value(input: &str) -> IResult<&str, Const> {
  use Const::*;
  alt((
    value(Nil, tag("'()")),
    map(boolean, Boolean),
    map(integer, Integer),
  ))(input)
}

#[test]
fn const_test() {
  println!("{:?}", const_value("-114514"));
}

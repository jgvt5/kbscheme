use nom::{Err, IResult};
use nom::character::complete::{char, digit1, multispace0};
use nom::error::ErrorKind;

pub fn sexpr(s: &str) -> IResult<&str, &str> {
  let (s, _) = char('(')(s)?;
  let (s, _) = multispace0(s)?;
  let (s, num) = digit1(s)?;
  let (s, _) = multispace0(s)?;
  let (s, _) = char(')')(s)?;
  Ok((s, num))
}
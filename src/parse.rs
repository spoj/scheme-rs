use crate::lisp::{Atom, Sexp};
use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, multispace0};
use nom::combinator::map;
use nom::combinator::verify;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::{IResult, character::complete::isize};

// Parsing
fn number(input: &str) -> IResult<&str, isize> {
    isize.parse(input)
}

fn symbol(input: &str) -> IResult<&str, &str> {
    verify(alphanumeric1, |s: &str| {
        s.chars().next().is_some_and(|c| c.is_alphabetic())
    })
    .parse(input)
}
pub fn sexp(input: &str) -> IResult<&str, Sexp> {
    delimited(
        multispace0,
        alt((map(atom, Sexp::Atom), map(list, Sexp::List))),
        multispace0,
    )
    .parse(input)
}

pub fn program(input: &str) -> IResult<&str, Vec<Sexp>> {
    many0(sexp).parse(input)
}

fn list(input: &str) -> IResult<&str, Vec<Sexp>> {
    delimited(tag("("), many0(sexp), tag(")")).parse(input)
}
fn atom(input: &str) -> IResult<&str, Atom> {
    alt((
        map(number, Atom::Number),
        map(symbol, |n| Atom::Symbol(n.to_string())),
    ))
    .parse(input)
}

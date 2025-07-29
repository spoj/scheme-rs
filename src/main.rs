use nom::Parser;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, space1};
use nom::combinator::map;
use nom::combinator::verify;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::{IResult, character::complete::isize};

fn number(input: &str) -> IResult<&str, isize> {
    isize.parse(input)
}

fn symbol(input: &str) -> IResult<&str, &str> {
    verify(alphanumeric1, |s: &str| {
        s.chars().next().is_some_and(|c| c.is_alphabetic())
    })
    .parse(input)
}
fn sexp(input: &str) -> IResult<&str, Sexp> {
    alt((map(atom, Sexp::Atom), map(list, Sexp::List))).parse(input)
}

fn list(input: &str) -> IResult<&str, Vec<Sexp>> {
    delimited(tag("("), separated_list0(space1, sexp), tag(")")).parse(input)
}
fn atom(input: &str) -> IResult<&str, Atom> {
    alt((
        map(number, Atom::Number),
        map(symbol, |n| Atom::Symbol(n.to_string())),
    ))
    .parse(input)
}

#[allow(unused)]
#[derive(Debug)]
enum Sexp {
    Atom(Atom),
    List(Vec<Sexp>),
}

#[allow(unused)]
#[derive(Debug)]
enum Atom {
    Number(isize),
    Symbol(String),
}

fn main() {
    let input = "(3 (2 3 -34 3) +5 ()) ";
    let _ = dbg!(sexp(input));
}

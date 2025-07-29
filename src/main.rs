use std::collections::HashMap;

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
pub fn sexp(input: &str) -> IResult<&str, Sexp> {
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
pub enum Sexp {
    Atom(Atom),
    List(Vec<Sexp>),
}

impl Sexp {
    fn as_atom(&self) -> Option<&Atom> {
        match self {
            Sexp::Atom(atom) => Some(atom),
            Sexp::List(_) => None,
        }
    }
}

impl Sexp {
    pub fn eval(&self, env: HashMap<String, Value>) -> Option<Value> {
        match self {
            Sexp::Atom(Atom::Number(n)) => Some(Value::Number(*n)),
            Sexp::Atom(Atom::Symbol(n)) => env.get(n).cloned(),
            Sexp::List(sexps) if sexps.is_empty() => None,
            Sexp::List(sexps)
                if sexps
                    .first()
                    .and_then(|s| s.as_atom())
                    .and_then(|a| a.as_atom_str())
                    == Some("add") =>
            {
                let res: isize = sexps[1..]
                    .iter()
                    .flat_map(|s| s.as_atom().and_then(|a| a.as_number()))
                    .sum();
                Some(Value::Number(res))
            }
            _ => unreachable!(),
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub enum Atom {
    Number(isize),
    Symbol(String),
}

impl Atom {
    fn as_atom_str(&self) -> Option<&str> {
        match self {
            Atom::Number(_) => None,
            Atom::Symbol(n) => Some(n),
        }
    }
    fn as_number(&self) -> Option<isize> {
        match self {
            Atom::Number(n) => Some(*n),
            Atom::Symbol(_) => None,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone)]
pub enum Value {
    Number(isize),
}

fn main() {
    let input = "(3 (2 3 -34 3) +5 ()) ";
    let _ = dbg!(sexp(input));
    let input = "(add 1 2 3 4 5 6)";
    let _ = dbg!(sexp(input));
    let result = sexp(input).unwrap().1.eval(Default::default());
    dbg!(result);
}

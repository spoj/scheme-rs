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
#[derive(Debug, Clone, PartialEq)]
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
    fn as_list(&self) -> Option<Vec<Sexp>> {
        match self {
            Sexp::Atom(_) => None,
            Sexp::List(v) => Some(v.clone()),
        }
    }
}

impl Sexp {
    pub fn eval(&self, env: &HashMap<String, Value>) -> Option<Value> {
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
                    .flat_map(|s| s.eval(env).and_then(|a| a.as_number()))
                    .sum();
                Some(Value::Number(res))
            }
            Sexp::List(sexps)
                if sexps
                    .first()
                    .and_then(|s| s.as_atom())
                    .and_then(|a| a.as_atom_str())
                    == Some("lambda") =>
            {
                let names: Option<Vec<String>> = sexps[1].as_list().and_then(|inner_sexps| {
                    inner_sexps
                        .into_iter()
                        .map(|s| {
                            s.as_atom()
                                .and_then(|a| a.as_atom_str().map(|s| s.to_owned()))
                        })
                        .collect()
                });
                let body = sexps[2].clone();
                Some(Value::Lambda(names?, body))
            }
            Sexp::List(sexps) => {
                let head = sexps[0].eval(env)?;
                match head {
                    Value::Number(_) => None,
                    Value::Lambda(names, body) => {
                        let values: Option<Vec<_>> =
                            sexps[1..].iter().map(|sexp| sexp.eval(env)).collect();
                        let values = values?;
                        let mut env_inner = env.clone();
                        names.iter().zip(values).for_each(|(name, value)| {
                            env_inner.insert(name.to_owned(), value);
                        });
                        body.eval(&env_inner)
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
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
}

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(isize),
    Lambda(Vec<String>, Sexp),
}

impl Value {
    fn as_number(&self) -> Option<isize> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_simple_add() {
        let input = "(add 1 2 3 4 5 6)";
        let result = sexp(input).unwrap().1.eval(&Default::default());
        assert_eq!(result, Some(Value::Number(21)));
    }
    #[test]
    fn test_complex_add() {
        let input = "(add (add 4 5 6) 1 2 3)";
        let result = sexp(input).unwrap().1.eval(&Default::default());
        assert_eq!(result, Some(Value::Number(21)));
    }
    #[test]
    fn test_env() {
        let input = "(add (add a a a) 1 2 3)";
        let result = sexp(input)
            .unwrap()
            .1
            .eval(&HashMap::from([("a".to_owned(), Value::Number(1))]));
        assert_eq!(result, Some(Value::Number(9)));
    }

    #[test]
    fn test_lambda() {
        let input = "((lambda (a) (add a a)) 7)";
        let result = sexp(input).unwrap().1.eval(&Default::default());
        assert_eq!(result, Some(Value::Number(14)));
    }
}

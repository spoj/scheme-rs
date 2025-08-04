use crate::{lisp::Value, parse::sexp};
pub mod lisp;
pub mod parse;

#[cfg(test)]
mod tests;

pub fn run_lisp(program: &str) -> Option<Value> {
    let (_, sexp) = sexp(program).ok()?;
    let mut env = Default::default();
    sexp.eval(&mut env)
}

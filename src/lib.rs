use crate::{lisp::Value, parse::program};
pub mod lisp;
pub mod parse;

#[cfg(test)]
mod tests;

pub fn run_lisp(repr: &str) -> Option<Value> {
    let env = Default::default();
    let (_, sexps) = program(repr).ok()?;
    let x: Vec<_> = sexps.into_iter().map(|s| s.eval(&env)).collect();
    x.last().cloned().flatten()
}

use std::collections::HashMap;
pub mod parse;

#[cfg(test)]
mod tests;

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Sexp {
    Atom(Atom),
    List(Vec<Sexp>),
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
    List(Vec<Value>),
    Lambda(Vec<String>, HashMap<String, Value>, Sexp),
}

impl Value {
    fn as_number(&self) -> Option<isize> {
        match self {
            Value::Number(n) => Some(*n),
            _ => None,
        }
    }
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

            // built-in `add`
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

            // built-in `list`
            Sexp::List(sexps)
                if sexps
                    .first()
                    .and_then(|s| s.as_atom())
                    .and_then(|a| a.as_atom_str())
                    == Some("list") =>
            {
                sexps[1..]
                    .iter()
                    .map(|x| x.eval(env))
                    .collect::<Option<Vec<Value>>>()
                    .map(Value::List)
            }
            
            // built-in `cond`
            Sexp::List(sexps)
                if sexps
                    .first()
                    .and_then(|s| s.as_atom())
                    .and_then(|a| a.as_atom_str())
                    == Some("cond") =>
            {
                let mut out = Some(Value::Number(0));
                for pair in &sexps[1..] {
                    if let Some(pair) = pair.as_list()
                        && pair.len() == 2
                        && pair[0].eval(env).is_some_and(|v| v != Value::Number(0))
                    {
                        out = pair[1].eval(env);
                        break;
                    }
                }
                out
            }

            // built-in `equal`
            Sexp::List(sexps)
                if sexps
                    .first()
                    .and_then(|s| s.as_atom())
                    .and_then(|a| a.as_atom_str())
                    == Some("equal") =>
            {
                let x: Option<Vec<isize>> = sexps[1..]
                    .iter()
                    .map(|s| s.eval(env).and_then(|v| v.as_number()))
                    .collect();
                if all_same(x?) {
                    Some(Value::Number(1))
                } else {
                    Some(Value::Number(0))
                }
            }

            // built-in `lambda`
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
                let captures = env.clone();
                Some(Value::Lambda(names?, captures, body))
            }

            // call by value for (f a b c). f has to be a lambda value.
            Sexp::List(sexps) => {
                let head = sexps[0].eval(env)?;
                if let Value::Lambda(names, captures, body) = head {
                    let values: Option<Vec<_>> =
                        sexps[1..].iter().map(|sexp| sexp.eval(env)).collect();
                    let values = values?;
                    let mut context_env = env.clone();

                    context_env.extend(captures);
                    context_env.extend(names.iter().cloned().zip(values));
                    body.eval(&context_env)
                } else {
                    None
                }
            }
        }
    }
}

fn all_same<I, J>(input: I) -> bool
where
    I: IntoIterator<Item = J>,
    J: PartialEq,
{
    let mut iter = input.into_iter();
    match iter.next() {
        Some(item) => iter.all(|next| next == item),
        None => true,
    }
}

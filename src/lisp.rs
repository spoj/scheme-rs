use std::collections::HashMap;

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

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(isize),
    List(Vec<Value>),
    Lambda(Vec<String>, HashMap<String, Value>, Sexp),
}

impl Sexp {
    fn as_list(&self) -> Option<Vec<Sexp>> {
        match self {
            Sexp::Atom(_) => None,
            Sexp::List(v) => Some(v.clone()),
        }
    }
}

impl Sexp {
    pub fn name_of_car(&self) -> Option<&str> {
        if let Sexp::List(sexps) = self
            && let Some(Sexp::Atom(Atom::Symbol(s))) = sexps.first()
        {
            return Some(&s[..]);
        }
        None
    }

    fn built_in_add(cdr: &[Sexp], env: &mut HashMap<String, Value>) -> Option<Value> {
        let res: isize = cdr
            .iter()
            .flat_map(|s| {
                if let Some(Value::Number(n)) = s.eval(env) {
                    Some(n)
                } else {
                    None
                }
            })
            .sum();
        Some(Value::Number(res))
    }
    fn build_in_car(cdr: &[Sexp], env: &mut HashMap<String, Value>) -> Option<Value> {
        if let Some(Value::List(v)) = cdr.first().and_then(|s| s.eval(env))
            && let Some(car) = v.first().cloned()
        {
            return Some(car);
        }
        None
    }
    fn build_in_empty(cdr: &[Sexp], env: &mut HashMap<String, Value>) -> Option<Value> {
        if let Some(Value::List(v)) = cdr.first().and_then(|s| s.eval(env))
            && v.is_empty()
        {
            return Some(Value::Number(1));
        }
        Some(Value::Number(0))
    }
    fn build_in_cdr(cdr: &[Sexp], env: &mut HashMap<String, Value>) -> Option<Value> {
        if let Some(Value::List(v)) = cdr.first().and_then(|s| s.eval(env)) {
            return Some(Value::List(v[1..].to_owned()));
        }
        None
    }
    fn built_in_list(cdr: &[Sexp], env: &mut HashMap<String, Value>) -> Option<Value> {
        cdr.iter()
            .map(|x| x.eval(env))
            .collect::<Option<Vec<Value>>>()
            .map(Value::List)
    }
    fn built_in_cond(cdr: &[Sexp], env: &mut HashMap<String, Value>) -> Option<Value> {
        let mut out = Some(Value::Number(0));
        for pair in cdr {
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
    fn built_in_equal(cdr: &[Sexp], env: &mut HashMap<String, Value>) -> Option<Value> {
        let x: Option<Vec<Value>> = cdr.iter().map(|s| s.eval(env)).collect();
        if all_same(x?) {
            Some(Value::Number(1))
        } else {
            Some(Value::Number(0))
        }
    }

    fn built_in_lambda(cdr: &[Sexp], env: &HashMap<String, Value>) -> Option<Value> {
        if cdr.len() != 2 {
            return None;
        }
        let names: Option<Vec<String>> = cdr[0].as_list().and_then(|inner_sexps| {
            inner_sexps
                .into_iter()
                .map(|s| {
                    if let Sexp::Atom(Atom::Symbol(s)) = s {
                        Some(s.to_owned())
                    } else {
                        None
                    }
                })
                .collect()
        });
        let body = cdr[1].clone();
        let captures = env.clone();
        Some(Value::Lambda(names?, captures, body))
    }

    pub fn eval(&self, env: &mut HashMap<String, Value>) -> Option<Value> {
        match self {
            Sexp::Atom(Atom::Number(n)) => Some(Value::Number(*n)),
            Sexp::Atom(Atom::Symbol(n)) => env.get(n).cloned(),
            Sexp::List(sexps) if sexps.is_empty() => None,

            Sexp::List(sexps) => match self.name_of_car() {
                // built-in functions and forms
                Some("add") => Self::built_in_add(&sexps[1..], env),
                Some("list") => Self::built_in_list(&sexps[1..], env),
                Some("car") => Self::build_in_car(&sexps[1..], env),
                Some("cdr") => Self::build_in_cdr(&sexps[1..], env),
                Some("empty") => Self::build_in_empty(&sexps[1..], env),
                Some("cond") => Self::built_in_cond(&sexps[1..], env),
                Some("equal") => Self::built_in_equal(&sexps[1..], env),
                Some("lambda") => Self::built_in_lambda(&sexps[1..], env),
                // call by value otherwise
                _ => {
                    let head = sexps[0].eval(env)?;
                    if let Value::Lambda(names, captures, body) = head {
                        let values: Option<Vec<_>> =
                            sexps[1..].iter().map(|sexp| sexp.eval(env)).collect();
                        let values = values?;
                        let mut context_env = env.clone();

                        context_env.extend(captures);
                        context_env.extend(names.iter().cloned().zip(values));
                        body.eval(&mut context_env)
                    } else {
                        None
                    }
                }
            },
        }
    }
}

pub fn all_same<I, J>(input: I) -> bool
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

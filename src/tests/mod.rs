use crate::all_same;
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
#[test]
fn test_lambda2() {
    let input = "((lambda (f x) (f (f x))) (lambda (a) (add a a)) 3)";
    let result = sexp(input).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(12)));
}
#[test]
fn test_eq1() {
    let input = "(equal)";
    let result = sexp(input).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(1)));
}
#[test]
fn test_eq2() {
    let input = "(equal 2)";
    let result = sexp(input).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(1)));
}
#[test]
fn test_eq3() {
    let input = "(equal 2 3)";
    let result = sexp(input).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(0)));
}
#[test]
fn test_eq4() {
    let input = "(equal 2 2)";
    let result = sexp(input).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(1)));
}
#[test]
fn test_eq5() {
    let input = "(equal 2 2 2 2)";
    let result = sexp(input).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(1)));
}
#[test]
fn test_eq6() {
    let input = "(equal 2 2 2 2 1)";
    let result = sexp(input).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(0)));
}
#[test]
fn test_all_same() {
    let input: Vec<usize> = vec![];
    assert!(all_same(input));
    let input: Vec<usize> = vec![1];
    assert!(all_same(input));
    let input: Vec<usize> = vec![1, 1];
    assert!(all_same(input));
    let input: Vec<usize> = vec![1, 2];
    assert!(!all_same(input));
    let input: Vec<usize> = vec![1, 1, 2];
    assert!(!all_same(input));
}

use super::*;
use crate::{all_same, parse::sexp};
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
fn test_all_same1() {
    let input: Vec<usize> = vec![];
    assert!(all_same(input));
}

#[test]
fn test_all_same2() {
    let input: Vec<usize> = vec![1];
    assert!(all_same(input));
}

#[test]
fn test_all_same3() {
    let input: Vec<usize> = vec![1, 1];
    assert!(all_same(input));
}

#[test]
fn test_all_same4() {
    let input: Vec<usize> = vec![1, 2];
    assert!(!all_same(input));
}

#[test]
fn test_all_same5() {
    let input: Vec<usize> = vec![1, 1, 2];
    assert!(!all_same(input));
}

#[test]
fn test_cond1() {
    let input = "(cond (0 1) (2 3))";
    let result = sexp(input).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(3)));
}
#[test]
fn test_cond2() {
    let input = "(cond (1 1) (2 3))";
    let result = sexp(input).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(1)));
}
#[test]
fn test_cond3() {
    let input = "(cond (0 1) (0 3))";
    let result = sexp(input).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(0)));
}
#[test]
fn test_cond4() {
    let input = "(cond ((equal 1 2) 1) ((equal 4 4) 3))";
    let result = sexp(input).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(3)));
}

// (x x) becomes
// (lambda (v) ((x x) v))

#[test]
fn test_recur_parse1() {
    let ycomb = "(lambda(f)((lambda (x) (f (lambda (v) ((x x) v)))) (lambda (x) (f (lambda (v) ((x x) v))))))";
    let result = sexp(ycomb).unwrap().1.eval(&Default::default());
    assert!(result.is_some());
}

#[test]
fn test_recur_parse2() {
    let fib = "(lambda (f) (lambda (n) (cond ((equal n 0) (1)) ((equal n 1) (1)) (1 (add (f (add n -1)) (f (add n -2)))))))";
    let result = sexp(fib).unwrap().1.eval(&Default::default());
    assert!(result.is_some());
}

#[test]
fn test_recur_fib() {
    let ycomb = "(lambda(f)((lambda (x) (f (lambda (v) ((x x) v)))) (lambda (x) (f (lambda (v) ((x x) v))))))";
    let fib = "(lambda (f) (lambda (n) (cond ((equal n 0) 1) ((equal n 1) 1) (1 (add (f (add n -1)) (f (add n -2)))))))";
    // let fib = "(lambda (f) (lambda (n) (cond ((equal n 0) 1) ((equal n 1) 1) (1 (f 0)))))";

    let program = format!("(({} {}) {})", ycomb, fib, 0);
    let result = sexp(&program).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(1)));

    let program = format!("(({} {}) {})", ycomb, fib, 1);
    let result = sexp(&program).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(1)));

    let program = format!("(({} {}) {})", ycomb, fib, 2);
    let result = sexp(&program).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(2)));

    let program = format!("(({} {}) {})", ycomb, fib, 3);
    let result = sexp(&program).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(3)));

    let program = format!("(({} {}) {})", ycomb, fib, 4);
    let result = sexp(&program).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(5)));

    let program = format!("(({} {}) {})", ycomb, fib, 5);
    let result = sexp(&program).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(8)));

    let program = format!("(({} {}) {})", ycomb, fib, 6);
    let result = sexp(&program).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(13)));
}

#[test]
fn test_list1() {
    let program = "(list 1 2 (add 3 4))";
    let result = sexp(program).unwrap().1.eval(&Default::default());
    assert_eq!(
        result,
        Some(Value::List(vec![
            Value::Number(1),
            Value::Number(2),
            Value::Number(7)
        ]))
    );
}

#[test]
fn test_list2() {
    let program = "(car (list 1 2 (add 3 4)))";
    let result = sexp(program).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(1)));
}

#[test]
fn test_list3() {
    let program = "(cdr (list 1 2 (add 3 4)))";
    let result = sexp(program).unwrap().1.eval(&Default::default());
    assert_eq!(
        result,
        Some(Value::List(vec![Value::Number(2), Value::Number(7)]))
    );
}

#[test]
fn test_list4() {
    let program = "(empty (cdr (list 1 2 (add 3 4))))";
    let result = sexp(program).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(0)));
}

#[test]
fn test_list5() {
    let program = "(empty (cdr (cdr (list 1 2 (add 3 4)))))";
    let result = sexp(program).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(0)));
}

#[test]
fn test_list6() {
    let program = "(empty (cdr (cdr (cdr (list 1 2 (add 3 4))))))";
    let result = sexp(program).unwrap().1.eval(&Default::default());
    assert_eq!(result, Some(Value::Number(1)));
}


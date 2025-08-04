use crate::lisp::*;
use crate::parse::*;
use crate::run_lisp;
use std::collections::HashMap;
#[test]
fn test_simple_add() {
    assert_eq!(run_lisp("(add 1 2 3 4 5 6)"), Some(Value::Number(21)));
}
#[test]
fn test_complex_add() {
    assert_eq!(run_lisp("(add (add 4 5 6) 1 2 3)"), Some(Value::Number(21)));
}
#[test]
fn test_env() {
    let input = "(add (add a a a) 1 2 3)";
    let result = sexp(input)
        .unwrap()
        .1
        .eval(&mut HashMap::from([("a".to_owned(), Value::Number(1))]));
    assert_eq!(result, Some(Value::Number(9)));
}

#[test]
fn test_lambda() {
    assert_eq!(
        run_lisp("((lambda (a) (add a a)) 7)"),
        Some(Value::Number(14))
    );
}
#[test]
fn test_lambda2() {
    assert_eq!(
        run_lisp("((lambda (f x) (f (f x))) (lambda (a) (add a a)) 3)"),
        Some(Value::Number(12))
    );
}
#[test]
fn test_eq1() {
    assert_eq!(run_lisp("(equal)"), Some(Value::Number(1)));
}
#[test]
fn test_eq2() {
    assert_eq!(run_lisp("(equal 2)"), Some(Value::Number(1)));
}
#[test]
fn test_eq3() {
    assert_eq!(run_lisp("(equal 2 3)"), Some(Value::Number(0)));
}
#[test]
fn test_eq4() {
    assert_eq!(run_lisp("(equal 2 2)"), Some(Value::Number(1)));
}
#[test]
fn test_eq5() {
    assert_eq!(run_lisp("(equal 2 2 2 2)"), Some(Value::Number(1)));
}
#[test]
fn test_eq6() {
    assert_eq!(run_lisp("(equal 2 2 2 2 1)"), Some(Value::Number(0)));
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
    assert_eq!(run_lisp("(cond (0 1) (2 3))"), Some(Value::Number(3)));
}
#[test]
fn test_cond2() {
    assert_eq!(run_lisp("(cond (1 1) (2 3))"), Some(Value::Number(1)));
}
#[test]
fn test_cond3() {
    assert_eq!(run_lisp("(cond (0 1) (0 3))"), Some(Value::Number(0)));
}
#[test]
fn test_cond4() {
    assert_eq!(
        run_lisp("(cond ((equal 1 2) 1) ((equal 4 4) 3))"),
        Some(Value::Number(3))
    );
}

// Z combinator is fixed point combinator for pass by value systems. Same as Y combinator but
// (x x) becomes (lambda (v) ((x x) v))

#[test]
fn test_recur_parse1() {
    let ycomb = "(lambda(f)((lambda (x) (f (lambda (v) ((x x) v)))) (lambda (x) (f (lambda (v) ((x x) v))))))";
    assert!(run_lisp(ycomb).is_some());
}

#[test]
fn test_recur_parse2() {
    let fib = "(lambda (f) (lambda (n) (cond ((equal n 0) (1)) ((equal n 1) (1)) (1 (add (f (add n -1)) (f (add n -2)))))))";
    assert!(run_lisp(fib).is_some());
}

#[test]
fn test_recur_fib() {
    let ycomb = "(lambda(f)((lambda (x) (f (lambda (v) ((x x) v)))) (lambda (x) (f (lambda (v) ((x x) v))))))";
    let fib = "(lambda (f) (lambda (n) (cond ((equal n 0) 1) ((equal n 1) 1) (1 (add (f (add n -1)) (f (add n -2)))))))";
    assert_eq!(
        run_lisp(&format!("(({} {}) {})", ycomb, fib, 0)),
        Some(Value::Number(1))
    );
    assert_eq!(
        run_lisp(&format!("(({} {}) {})", ycomb, fib, 1)),
        Some(Value::Number(1))
    );
    assert_eq!(
        run_lisp(&format!("(({} {}) {})", ycomb, fib, 2)),
        Some(Value::Number(2))
    );
    assert_eq!(
        run_lisp(&format!("(({} {}) {})", ycomb, fib, 3)),
        Some(Value::Number(3))
    );
    assert_eq!(
        run_lisp(&format!("(({} {}) {})", ycomb, fib, 4)),
        Some(Value::Number(5))
    );
    assert_eq!(
        run_lisp(&format!("(({} {}) {})", ycomb, fib, 5)),
        Some(Value::Number(8))
    );
    assert_eq!(
        run_lisp(&format!("(({} {}) {})", ycomb, fib, 6)),
        Some(Value::Number(13))
    );
}

#[test]
fn test_list1() {
    assert_eq!(
        run_lisp("(list 1 2 (add 3 4))"),
        Some(Value::List(vec![
            Value::Number(1),
            Value::Number(2),
            Value::Number(7)
        ]))
    );
}

#[test]
fn test_list2() {
    assert_eq!(
        run_lisp("(car (list 1 2 (add 3 4)))"),
        Some(Value::Number(1))
    );
}

#[test]
fn test_list3() {
    assert_eq!(
        run_lisp("(cdr (list 1 2 (add 3 4)))"),
        Some(Value::List(vec![Value::Number(2), Value::Number(7)]))
    );
}

#[test]
fn test_list4() {
    assert_eq!(
        run_lisp("(empty (cdr (list 1 2 (add 3 4))))"),
        Some(Value::Number(0))
    );
}

#[test]
fn test_list5() {
    assert_eq!(
        run_lisp("(empty (cdr (cdr (list 1 2 (add 3 4)))))"),
        Some(Value::Number(0))
    );
}

#[test]
fn test_list6() {
    assert_eq!(
        run_lisp("(empty (cdr (cdr (cdr (list 1 2 (add 3 4))))))"),
        Some(Value::Number(1))
    );
}

#[test]
fn test_list7() {
    let ycomb = "(lambda(f)((lambda (x)(f(lambda (v) ((x x)v)))) (lambda(x)(f(lambda(v) ((x x) v) ) ) ) )  )";
    let len = "(lambda (f) (lambda (v)  (cond ((empty v) 0) (1 (add 1 (f (cdr v)))))))";
    let data = "(list 1 2 (add 3 4))";

    let program = format!("(({ycomb} {len}) {data})");
    assert_eq!(run_lisp(&program), Some(Value::Number(3)));
}

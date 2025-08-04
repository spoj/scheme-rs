# tiny-lisp-rs

A tiny Lisp interpreter written in Rust.

## Features

*   Basic arithmetic (`add`)
*   List manipulation (`list`, `car`, `cdr`)
*   Conditionals (`cond`)
*   Equality checking (`equal`)
*   First-class functions with lexical scoping (`lambda`)

## Building

To build the project, run:

```bash
cargo build
```

## Usage

The interpreter can be used as a library. You can parse and evaluate Lisp code like this:

```rust
use tiny_lisp_rs::parse::sexp;
use std::collections::HashMap;

let code = "(add 1 2)";
let parsed = sexp(code).unwrap().1;
let result = parsed.eval(&HashMap::new());
println!("{:?}", result);
```

### Examples

Here are some examples of the Lisp dialect supported by this interpreter:

*   `(add 1 2)` => `3`
*   `(list 1 2 3)` => `[1, 2, 3]`
*   `(car (list 1 2 3))` => `1`
*   `(cdr (list 1 2 3))` => `[2, 3]`
*   `(cond ((equal 1 1) 10) ((equal 1 2) 20))` => `10`
*   `((lambda (x) (add x 1)) 5)` => `6`

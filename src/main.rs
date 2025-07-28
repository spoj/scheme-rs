#[allow(unused)]
#[derive(Debug)]
enum Sexp {
    Atom(Atom),
    List(Vec<Sexp>),
}

#[allow(unused)]
#[derive(Debug)]
enum Atom {
    Number(usize),
    Symbol(String),
}

struct Parser<'a> {
    inner: &'a str,
    cursor: usize,
}

impl<'a> Parser<'a> {
    fn new(inner: &'a str) -> Self {
        let cursor = 0;
        Self { inner, cursor }
    }
    fn peek(&mut self) -> Option<&'a str> {
        if self.cursor < self.inner.len() {
            Some(&self.inner[self.cursor..self.cursor + 1])
        } else {
            None
        }
    }
    fn consume_if<P>(&mut self, pred: P) -> Option<&'a str>
    where
        P: Fn(&'a str) -> bool,
    {
        if let Some(x) = self.peek()
            && pred(x)
        {
            self.cursor += 1;
            Some(x)
        } else {
            None
        }
    }
    fn take_while<P>(&mut self, pred: P) -> &'a str
    where
        P: Fn(&'a str) -> bool,
    {
        let start = self.cursor;
        while self.consume_if(&pred).is_some() {}
        let end = self.cursor;
        &self.inner[start..end]
    }
    fn parse_number(&mut self) -> Option<usize> {
        let num = self.take_while(|s| s.chars().next().is_some_and(|c| c.is_ascii_digit()));
        let num = num.parse().ok()?;
        Some(num)
    }
    fn parse_symbol(&mut self) -> Option<String> {
        let name = self.take_while(|s| s.chars().next().is_some_and(|c| c.is_ascii_alphanumeric()));
        (!name.is_empty()).then_some(name.to_string())
    }
    fn consume_spaces(&mut self) {
        while self.consume_if(|s| s == " ").is_some() {}
    }
    fn consume_right_paren(&mut self) -> bool {
        self.consume_if(|s| s == ")").is_some()
    }
    fn parse_list(&mut self) -> Option<Sexp> {
        let mut inner: Vec<Sexp> = vec![];
        match &self.inner[self.cursor..self.cursor + 1] {
            "(" => {
                self.cursor += 1;
                self.consume_spaces();
                while let Some(s) = self.parse_sexp() {
                    inner.push(s);
                    self.consume_spaces();
                }
                self.consume_right_paren();
                Some(Sexp::List(inner))
            }
            _ => None,
        }
    }
    fn parse_atom(&mut self) -> Option<Sexp> {
        None.or_else(|| self.parse_number().map(|n| Sexp::Atom(Atom::Number(n))))
            .or_else(|| self.parse_symbol().map(|n| Sexp::Atom(Atom::Symbol(n))))
    }
    fn parse_sexp(&mut self) -> Option<Sexp> {
        self.parse_atom().or_else(|| self.parse_list())
    }
}

fn main() {
    let input = "(3 (23 34) 5 ()) ";
    let mut parser = Parser::new(input);
    dbg!(parser.parse_sexp());
}

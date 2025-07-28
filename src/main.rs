#[derive(Debug)]
enum Sexp {
    Number(usize),
    List(Vec<Sexp>),
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
    fn parse_sexp(&mut self) -> Option<Sexp> {
        self.parse_number()
            .map(Sexp::Number)
            .or_else(|| self.parse_list())
    }
}

fn main() {
    let input = "(1 2 (1 2 3))";
    let mut parser = Parser::new(input);
    dbg!(parser.parse_sexp());
}

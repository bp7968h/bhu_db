use std::{iter::Peekable, str::Chars};


pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    current: usize,
    line: usize,
    start: usize,
    errors: Vec<String>,
}

impl<'a> Lexer<'a>  {
    pub fn new(source: Peekable<Chars<'a>>) -> Self {
        Lexer {
            source,
            current: 0,
            line: 1,
            start: 0,
            errors: Vec::new(),
        }
    }
}
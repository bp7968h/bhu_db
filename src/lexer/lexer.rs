use std::{iter::Peekable, str::Chars};

use super::{token::Token, token_type::{Literal, TokenType}};

pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,
    current: usize,
    line: usize,
    column: usize,
    errors: Vec<String>,
    tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            source: source.chars().peekable(),
            current: 0,
            line: 1,
            column: 1,
            errors: Vec::new(),
            tokens: Vec::new(),
        }
    }

    pub fn tokens(self) -> Vec<Token> {
        self.tokens
    }

    pub fn tokenize(&mut self) {
        while let Some(character) = self.source.peek() {
            match *character {
                '\n' => {
                    // increament the line and reset the column back to 1
                    // reset only if we have next character otherwise no need to reset
                    self.line += 1;
                    if let Some(_) = self.source.next() {
                        self.column = 1;
                    }
                }
                '(' => {
                    self.add_token_move_ahead(TokenType::LEFTPAREN, "(", None);
                },
                ')' => {
                    self.add_token_move_ahead(TokenType::RIGHTPAREN, ")", None);
                },
                other => {
                    let err_msg = format!("Unexpected character '{}' at line [{}], column [{}]", other, self.line, self.column);
                    self.errors.push(err_msg);
                    self.move_ahead();
                }
            }
        }
    }

    // add token and move forward at the same time
    fn add_token_move_ahead(&mut self, token_type: TokenType, lexeme: &str, literal: Option<Literal>) {
        self.add_token(token_type, lexeme, literal);
        self.move_ahead();
    }

    // just add token
    fn add_token(&mut self, token_type: TokenType, lexeme: &str, literal: Option<Literal>) {
        self.tokens.push(Token::new(
            token_type,
            lexeme.to_string(),
            literal,
            self.line,
            self.column
        ));
    }

    // just move forward as long as we have next character
    fn move_ahead(&mut self) {
        // increament the column cursor by 1 as we go forward
        if let Some(_) = self.source.next() {
            self.column += 1;
        }
    }
}

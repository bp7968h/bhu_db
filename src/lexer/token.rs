use super::token_type::{Literal, TokenType};
use std::fmt;

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: usize,
    column: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
        column: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line,
            column,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let literal_display = match &self.literal {
            Some(lit) => format!("{}", lit),
            None => "None".to_string(),
        };

        write!(
            f,
            "Token {{ 
                type: {:?}, 
                lexeme: '{}', 
                literal: {}, 
                line: {}, 
                column: {} 
            }}",
            self.token_type, self.lexeme, literal_display, self.line, self.column
        )
    }
}

use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    // Keywords
    Create,
    Update,
    Insert,
    Select,
    Delete,
    Database,
    Table,
    Begin,
    Commit,
    Rollback,
    True,
    False,
    NULL,

    //Symbols
    LeftParen,
    RightParen,
    Comma,
    SemiColon,
    Equal,
    Asterisk,
    Plus,          
    Minus,         
    Slash,         
    Percent,       
    LessThan,      
    GreaterThan,   
    LessEqual,   
    GreaterEqual,
    NotEqual,      

    // Logical Operators
    And,
    Or,
    Not,

    // Misc
    Identifier,
    String,
    Number,
    Float,
    EOF,
}

#[derive(Debug)]
pub enum Literal {
    StringLiteral(String),
    NumberLiteral(isize),
    FloatLiteral(f64),
    BooleanLiteral(bool),
    NullLiteral,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::StringLiteral(s) => write!(f, "{}", s),
            Literal::NumberLiteral(n) => write!(f, "{}", n),
            Literal::FloatLiteral(fl) => write!(f, "{}", fl),
            Literal::BooleanLiteral(b) => write!(f, "{}", b),
            Literal::NullLiteral=> write!(f, "Null"),
        }
    }
}

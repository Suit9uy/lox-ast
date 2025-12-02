use crate::token_type::TokenType;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Object {
    Num(u64),
    Str(String),
    Nil,
    True,
    False,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Num(x) => write!(f, "{x}"),
            Object::Str(x) => write!(f, "\"{x}\""),
            Object::Nil => write!(f, "nil"),
            Object::True => write!(f, "True"),
            Object::False => write!(f, "False"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    ttype: TokenType,
    lexeme: String,
    literal: Option<Object>,
    line: usize,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<Object>, line: usize) -> Self {
        Token {
            ttype,
            lexeme,
            literal,
            line,
        }
    }

    pub fn eof(line: usize) -> Token {
        Token {
            ttype: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.ttype,
            self.lexeme,
            if let Some(literal) = &self.literal {
                literal.to_string()
            } else {
                "None".to_string()
            }
        )
    }
}

/*
* passibly:
pub enum Token {
    Literal {lexeme: String, literal: <...>},
    Keyword {lexeme: String, ttype: String}
}
*/

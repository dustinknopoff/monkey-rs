use std::fmt::{Display, Error, Formatter};
use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static!{
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn".into(), TokenType::FUNCTION);
        m.insert("let".into(), TokenType::LET);
        m
    };
}

pub(crate) fn lookup_ident(ident: String) -> TokenType {
    let lookup = KEYWORDS.get(&ident);
    if let Some(tok) = lookup {
        tok.clone()
    } else {
        TokenType::IDENT
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    COMMA,
    SEMICOLON,
    RPAREN,
    LPAREN,
    RBRACE,
    LBRACE,
    FUNCTION,
    LET,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        use TokenType::*;
        match self {
            ILLEGAL => write!(f, "ILLEGAL"),
            EOF => write!(f, "EOF"),
            IDENT => write!(f, "IDENT"),
            INT => write!(f, "INT"),
            ASSIGN => write!(f, "ASSIGN"),
            PLUS => write!(f, "PLUS"),
            COMMA => write!(f, "COMMA"),
            SEMICOLON => write!(f, "SEMICOLON"),
            RPAREN => write!(f, "RPAREN"),
            LPAREN => write!(f, "LPAREN"),
            RBRACE => write!(f, "RBRACE"),
            LBRACE => write!(f, "LBRACE"),
            FUNCTION => write!(f, "FUNCTION"),
            LET => write!(f, "LET"),
        }
    }
}

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) t_type: TokenType,
    pub(crate) literal: String,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            t_type: TokenType::EOF,
            literal: "".to_string(),
        }
    }
}

impl Token {
    pub(crate) fn new<P: Into<String>>(t_type: TokenType, lit: P) -> Self {
        Token {
            t_type,
            literal: lit.into(),
        }
    }
}

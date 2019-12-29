use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

lazy_static! {
    static ref KEYWORDS: HashMap<String, TokenType> = {
        let mut m = HashMap::new();
        m.insert("fn".into(), TokenType::FUNCTION);
        m.insert("let".into(), TokenType::LET);
        m.insert("true".into(), TokenType::TRUE);
        m.insert("false".into(), TokenType::FALSE);
        m.insert("if".into(), TokenType::IF);
        m.insert("else".into(), TokenType::ELSE);
        m.insert("return".into(), TokenType::RETURN);
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
#[allow(dead_code, non_camel_case_types)]
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    SLASH,
    ASTERISK,
    LT,
    GT,
    COMMA,
    SEMICOLON,
    RPAREN,
    LPAREN,
    RBRACE,
    LBRACE,
    FUNCTION,
    LET,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    EQ,
    NOT_EQ,
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
            MINUS => write!(f, "MINUS"),
            BANG => write!(f, "BANG"),
            SLASH => write!(f, "SLASH"),
            ASTERISK => write!(f, "ASTERISK"),
            LT => write!(f, "LT"),
            GT => write!(f, "GT"),
            COMMA => write!(f, "COMMA"),
            SEMICOLON => write!(f, "SEMICOLON"),
            RPAREN => write!(f, "RPAREN"),
            LPAREN => write!(f, "LPAREN"),
            RBRACE => write!(f, "RBRACE"),
            LBRACE => write!(f, "LBRACE"),
            FUNCTION => write!(f, "FUNCTION"),
            LET => write!(f, "LET"),
            IF => write!(f, "IF"),
            FALSE => write!(f, "FALSE"),
            TRUE => write!(f, "TRUE"),
            ELSE => write!(f, "ELSE"),
            RETURN => write!(f, "RETURN"),
            EQ => write!(f, "EQ"),
            NOT_EQ => write!(f, "NOT_EQ"),
        }
    }
}

#[derive(Debug, Clone)]
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

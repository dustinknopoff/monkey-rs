use crate::token::{Token, TokenType};
use crate::ast::Statement::Let;

pub(crate) trait Node {
    fn token_literal(&self) -> String;
}
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Statement {
    Let(LetStatement),
    Ident(Identifier),
    Return(ReturnStatement)
}

trait InternalStatement: Node {
    fn statement_node(&self);
}

pub(crate) trait Expression: Node {
    fn expression_node(&self) -> String;
}

pub(crate) struct Program {
    pub(crate) statements: Vec<Statement>,
}

pub(crate) struct DefaultExpression(String);

impl Node for DefaultExpression {
    fn token_literal(&self) -> String {
        self.0.clone()
    }
}

impl Expression for DefaultExpression {
    fn expression_node(&self) -> String {
        self.0.clone()
    }
}

impl Default for DefaultExpression {
    fn default() -> Self {
        Self(String::new())
    }
}

impl Program {
    pub fn new() -> Self {
        Program {
            statements: Vec::new(),
        }
    }
}

impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(first) = self.statements.first() {
            match first {
                Statement::Let(lt) => lt.token_literal(),
                Statement::Ident(id) => id.token_literal(),
                Statement::Return(ret) => ret.token_literal(),
            };
        };
        String::new()
    }
}

pub(crate) struct LetStatement {
    pub(crate) token: Token, // TokenType::Let
    pub(crate) name: Identifier,
    pub(crate) value: Box<dyn Expression>,
}

impl LetStatement {
    pub fn new(token: Token) -> Self {
        let ident = Identifier {
            token: Token::new(TokenType::EOF, ""),
            value: "".to_string()
        };
        LetStatement {
            token,
            name: ident.clone(),
            value: Box::new(DefaultExpression(String::new()))
        }
    }
}

impl std::fmt::Debug for LetStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LetStatement {{ token: {:?}, name: {:?}, value: {:?} }}",
            self.token,
            self.name,
            self.value.expression_node()
        )
    }
}

impl InternalStatement for LetStatement {
    fn statement_node(&self) {}
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Identifier {
    pub(crate) token: Token, // TokenType::Ident
    pub(crate) value: String,
}

impl InternalStatement for Identifier {
    fn statement_node(&self) {}
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

pub(crate) struct ReturnStatement {
    pub(crate) token: Token, // TokenType::Return
    pub(crate) return_value: Box<dyn Expression>,
}

impl ReturnStatement {
    pub fn new(token: Token) -> Self {
        Self {
            token,
            return_value: Box::new(DefaultExpression(String::new()))
        }
    }
}

impl InternalStatement for ReturnStatement {
    fn statement_node(&self) {}
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }
}

impl std::fmt::Debug for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "LetStatement {{ token: {:?}, return_value: {:?} }}",
            self.token,
            self.return_value.expression_node()
        )
    }
}
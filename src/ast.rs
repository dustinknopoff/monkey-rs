use crate::token::Token;

pub(crate) trait Node {
    fn token_literal(&self) -> String;
}
#[allow(dead_code)]
#[derive(Debug)]
pub(crate) enum Statement {
    Let(LetStatement),
    Ident(Identifier),
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

impl Node for Program {
    fn token_literal(&self) -> String {
        if let Some(first) = self.statements.first() {
            match first {
                Statement::Let(lt) => lt.token_literal(),
                Statement::Ident(id) => id.token_literal(),
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

#[derive(Debug)]
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

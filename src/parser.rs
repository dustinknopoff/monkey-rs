use crate::ast;
use crate::lexer::Lexer;
use crate::token::Token;
use std::fmt::{Error, Formatter};

#[derive(Debug)]
pub(crate) struct ParserError(String);

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "ParserError: {}", self.0)
    }
}

impl std::error::Error for ParserError {}

pub(crate) struct Parser {
    lexer: Lexer,
    cur_token: Token,
    peek_token: Token,
}

impl Parser {
    fn new(lexer: Lexer) -> Self {
        let mut p = Parser {
            lexer,
            cur_token: Default::default(),
            peek_token: Default::default(),
        };
        p.next_token();
        p.next_token();
        p
    }

    fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn parse_program(&mut self) -> Result<ast::Program, Box<dyn std::error::Error>> {
        Err(Box::new(ParserError(String::new())))
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{Node, Statement};
    use crate::lexer::Lexer;
    use crate::parser::Parser;

    struct TestIdent<'a>(&'a str);

    #[test]
    fn test_let_statements() {
        let input = r#"
		let x = 5;
		let y = 10;
		let foobar = 838383;
		"#;
        let mut l = Lexer::new(input.into());
        let mut p = Parser::new(l);

        let program = p.parse_program();
        if program.is_err() {
            panic!("ParseProgram returned Err.")
        }
        let program = program.unwrap();
        if program.statements.len() != 3 {
            panic!(
                "programStatements does not contain 3 statements. got={}",
                program.statements.len()
            )
        }
        let tests = vec![TestIdent("x"), TestIdent("y"), TestIdent("foobar")];
        for (i, tt) in tests.iter().enumerate() {
            let stmt = &program.statements[i];
            if !test_let_statement(&stmt, tt) {
                return;
            }
        }
    }

    fn test_let_statement(actual: &Statement, expected: &TestIdent) -> bool {
        return match actual {
            Statement::Let(stmt) if stmt.token_literal().as_str() != "let" => {
                eprintln!("s.TokenLiteral not 'let'. got={:?}", stmt);
                false
            }
            Statement::Let(stmt) if &stmt.name.value != expected.0 => {
                eprintln!(
                    "letStmt.Name.Value not '{}'. got={}",
                    expected.0, &stmt.name.value
                );
                false
            }
            Statement::Let(stmt) if &stmt.name.token_literal() != expected.0 => {
                eprintln!(
                    "letStmt.Name.TokenLiteral not '{}'. got={}",
                    expected.0,
                    &stmt.name.token_literal()
                );
                false
            }
            Statement::Let(_) => true,
            _ => {
                eprintln!("s not *ast.LetStatement. got={:?}", actual);
                false
            }
        };
    }
}

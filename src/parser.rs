use crate::ast::{self, Program, Statement, LetStatement, Identifier, ReturnStatement};
use crate::lexer::Lexer;
use crate::token::{Token, TokenType};
use std::fmt::{Formatter};

#[derive(Debug, Clone)]
pub(crate) struct ParserError(String);

impl std::fmt::Display for ParserError {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::error::Error> {
		write!(f, "ParserError: {}", self.0)
	}
}

impl std::error::Error for ParserError {}

pub(crate) struct Parser {
	lexer: Lexer,
	errors: Vec<ParserError>,
	cur_token: Token,
	peek_token: Token,
}

impl Parser {
	pub fn new(lexer: Lexer) -> Self {
		let mut p = Parser {
			lexer,
			errors: Vec::new(),
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

	pub fn parse_program(&mut self) -> Result<ast::Program, ParserError> {
		let mut program = Program::new();

		while self.cur_token.t_type != TokenType::EOF {
			let stmt = self.parse_statement();
			if let Ok(state) = stmt {
				program.statements.push(state);
			}
			self.next_token()
		}
		Ok(program)
	}

	fn parse_statement(&mut self) -> Result<ast::Statement, ParserError> {
		use TokenType::*;
		match self.cur_token.t_type {
			LET => self.parse_let_statement(),
			RETURN => self.parse_return_statement(),
			_ => Err(ParserError(String::from("unimplemented!"))),
		}
	}

	fn parse_let_statement(&mut self) -> Result<ast::Statement, ParserError> {
		let mut stmt = LetStatement::new(self.cur_token.clone());
		self.expect_peek(TokenType::IDENT)?;
		stmt.name = Identifier { token: self.cur_token.clone(), value: self.cur_token.literal.clone() };
		self.expect_peek(TokenType::ASSIGN)?;

		// TODO: We're skipping the expressions until we encounter a semicolon
		while !self.cur_token_is(TokenType::SEMICOLON) {
			self.next_token();
		}

		Ok(Statement::Let(stmt))
	}

	fn parse_return_statement(&mut self) -> Result<ast::Statement, ParserError> {
		let stmt = ReturnStatement::new(self.cur_token.clone());

		self.next_token();

		// TODO: We're skipping the expressions until we encounter a semicolon
		while !self.cur_token_is(TokenType::SEMICOLON) {
			self.next_token();
		}

		Ok(Statement::Return(stmt))
	}

	fn cur_token_is(&mut self, token_type: TokenType) -> bool {
		self.cur_token.t_type == token_type
	}

	fn peek_token_is(&mut self, token_type: &TokenType) -> bool {
		&self.peek_token.t_type == token_type
	}

	fn expect_peek(&mut self, token_type: TokenType) -> Result<(), ParserError> {
		if self.peek_token_is(&token_type) {
			self.next_token();
			Ok(())
		} else {
			let err = ParserError(format!("Expected: {}, Got: {}", token_type, self.peek_token.t_type));
			self.errors.push(err.clone());
			Err(err)
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::ast::{Node, Statement};
	use crate::lexer::Lexer;
	use crate::parser::Parser;
	use std::error::Error;

	struct TestIdent<'a>(&'a str);

	#[test]
	fn test_let_statements() {
		let input = r#"
		let x = 5;
		let y = 10;
		let foobar = 838383;
		"#;
		let l = Lexer::new(input.into());
		let mut p = Parser::new(l);

		let program = p.parse_program();
		if program.is_err() {
			panic!("ParseProgram returned Err.")
		}
		let program = program.unwrap();
		check_parser_errors(&p);
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

	#[test]
	fn test_return_statements() {
		let input = r#"
		return 5;
		return 10;
		return 993322;
		"#;

		let l = Lexer::new(input.into());
		let mut p = Parser::new(l);

		let program = p.parse_program();
		if program.is_err() {
			panic!("ParseProgram returned Err.")
		}
		let program = program.unwrap();
		check_parser_errors(&p);
		if program.statements.len() != 3 {
			panic!(
				"programStatements does not contain 3 statements. got={}",
				program.statements.len()
			)
		}

		program.statements.iter().for_each(|stmt| {
			match stmt {
				Statement::Return(ret) if ret.token.literal.as_str() != "return" => eprintln!("token literal not 'return', got {}", ret.token.literal),
				_ => eprintln!("stmt not Return. got={:?}", stmt)
			}
		})
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

	fn check_parser_errors(p: &Parser) {
		let errors = &p.errors;
		if errors.is_empty() {
			return;
		}

		eprintln!("parser has {} errors", errors.len());
		errors.iter().for_each(|msg| {
			eprintln!("{}", msg);
		});
		panic!();
	}
}

use crate::token::{Token, TokenType, lookup_ident};

#[derive(Debug)]
pub(crate) struct Lexer {
	input: String,
	position: usize,
	// current position in input (points to current char)
	read_position: usize,
	// current reading position in input (after current char)
	ch: u8,              // current char under examination
}

impl Lexer {
	pub(crate) fn new(input: String) -> Self {
		let mut l = Lexer {
			input,
			position: 0,
			read_position: 0,
			ch: 0,
		};
		l.read_char();
		l
	}

	pub(crate) fn read_char(&mut self) {
		if self.read_position >= self.input.len() {
			self.ch = 0;
		} else {
			self.ch = self.input.as_bytes()[self.read_position];
		}
		self.position = self.read_position;
		self.read_position += 1;
	}

	pub(crate) fn next_token(&mut self) -> Token {
		let mut tok: Token = Default::default();
		self.skip_whitespace();
		match self.ch {
			b'=' => tok = Token::new(TokenType::ASSIGN, "="),
			b';' => tok = Token::new(TokenType::SEMICOLON, ";"),
			b'(' => tok = Token::new(TokenType::LPAREN, "("),
			b')' => tok = Token::new(TokenType::RPAREN, ")"),
			b'{' => tok = Token::new(TokenType::LBRACE, "{"),
			b'}' => tok = Token::new(TokenType::RBRACE, "}"),
			b',' => tok = Token::new(TokenType::COMMA, ","),
			b'+' => tok = Token::new(TokenType::PLUS, "+"),
			0 => {}
			b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
				tok.literal = self.read_identifier();
				self.position -= 1;
				self.read_position -= 1;
				tok.t_type = lookup_ident(tok.literal.clone());
			}
			b'0'..=b'9' => {
				tok.literal = self.read_number();
				self.position -= 1;
				self.read_position -= 1;
				tok.t_type = TokenType::INT;
			}
			_ => {
				tok = Token::new(TokenType::ILLEGAL, std::str::from_utf8(&[self.ch]).unwrap().to_string());
			}
		};
		self.read_char();
		tok
	}

	fn read_identifier(&mut self) -> String {
		let position = self.position;
		while Lexer::is_letter(self.ch) {
			self.read_char();
		}
		std::str::from_utf8(&self.input.as_bytes()[position..self.position]).unwrap().to_string()
	}

	fn read_number(&mut self) -> String {
		let position = self.position;
		while Lexer::is_digit(self.ch) {
			self.read_char();
		}
		std::str::from_utf8(&self.input.as_bytes()[position..self.position]).unwrap().to_string()
	}

	fn skip_whitespace(&mut self) {
		while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
			self.read_char()
		}
	}

	fn is_letter(ch: u8) -> bool {
		match ch {
			b'a'..=b'z' | b'A'..=b'Z' | b'_' => true,
			_ => false
		}
	}

	fn is_digit(ch: u8) -> bool {
		match ch {
			b'0'..=b'9' => true,
			_ => false,
		}
	}
}

mod tests {
	use super::Lexer;
	use crate::token::TokenType;

	#[derive(Debug)]
	struct ExpectedToken {
		expected_type: TokenType,
		expected_literal: String,
	}

	impl ExpectedToken {
		fn new<P: Into<String>>(expected_type: TokenType, expected_literal: P) -> Self {
			ExpectedToken {
				expected_type,
				expected_literal: expected_literal.into(),
			}
		}
	}

	#[test]
	fn test_next_token() {
		let input = r#"let five = 5;
let ten = 10;

let add = fn(x, y) {
  x + y;
};

let result = add(five, ten);
"#;
		let tests = vec![
			ExpectedToken::new(TokenType::LET, "let"),
			ExpectedToken::new(TokenType::IDENT, "five"),
			ExpectedToken::new(TokenType::ASSIGN, "="),
			ExpectedToken::new(TokenType::INT, "5"),
			ExpectedToken::new(TokenType::SEMICOLON, ";"),
			ExpectedToken::new(TokenType::LET, "let"),
			ExpectedToken::new(TokenType::IDENT, "ten"),
			ExpectedToken::new(TokenType::ASSIGN, "="),
			ExpectedToken::new(TokenType::INT, "10"),
			ExpectedToken::new(TokenType::SEMICOLON, ";"),
			ExpectedToken::new(TokenType::LET, "let"),
			ExpectedToken::new(TokenType::IDENT, "add"),
			ExpectedToken::new(TokenType::ASSIGN, "="),
			ExpectedToken::new(TokenType::FUNCTION, "fn"),
			ExpectedToken::new(TokenType::LPAREN, "("),
			ExpectedToken::new(TokenType::IDENT, "x"),
			ExpectedToken::new(TokenType::COMMA, ","),
			ExpectedToken::new(TokenType::IDENT, "y"),
			ExpectedToken::new(TokenType::RPAREN, ")"),
			ExpectedToken::new(TokenType::LBRACE, "{"),
			ExpectedToken::new(TokenType::IDENT, "x"),
			ExpectedToken::new(TokenType::PLUS, "+"),
			ExpectedToken::new(TokenType::IDENT, "y"),
			ExpectedToken::new(TokenType::SEMICOLON, ";"),
			ExpectedToken::new(TokenType::RBRACE, "}"),
			ExpectedToken::new(TokenType::SEMICOLON, ";"),
			ExpectedToken::new(TokenType::LET, "let"),
			ExpectedToken::new(TokenType::IDENT, "result"),
			ExpectedToken::new(TokenType::ASSIGN, "="),
			ExpectedToken::new(TokenType::IDENT, "add"),
			ExpectedToken::new(TokenType::LPAREN, "("),
			ExpectedToken::new(TokenType::IDENT, "five"),
			ExpectedToken::new(TokenType::COMMA, ","),
			ExpectedToken::new(TokenType::IDENT, "ten"),
			ExpectedToken::new(TokenType::RPAREN, ")"),
			ExpectedToken::new(TokenType::SEMICOLON, ";"),
			ExpectedToken::new(TokenType::EOF, ""),
		];

		let mut l = Lexer::new(input.to_string());

		for (i, tt) in tests.iter().enumerate() {
			let tok = l.next_token();
			assert_eq!(
				tok.t_type, tt.expected_type,
				"tests[{}] - tokentype wrong. expected={}, got {}",
				i, tt.expected_type, tok.t_type,
			);
			assert_eq!(
				tok.literal, tt.expected_literal,
				"tests[{}] - literal wrong. expected={}, got={}",
				i, tt.expected_literal, tok.literal,
			)
		}
	}
}

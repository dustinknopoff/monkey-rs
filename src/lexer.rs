use crate::token::{lookup_ident, Token, TokenType};

#[derive(Debug)]
pub(crate) struct Lexer {
    input: String,
    position: usize,
    // current position in input (points to current char)
    read_position: usize,
    // current reading position in input (after current char)
    ch: u8, // current char under examination
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

    pub(crate) fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }
        self.input.as_bytes()[self.read_position]
    }

    pub(crate) fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();
        tok = match self.ch {
            b'=' if self.peek_char() == b'=' => {
                self.read_char();
                Token::new(TokenType::EQ, "==")
            }
            b'=' => Token::new(TokenType::ASSIGN, "="),
            b';' => Token::new(TokenType::SEMICOLON, ";"),
            b'(' => Token::new(TokenType::LPAREN, "("),
            b')' => Token::new(TokenType::RPAREN, ")"),
            b'{' => Token::new(TokenType::LBRACE, "{"),
            b'}' => Token::new(TokenType::RBRACE, "}"),
            b',' => Token::new(TokenType::COMMA, ","),
            b'+' => Token::new(TokenType::PLUS, "+"),
            b'-' => Token::new(TokenType::MINUS, "-"),
            b'*' => Token::new(TokenType::ASTERISK, "*"),
            b'/' => Token::new(TokenType::SLASH, "/"),
            b'!' if self.peek_char() == b'=' => {
                self.read_char();
                Token::new(TokenType::NOT_EQ, "!=")
            }
            b'!' => Token::new(TokenType::BANG, "!"),
            b'<' => Token::new(TokenType::LT, "<"),
            b'>' => Token::new(TokenType::GT, ">"),
            0 => Default::default(),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let literal = self.read_identifier();
                self.position -= 1;
                self.read_position -= 1;
                Token::new(lookup_ident(literal.clone()), literal)
            }
            b'0'..=b'9' => {
                let int = self.read_number();
                self.position -= 1;
                self.read_position -= 1;
                Token::new(TokenType::INT, int)
            }
            _ => Token::new(
                TokenType::ILLEGAL,
                std::str::from_utf8(&[self.ch]).unwrap().to_string(),
            ),
        };
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char();
        }
        std::str::from_utf8(&self.input.as_bytes()[position..self.position])
            .unwrap()
            .to_string()
    }

    fn read_number(&mut self) -> String {
        let position = self.position;
        while Lexer::is_digit(self.ch) {
            self.read_char();
        }
        std::str::from_utf8(&self.input.as_bytes()[position..self.position])
            .unwrap()
            .to_string()
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char()
        }
    }

    fn is_letter(ch: u8) -> bool {
        match ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => true,
            _ => false,
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
!-/*5;
5 < 10 > 5;
if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
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
            ExpectedToken::new(TokenType::BANG, "!"),
            ExpectedToken::new(TokenType::MINUS, "-"),
            ExpectedToken::new(TokenType::SLASH, "/"),
            ExpectedToken::new(TokenType::ASTERISK, "*"),
            ExpectedToken::new(TokenType::INT, "5"),
            ExpectedToken::new(TokenType::SEMICOLON, ";"),
            ExpectedToken::new(TokenType::INT, "5"),
            ExpectedToken::new(TokenType::LT, "<"),
            ExpectedToken::new(TokenType::INT, "10"),
            ExpectedToken::new(TokenType::GT, ">"),
            ExpectedToken::new(TokenType::INT, "5"),
            ExpectedToken::new(TokenType::SEMICOLON, ";"),
            ExpectedToken::new(TokenType::IF, "if"),
            ExpectedToken::new(TokenType::LPAREN, "("),
            ExpectedToken::new(TokenType::INT, "5"),
            ExpectedToken::new(TokenType::LT, "<"),
            ExpectedToken::new(TokenType::INT, "10"),
            ExpectedToken::new(TokenType::RPAREN, ")"),
            ExpectedToken::new(TokenType::LBRACE, "{"),
            ExpectedToken::new(TokenType::RETURN, "return"),
            ExpectedToken::new(TokenType::TRUE, "true"),
            ExpectedToken::new(TokenType::SEMICOLON, ";"),
            ExpectedToken::new(TokenType::RBRACE, "}"),
            ExpectedToken::new(TokenType::ELSE, "else"),
            ExpectedToken::new(TokenType::LBRACE, "{"),
            ExpectedToken::new(TokenType::RETURN, "return"),
            ExpectedToken::new(TokenType::FALSE, "false"),
            ExpectedToken::new(TokenType::SEMICOLON, ";"),
            ExpectedToken::new(TokenType::RBRACE, "}"),
            ExpectedToken::new(TokenType::INT, "10"),
            ExpectedToken::new(TokenType::EQ, "=="),
            ExpectedToken::new(TokenType::INT, "10"),
            ExpectedToken::new(TokenType::SEMICOLON, ";"),
            ExpectedToken::new(TokenType::INT, "10"),
            ExpectedToken::new(TokenType::NOT_EQ, "!="),
            ExpectedToken::new(TokenType::INT, "9"),
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

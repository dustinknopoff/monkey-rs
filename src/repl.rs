use std::io::{Read, Write};
use scanner_rust::Scanner;
use crate::lexer::Lexer;
use crate::token::TokenType;

static PROMPT: &'static str = ">>";

pub(crate) fn start<I: Read, O: Write>(inpt: I, out: O) {
	let mut scanner = Scanner::scan_stream(inpt);
	loop {
		println!("{}", PROMPT);
		let scanned = scanner.next_line().unwrap();
		if let Some(val) = scanned {
			let mut lexer = Lexer::new(val);
			loop {
				let tok = lexer.next_token();
				if tok.t_type == TokenType::EOF {
					break
				} else {
					println!("{{Type: {}, Literal: {}}}", tok.t_type, tok.literal);
				}
			}
		} else {
			continue
		}
	}
}
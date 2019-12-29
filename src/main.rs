pub(crate) mod ast;
pub(crate) mod lexer;
pub(crate) mod parser;
pub(crate) mod repl;
pub(crate) mod token;
use std::{env, io};
fn main() {
    let user = env::var_os("USER").unwrap().into_string().unwrap();
    println!("Hello {}! This is the Monkey programming language!", user);
    println!("Feel free to type in commands");
    repl::start(io::stdin(), io::stdout());
}

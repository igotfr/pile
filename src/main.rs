mod lexer;
use crate::lexer::*;

mod parser;
use crate::parser::*;

mod interpreter;
use crate::interpreter::*;

fn main() {
    let masterpiece = "1 1\n + print";
    let mut lexer = Lexer::new(masterpiece);
    let mut parser = Parser::new(&mut lexer);
    Interpreter::new(&mut parser).run();
}

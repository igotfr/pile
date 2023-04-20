use crate::lexer::*;

#[derive(Debug)]
pub enum OpKind {
    PushInt(i32),
    PushFloat(f32),
    Plus,
    Minus,
    Mul,
    Div,
    Print,
}

#[derive(Debug)]
pub enum Node {
    Op(OpKind)
}

#[derive(Debug)]
pub struct Parser<'a> {
    pub lexer: &'a mut Lexer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Self { lexer }
    }
}

type ParseError = (usize, String);

impl Iterator for Parser<'_> {
    type Item = Result<Node, ParseError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(lexem) = self.lexer.next() {
            let text = &self.lexer.input[lexem.1..lexem.2];
            return Some(match (lexem.0, text) {
                (TokenKind::LiteralInt, _) => Ok(Node::Op(OpKind::PushInt(text.parse().unwrap()))),
                (TokenKind::LiteralFloat, _) => {
                    Ok(Node::Op(OpKind::PushFloat(text.parse().unwrap())))
                }
                (_, "+") => Ok(Node::Op(OpKind::Plus)),
                (_, "-") => Ok(Node::Op(OpKind::Minus)),
                (_, "*") => Ok(Node::Op(OpKind::Mul)),
                (_, "/") => Ok(Node::Op(OpKind::Div)),
                (_, "print") => Ok(Node::Op(OpKind::Print)),
                (_, &_) => {
                    Err((lexem.3, format!("unknown lexem `{}`", text)))
                }
            });
        }
        None
    }
}

use std::iter::Iterator;

#[derive(Debug)]
pub enum TokenKind {
    LiteralInt,
    LiteralFloat,
    Word,
}

pub fn slice_to_tokenkind(slice: &str) -> TokenKind {
    if slice.parse::<i32>().is_ok() {
        return TokenKind::LiteralInt;
    } else if slice.parse::<f32>().is_ok() {
        return TokenKind::LiteralFloat;
    } else {
        return TokenKind::Word;
    }
}

#[derive(Debug)]
pub struct Lexem(pub TokenKind, pub usize, pub usize, pub usize);

#[derive(Debug)]
pub struct Lexer<'a> {
    pub input: &'a str,
    index: usize,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Self { input, index: 0, line: 0 }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Lexem;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.input.len() {
            return None;
        }
        if let Some(possible_newline) = self.input.chars().nth(self.index) {
            if possible_newline == '\n' {
                self.line += 1;
            } 
        }

        let start = self.index;
        while let Some(ch) = self.input.chars().nth(self.index) {
            if ch.is_whitespace() {
                break;
            }
            self.index += 1;
        }
        let end = self.index;
        self.index += 1;

        return Some(Lexem(
            slice_to_tokenkind(&self.input[start..end]),
            start,
            end,
            self.line,
        ));
    }
}

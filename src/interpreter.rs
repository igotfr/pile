use crate::parser::*;

pub enum Value {
    Float(f32),
    Int(i32),
}

pub struct Interpreter<'a> {
    parser: &'a mut Parser<'a>,
    stack: Vec<Value>,
}

impl<'a> Interpreter<'a> {
    pub fn new(parser: &'a mut Parser<'a>) -> Interpreter<'a> {
        Self {
            parser,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) {
        use Node::*;
        use Value::*;
        use OpKind::*;
        for res in &mut self.parser {
            if let Ok(node) = res {
                match node {
                    Op(PushInt(n)) => self.stack.push(Int(n)),
                    Op(PushFloat(n)) => self.stack.push(Float(n)),
                    Op(Plus)  => println!("Plus"),
                    Op(Minus) => println!("Minus"),
                    Op(Mul)   => println!("Mul"),
                    Op(Div)   => println!("Div"),
                    Op(Print) => println!("Print"),
                }
            } else if let Err((ln, msg)) = res {
                println!("pile: parse error at line {}: {}", ln, msg);
            }
        }
    }
}




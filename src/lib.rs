pub mod parser;
pub mod vm;

// use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
pub struct Insn {
    pub code: Code,
    pub opts: Vec<OptionValue>,
}

impl Insn {
    pub fn new(code: Code, opts: Vec<OptionValue>) -> Self {
        Self { code, opts }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code {
    Nop,
    Push,
    Pop,
    Dup,
    Add,
    Sub,
    Mul,
    Div,
    Not,
    Smaller,
    Bigger,
    Goto,
    If,
    Error,
}

impl Code {
    pub fn new(code: &str) -> Self {
        match code {
            "nop" => Code::Nop,
            "push" => Code::Push,
            "pop" => Code::Pop,
            "dup" => Code::Dup,
            "add" => Code::Add,
            "sub" => Code::Sub,
            "mul" => Code::Mul,
            "div" => Code::Div,
            "not" => Code::Not,
            "smaller" => Code::Smaller,
            "bigger" => Code::Bigger,
            "goto" => Code::Goto,
            "if" => Code::If,
            _ => Code::Error,
        }
    }
}

#[derive(Debug, Clone)]
pub enum OptionValue {
    Integer(i32),
    Goto(i32), // pos
}

#[derive(Debug, Clone)]
pub struct Label {
    pub name: String,
    pub pos: i32,
    pub id: u32,
}

impl Label {
    pub fn new(name: String, id: u32) -> Self {
        let pos = -1;
        Self { name, pos, id }
    }
}

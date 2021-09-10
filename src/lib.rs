pub mod parser;
pub mod vm;

#[derive(Debug, Clone)]
pub struct Insn {
    pub code: Code,
    pub opts: Vec<InsnOption>,
}

impl Insn {
    pub fn new(code: Code, opts: Vec<InsnOption>) -> Self {
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

#[derive(Debug, Clone)]
pub struct InsnOption(OptionValue);

#[derive(Debug, Clone)]
pub enum OptionValue {
    Integer(i32),
    Label(Label),
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

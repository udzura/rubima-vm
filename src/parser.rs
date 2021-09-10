use crate::*;

use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    Insn(Insn),
    Label(Label),
}

macro_rules! match_re {
    ($s:expr, $t:expr) => {
        Regex::new($s).unwrap().find($t)
    };
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub insn: Vec<Insn>,
    pub labels: HashMap<String, Label>,
    pub current_label_id: u32,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            insn: vec![],
            labels: HashMap::new(),
            current_label_id: 0,
        }
    }

    fn get_label_id(&mut self) -> u32 {
        self.current_label_id += 1;
        self.current_label_id
    }

    pub fn parse(prog: &str) -> Result<(), ParseError> {
        let pc = 0u32;
        for line in prog.lines().into_iter() {
            dbg!(line);
            let tokens: Vec<Token> = vec![];
            let line = line.trim_start().trim_end();

            if let Some(matched) = match_re!("", line) {
                dbg!(matched);
            }
        }

        Ok(())
    }
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    line: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: line={:?}", self.line)
    }
}

impl Error for ParseError {}

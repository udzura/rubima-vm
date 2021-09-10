use crate::*;

use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    Insn(Insn),
    Label(Label),
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub labels: HashMap<String, Label>,
    pub current_pabel_id: u32,
}

impl Parser {}

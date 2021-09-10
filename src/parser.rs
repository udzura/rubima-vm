use crate::*;

use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone)]
pub enum Token {
    Cd(Code),
    Int(i32),
    Lbl(Label),
    Marker(Label),
}

macro_rules! re {
    ($s:expr) => {
        Regex::new($s).unwrap()
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

    pub fn parse(&mut self, prog: &str) -> Result<(), ParseError> {
        let mut pc = 0i32;
        for line in prog.lines().into_iter() {
            dbg!(line);
            let mut tokens: Vec<Token> = vec![];
            let mut line = line.trim_start().trim_end();

            let re = re!(r"\A:\w+\z");
            let matched = re.find(line);
            if let Some(m) = matched {
                let name = m.as_str();
                let label = Label::new(name.to_owned(), self.get_label_id());
                self.labels.insert(name.to_owned(), label.clone());
                tokens.push(Token::Marker(label.clone()));
                let len = line.len();
                line = &line[len..len];
            }

            while line.len() > 0 {
                let re = re!(r"\A:[a-z]+");
                let matched = re.find(line);
                if let Some(m) = matched {
                    let name = m.as_str();
                    let label = self
                        .labels
                        .get(&name.to_string())
                        .map(|l| l.clone())
                        .unwrap_or_else(|| {
                            let l = Label::new(name.to_owned(), self.get_label_id());
                            self.labels.insert(name.to_owned(), l.clone());
                            l
                        });
                    tokens.push(Token::Lbl(label.clone()));

                    let end = line.len();
                    line = &line[m.end()..end];
                    continue;
                }

                let re = re!(r"\A\s+");
                let matched = re.find(line);
                if let Some(m) = matched {
                    // ign
                    let end = line.len();
                    line = &line[m.end()..end];
                    continue;
                }

                let re = re!(r"\A#.*");
                let matched = re.find(line);
                if let Some(m) = matched {
                    // ign
                    let end = line.len();
                    line = &line[m.end()..end];
                    continue;
                }

                let re = re!(r"\A[a-z]+");
                let matched = re.find(line);
                if let Some(m) = matched {
                    tokens.push(Token::Cd(Code::new(m.as_str())));

                    let end = line.len();
                    line = &line[m.end()..end];
                    continue;
                }

                let re = re!(r"\A\d+");
                let matched = re.find(line);
                if let Some(m) = matched {
                    tokens.push(Token::Int(m.as_str().parse().unwrap()));

                    let end = line.len();
                    line = &line[m.end()..end];
                    continue;
                }

                return Err(ParseError::new(line));
            }

            if tokens.is_empty() {
                continue;
            }

            match &tokens[0] {
                Token::Marker(label) => {
                    let l = label.clone();
                    let mut val = l.0.borrow_mut();
                    val.pos = pc;
                }
                Token::Cd(code) => {
                    pc += 1;
                    let opts = make_opts(&tokens[1..]);
                    let insn = Insn::new(code.clone(), opts);
                    self.insn.push(insn);
                }
                _ => {
                    panic!("Invalid pattern");
                }
            }
        }

        Ok(())
    }
}

fn make_opts(tokens: &[Token]) -> Vec<OptionValue> {
    let mut ret = vec![];
    for token in tokens.iter() {
        match &token {
            &Token::Int(i) => {
                ret.push(OptionValue::Integer(*i));
            }
            &Token::Lbl(l) => {
                ret.push(OptionValue::Goto(l.clone()));
            }
            _ => {
                panic!("Invalid token: {:?}", token);
            }
        }
    }
    ret
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ParseError {
    line: String,
}

impl ParseError {
    pub fn new(s: &str) -> Self {
        Self {
            line: s.to_string(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ParseError: line={:?}", self.line)
    }
}

impl Error for ParseError {}

#[test]
fn test_parse_sample() {
    let mut parser = Parser::new();
    let code = "
  #
  push 1
:label
  push 1
  add
  dup
  push 1000
  bigger
  if :label
";
    parser.parse(code).unwrap();
    dbg!(&parser.insn);
    assert_eq!(7, parser.insn.len());
}

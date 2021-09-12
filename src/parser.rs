use crate::*;

use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub enum Token<'a> {
    Cd(Code),
    Int(i32),
    Lbl(&'a str),
    Marker(&'a str),
}

macro_rules! re {
    ($s:expr) => {
        Regex::new($s).unwrap()
    };
}

#[derive(Debug, Clone)]
pub struct Parser {
    pub insn: Vec<Insn>,
}

impl Parser {
    pub fn new() -> Self {
        Self { insn: vec![] }
    }

    pub fn parse(&mut self, prog: &str) -> Result<(), ParseError> {
        let mut current_label_id: u32 = 0;
        let mut labels: HashMap<String, Label> = HashMap::default();
        let mut pc = 0i32;
        for line in prog.lines().into_iter() {
            dbg!(line);
            let mut tokens: Vec<Token> = vec![];
            let mut line = line.trim_start().trim_end();

            let re = re!(r"\A:\w+\z");
            let matched = re.find(line);
            if let Some(m) = matched {
                let name = m.as_str();
                current_label_id += 1;
                let label = Label::new(name.to_owned(), current_label_id);
                labels.insert(name.to_owned(), label.clone());
                tokens.push(Token::Marker(name));
                let len = line.len();
                line = &line[len..len];
            }

            while line.len() > 0 {
                let re = re!(r"\A:[a-z]+");
                let matched = re.find(line);
                if let Some(m) = matched {
                    let name = m.as_str();
                    let name = if labels.contains_key(&name.to_string()) {
                        name
                    } else {
                        current_label_id += 1;
                        let l = Label::new(name.to_owned(), current_label_id);
                        labels.insert(name.to_owned(), l.clone());
                        name
                    };
                    tokens.push(Token::Lbl(name));

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

            if let Token::Marker(name) = tokens.get(0).unwrap() {
                let mut label = labels.get_mut(*name).unwrap();
                label.pos = pc;
            }

            if let Token::Cd(code) = tokens.get(0).unwrap() {
                pc += 1;
                let opts = make_opts(&tokens[1..], &labels);
                let insn = Insn::new(code.clone(), opts);
                self.insn.push(insn);
            }
        }

        Ok(())
    }
}

fn make_opts(tokens: &[Token], labels: &HashMap<String, Label>) -> Vec<OptionValue> {
    let mut ret = vec![];
    for token in tokens.iter() {
        match &token {
            &Token::Int(i) => {
                ret.push(OptionValue::Integer(*i));
            }
            &Token::Lbl(name) => {
                let l = labels.get(*name).unwrap();
                ret.push(OptionValue::Goto(l.pos));
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

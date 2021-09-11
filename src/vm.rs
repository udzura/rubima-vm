use crate::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Value {
    Int(i32),
    Bool(bool),
}

impl Value {
    pub fn as_bool(&self) -> bool {
        match self {
            &Self::Bool(b) => b,
            _ => true,
        }
    }

    pub fn try_int(&self) -> Result<i32, RuntimeError> {
        match self {
            &Self::Int(i) => Ok(i),
            _ => Err(RuntimeError::new("Value is not an int")),
        }
    }
}

impl From<OptionValue> for Value {
    fn from(from: OptionValue) -> Self {
        match from {
            OptionValue::Integer(i) => Self::Int(i),
            _ => {
                panic!("Invalid sequence");
            }
        }
    }
}

impl From<i32> for Value {
    fn from(from: i32) -> Self {
        Self::Int(from)
    }
}

impl From<bool> for Value {
    fn from(from: bool) -> Self {
        Self::Bool(from)
    }
}

pub struct RubiMaVm {
    stack: Vec<Value>,
    pc: i32,
}

impl RubiMaVm {
    pub fn new() -> Self {
        Self {
            stack: vec![],
            pc: 0,
        }
    }

    pub fn eval(&mut self, seqence: &[Insn]) -> Result<Option<Value>, RuntimeError> {
        while let Some(insn) = seqence.get(self.pc as usize) {
            self.dispatch(insn)?;
        }
        Ok(self.stack.get(0).map(|v| v.clone()))
    }

    pub fn dispatch(&mut self, insn: &Insn) -> Result<(), RuntimeError> {
        use crate::Code::*;
        dbg!(&insn);
        match insn.code {
            Nop => {}
            Push => {
                self.push(insn.opts[0].clone().into());
            }
            Pop => {
                self.pop()?;
            }
            Dup => {
                let popped = self.pop()?;
                self.push(popped.clone());
                self.push(popped.clone());
            }
            Add => {
                let target = self.pop()?.try_int()? + self.pop()?.try_int()?;
                self.push(target.into());
            }
            Sub => {
                let target = self.pop()?.try_int()? - self.pop()?.try_int()?;
                self.push(target.into());
            }
            Mul => {
                let target = self.pop()?.try_int()? * self.pop()?.try_int()?;
                self.push(target.into());
            }
            Div => {
                let target = self.pop()?.try_int()? / self.pop()?.try_int()?;
                self.push(target.into());
            }
            Not => {
                let target = self.pop()?.as_bool();
                self.push((!target).into());
            }
            Smaller => {
                let target = self.pop()?.try_int()? < self.pop()?.try_int()?;
                self.push(target.into());
            }
            Bigger => {
                let target = self.pop()?.try_int()? > self.pop()?.try_int()?;
                self.push(target.into());
            }
            Goto => match &insn.opts[0] {
                OptionValue::Goto(label) => {
                    let label = label.clone();
                    self.pc = label.0.borrow().pos;
                    return Ok(());
                }
                _ => {
                    return Err(RuntimeError::new("goto should accept label"));
                }
            },
            If => {
                if self.pop()?.as_bool() {
                    match &insn.opts[0] {
                        OptionValue::Goto(label) => {
                            let label = label.clone();
                            self.pc = label.0.borrow().pos;
                            return Ok(());
                        }
                        _ => {
                            return Err(RuntimeError::new("goto should accept label"));
                        }
                    }
                }
            }

            _ => return Err(RuntimeError::new("Error or unknown")),
        }
        self.pc += 1;
        Ok(())
    }

    fn push(&mut self, val: Value) {
        self.stack.push(val);
        dbg!(&self.stack);
    }

    fn pop(&mut self) -> Result<Value, RuntimeError> {
        let v = self
            .stack
            .pop()
            .ok_or_else(|| RuntimeError::new("Empty stack"));
        dbg!(&self.stack);
        v
    }
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct RuntimeError {
    reason: String,
}

impl RuntimeError {
    pub fn new(reason: &str) -> Self {
        Self {
            reason: reason.to_owned(),
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "RuntimeError: {}", self.reason)
    }
}

impl Error for RuntimeError {}

#[test]
fn test_vm_run() {
    let mut parser = crate::parser::Parser::new();
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

    let mut vm = RubiMaVm::new();
    let ret = vm.eval(&parser.insn).unwrap();

    assert_eq!(1000, ret.unwrap().try_int().unwrap());
}

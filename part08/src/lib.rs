use regex::Regex;
use std::fmt;
use std::{collections::HashSet, convert::TryFrom};
#[macro_use]
extern crate lazy_static;

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
  Nop(isize),
  Acc(isize),
  Jmp(isize),
}

impl fmt::Display for Op {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self {
      Op::Nop(v) => write!(f, "Nop {}", v),
      Op::Acc(v) => write!(f, "Acc {}", v),
      Op::Jmp(v) => write!(f, "Jmp {}", v),
    }
  }
}

impl Op {
  pub fn parse(line: &str) -> Option<Op> {
    lazy_static! {
      static ref RE: Regex = Regex::new(r"^([a-z]+) (\+?\-?\d+)$").unwrap();
    }
    let caps = RE.captures(line)?;
    let op = caps.get(1)?.as_str();
    let val = caps.get(2)?.as_str().parse().ok()?;
    match op {
      "acc" => Some(Op::Acc(val)),
      "jmp" => Some(Op::Jmp(val)),
      "nop" => Some(Op::Nop(val)),
      _ => None,
    }
  }
}

#[derive(Debug, PartialEq)]
pub enum ProgramError {
  OutOfBounds,
  InfiniteLoopError(isize),
}

impl fmt::Display for ProgramError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      _ => write!(f, "Infinite loop"),
    }
  }
}

impl std::error::Error for ProgramError {}

pub struct Program {
  ops: Vec<Op>,
  accumulator: isize,
  instruction: usize,
}

impl Program {
  pub fn new(ops: Vec<Op>) -> Program {
    Program {
      ops,
      accumulator: 0,
      instruction: 0,
    }
  }

  pub fn parse(input: &str) -> Option<Program> {
    let res: Result<Vec<_>, _> = input
      .lines()
      .map(|l| Op::parse(l).ok_or("failed to parse"))
      .collect();
    match res {
      Ok(ops) => Some(Program::new(ops)),
      Err(_) => None,
    }
  }

  pub fn munge_and_run(self) -> isize {
    // generate all possible mutations
    for i in 0..self.ops.len() {
      let mut new_ops = self.ops.clone();
      new_ops[i] = match new_ops[i] {
        Op::Acc(x) => Op::Acc(x),
        Op::Nop(x) => Op::Jmp(x),
        Op::Jmp(x) => Op::Nop(x),
      };
      let p = Program {
        ops: new_ops,
        accumulator: self.accumulator,
        instruction: self.instruction,
      };
      let res = p.run();
      if res.is_ok() {
        return res.unwrap();
      }
    }
    panic!("couldn't munge");
  }

  pub fn run(mut self) -> Result<isize, ProgramError> {
    let mut seen = HashSet::new();
    loop {
      if self.instruction == self.ops.len() {
        return Ok(self.accumulator);
      }
      if seen.contains(&self.instruction) {
        return Err(From::from(ProgramError::InfiniteLoopError(
          self.accumulator,
        )));
      }
      let op = self
        .ops
        .get(self.instruction)
        .ok_or(ProgramError::OutOfBounds)?;
      seen.insert(self.instruction);
      match op {
        Op::Acc(val) => {
          self.accumulator += val;
          self.instruction += 1;
        }
        Op::Jmp(val) => {
          self.instruction = usize::try_from((self.instruction as isize) + val)
            .map_err(|_| ProgramError::OutOfBounds)?;
        }
        Op::Nop(_) => self.instruction += 1,
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  static INPUT: &str = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
nop -4
acc +6";

  #[test]
  fn test_parse_op() {
    assert_eq!(Some(Op::Acc(-2)), Op::parse("acc -2"));
    assert_eq!(Some(Op::Nop(0)), Op::parse("nop +0"));
    assert_eq!(Some(Op::Nop(0)), Op::parse("nop -0"));
  }

  #[test]
  fn test_success() {
    let program = Program::parse(INPUT).unwrap();
    let res = program.run();
    assert_eq!(res, Ok(8));
  }

  #[test]
  fn test_infinite_loop() {
    let program = Program::parse(INPUT).unwrap();
    let res = program.run();
    assert_eq!(res, Err(ProgramError::InfiniteLoopError(5)));
  }

  #[test]
  fn test_munge_and_run() {
    let program = Program::parse(INPUT).unwrap();
    assert_eq!(program.munge_and_run(), 8);
  }
}

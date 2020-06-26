#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Range(usize, usize);

impl Range {
  pub fn merge(&self, other: &Range) -> Range {
    use std::cmp::{max, min};
    Range(min(self.0, other.0), max(self.1, other.1))
  }
  pub fn unite(v1: Range, v2: Range) -> Range {
    v1.merge(&v2)
  }
  pub fn dummy() -> Range {
    Range(0, 0)
  }
  pub fn make(start: usize, size: usize) -> Range {
    Range(start, start + size)
  }
  pub fn make_start_end(start: usize, end: usize) -> Range {
    Range(start, end)
  }
  pub fn to_tuple(&self) -> (usize, usize) {
    let Range(start, end) = self;
    if start > end {
      (*end, *start)
    } else {
      (*start, *end)
    }
  }
}

#[derive(Debug, Clone)]
pub enum UntypedASTMain {
  IntConst(i64),
  FloatConst(f64),
  BoolConst(bool),
  IfThenElse(Box<UntypedAST>, Box<UntypedAST>, Box<UntypedAST>),
  Apply((String, Range), Vec<UntypedAST>),
  BinApply((String, Range), Box<UntypedAST>, Box<UntypedAST>),
  ContentOf(String),
}

pub type UntypedAST = (UntypedASTMain, Range);

#[allow(non_snake_case)]
pub fn binary_operator(
  utastL: UntypedAST,
  optok: (String, Range),
  utastR: UntypedAST,
) -> UntypedAST {
  let (_, rngl) = utastL.clone();
  let (op, oprng) = optok;
  let (_, rngr) = utastR.clone();
  let rng = rngl.merge(&rngr);
  (
    UntypedASTMain::BinApply((op, oprng), Box::new(utastL), Box::new(utastR)),
    rng,
  )
}

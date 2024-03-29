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

#[derive(Debug, Clone, PartialEq)]
pub enum UntypedASTMain {
  IntConst(i64),
  FloatConst(f64),
  BoolConst(bool),
  IfThenElse(Box<UntypedAST>, Box<UntypedAST>, Box<UntypedAST>),
  Apply(Box<UntypedAST>, Box<UntypedAST>),
  BinApply((String, Range), Box<UntypedAST>, Box<UntypedAST>),
  ContentOf(Vec<String>, String),
  LetExp(String, Box<UntypedAST>, Box<UntypedAST>),
  LetRecExp(String, String, Box<UntypedAST>, Box<UntypedAST>),
  FunExp(String, Box<UntypedAST>),
  FinishHeaderFile,
}

pub type UntypedAST = (UntypedASTMain, Range);

// `let`や`let mut`で定義された変数情報
#[derive(Debug, Clone)]
pub enum UntypedVarInfo {
  LetVar(String, UntypedAST),
  LetVarMut(String, UntypedAST),
  ReplaceVar(String, UntypedAST),
}

#[derive(Debug, Clone)]
pub struct UntypedFnInfo {
  pub name: String,
  pub args: Vec<String>,
  pub vars: Vec<UntypedVarInfo>,
  pub return_var: UntypedAST,
  pub range: Range,
}

//impl UntypedFnInfo {
//  pub fn to_untyped_fn(&self, env: &[UntypedAST]) -> UntypedFn {
//    let mut lst =
//  }
//}

#[derive(Debug, Clone)]
pub struct UntypedFn {
  pub vas: Vec<UntypedVarInfo>,
  pub return_var: UntypedAST,
}

#[allow(non_snake_case)]
pub fn binary_operator(
  utastL: UntypedAST,
  optok: (String, Range),
  utastR: UntypedAST,
) -> UntypedAST {
  let (_, rngl) = utastL;
  let (op, oprng) = optok;
  let (_, rngr) = utastR;
  let rng = rngl.merge(&rngr);
  (
    UntypedASTMain::BinApply((op, oprng), Box::new(utastL), Box::new(utastR)),
    rng,
  )
}

pub fn make_lambda_list(idlist: Vec<String>, utastlast: UntypedAST, rng: Range) -> UntypedAST {
  let mut utast = utastlast;
  for id in idlist.iter() {
    let new_utast = (UntypedASTMain::FunExp(id.clone(), Box::new(utast)), rng);
    utast = new_utast;
  }
  utast
}

#[derive(Debug, Clone, PartialEq)]
pub enum ASTMain {
  IntConst(i64),
  FloatConst(f64),
  BoolConst(bool),
  IfThenElse(Box<AST>, Box<AST>, Box<AST>),
  Apply(Box<AST>, Box<AST>),
  BinApply((String, Range), Box<AST>, Box<AST>),
  ContentOf(Vec<String>, String),
}

pub type AST = (ASTMain, Range);

pub type TyVar = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum Ty {
  TyInt,
  TyBool,
  TyFloat,
  TyVar(TyVar),
  TyFun(Box<Ty>, Box<Ty>),
  TyList(Box<Ty>),
}

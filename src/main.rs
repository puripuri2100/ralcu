use anyhow::Result;
use clap::{Parser, ValueEnum};
use std::fs::File;
use std::io::prelude::*;

pub mod environment;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod types;

#[derive(Debug, Clone, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  value: String,
  #[clap(short, long)]
  r#type: Type,
}

#[derive(Debug, ValueEnum, Clone)]
enum Type {
  File,
  Text,
}

fn main() -> Result<()> {
  let args = Args::parse();
  let contents = match args.r#type {
    Type::Text => args.value,
    Type::File => {
      let input_file_name = args.value;
      let mut f = File::open(input_file_name)?;
      let mut contents = String::new();
      f.read_to_string(&mut contents)?;
      contents
    }
  };
  a(&contents)?;
  Ok(())
}

#[allow(dead_code)]
fn a(s: &str) -> Result<()> {
  println!(" --- --- ---\n{s}\n --- --- ---");
  let env = environment::empty();
  let tokens = lexer::lex(s)?;
  let ast = parser::parse(tokens)?;
  let (_id, _newenv, v) = eval::eval_decl(env, ast);
  println!("-> {}", eval::string_of_exval(v));
  Ok(())
}

#[allow(dead_code)]
fn b(s: &str) -> Result<()> {
  println!(" --- --- ---\n{s}\n --- --- ---");
  let tokens = lexer::lex(s)?;
  let ast_res = parser::parse(tokens)?;
  println!("{ast_res:?}");
  Ok(())
}

#[cfg(test)]
mod test {
  use crate::environment;
  use crate::eval::{
    self,
    Exval::{self, *},
  };
  use crate::lexer;
  use crate::parser;

  fn test_eval(s: &str) -> Exval {
    let env = environment::empty();
    let tokens = lexer::lex(s).unwrap();
    let ast = parser::parse(tokens).unwrap();
    let (_id, _newenv, v) = eval::eval_decl(env, ast);
    v
  }

  // 四則演算
  #[test]
  fn test1() {
    let s = "(2 * (2 + 3) - 6) / 2";
    let v = IntV(2);
    assert_eq!(v, test_eval(s))
  }

  // 条件分岐
  #[test]
  fn test2() {
    let s = "if true then 5 else 6";
    let v = IntV(5);
    assert_eq!(v, test_eval(s))
  }

  // 比較
  #[test]
  fn test3() {
    let s = "5 < 6";
    let v = BoolV(true);
    assert_eq!(v, test_eval(s))
  }

  // 論理和等
  #[test]
  fn test4() {
    let s = "(not true) || (false && true)";
    let v = BoolV(false);
    assert_eq!(v, test_eval(s))
  }

  // 変数定義
  #[test]
  fn test5() {
    let s = "let x = 5 in let x = x + 1 in x + 1";
    let v = IntV(7);
    assert_eq!(v, test_eval(s))
  }

  // 関数適用
  #[test]
  fn test6() {
    let s = "int ((sin 5.7) +. 6.4)";
    let v = IntV(5);
    assert_eq!(v, test_eval(s))
  }

  // 関数定義
  #[test]
  fn test7() {
    let s = "let f x y = x + y in f 2 3";
    let v = IntV(5);
    assert_eq!(v, test_eval(s))
  }

  // 無名関数定義
  #[test]
  fn test8() {
    let s = "let f = fun x y -> x + y in f 2 3";
    let v = IntV(5);
    assert_eq!(v, test_eval(s))
  }

  // 部分適用
  #[test]
  fn test9() {
    let s = "let f = fun x y -> x + y in let g = f 2 in g 3";
    let v = IntV(5);
    assert_eq!(v, test_eval(s))
  }

  // 2項演算子の定義
  #[test]
  fn test10() {
    let s = "let (^^) v1 v2 = v1 + v2 in 2 ^^ 3";
    let v = IntV(5);
    assert_eq!(v, test_eval(s))
  }
}

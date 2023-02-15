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

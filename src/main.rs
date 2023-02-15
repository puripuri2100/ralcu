use clap::{App, Arg};
use std::fs::File;
use std::io::prelude::*;

pub mod environment;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod types;

fn main() {
  let app = App::new("ralcu").version("0.0.1").arg(
    Arg::with_name("input")
      .help("Specify input file")
      .value_name("FILE")
      .takes_value(true),
  );
  let matches = app.get_matches();
  let input_file_name = matches.value_of("input").unwrap();
  let mut f = File::open(input_file_name).unwrap();
  let mut contents = String::new();
  f.read_to_string(&mut contents).unwrap();
  println!("file name: {input_file_name}");
  a(&contents);
}

#[allow(dead_code)]
fn a(s: &str) {
  println!(" --- --- ---\n{s}\n --- --- ---");
  let env = environment::empty();
  let tokens = lexer::lex(s).unwrap();
  let ast_res = parser::parse(tokens);
  //println!("{:?}", ast_res);
  let ast = ast_res.unwrap();
  let (_id, _newenv, v) = eval::eval_decl(env, ast);
  println!("-> {}", eval::string_of_exval(v));
  //println!("{:?}", newenv);
}

#[allow(dead_code)]
fn b(s: &str) {
  println!(" --- --- ---\n{s}\n --- --- ---");
  let tokens = lexer::lex(s).unwrap();
  let ast_res = parser::parse(tokens);
  println!("{ast_res:?}");
}

use std::fs::File;
use std::io::prelude::*;

pub mod environment;
pub mod eval;
pub mod lexer;
pub mod parser;
pub mod types;

fn main() {
  //let mut input_file_name = "hoge";
  //let mut f = File::open(&mut input_file_name).unwrap();
  //let mut contents = String::new();
  //f.read_to_string(&mut contents).unwrap();
  let s = "1 + 2 * 3";
  a(s);
  let s = "2 * 2 + 3";
  a(s);
  let s = "2 * (2 + 3)";
  a(s);
  let s = "if 2 < 3 then 2 + 3 else 2 * 3";
  a(s);
  let s = "v + (if 2 < 3 then 2 + 3 else 2 * 3)";
  a(s);
  let s = "v + i * x";
  a(s);
  //let s = "a b c d";
  //a(s);
  //  let s = "5 + int(5.6)";
  //  a(s);
  //  let s = "5.5 +. float(5)";
  //  a(s);
  //  let s = "int(float(5)) + 5";
  //  a(s);
  //  let s = "sin(3.14 *. 2.0)";
  //  a(s);
  //  let s = "int_plus(3, 2)";
  //  a(s);
}

#[allow(dead_code)]
fn a(s: &str) {
  let env = environment::extend(
    "i".to_owned(),
    eval::Exval::IntV(1),
    environment::extend(
      "v".to_owned(),
      eval::Exval::IntV(5),
      environment::extend("x".to_owned(), eval::Exval::IntV(10), environment::empty()),
    ),
  );
  let tokens = lexer::lex(s).unwrap();
  let ast = parser::parse(tokens).unwrap();
  let (id, newenv, v) = eval::eval_decl(env, ast);
  println!("val {} =", id);
  println!("{:?}", v);
  println!("{:?}", newenv);
}

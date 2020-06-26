use std::fs::File;
use std::io::prelude::*;

pub mod backend;
pub mod frontend;
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
  let s = "5 + (if 2 < 3 then 2 + 3 else 2 * 3)";
  a(s);
  let s = "5 + int(5.6)";
  a(s);
  let s = "5.5 +. float(5)";
  a(s);
  let s = "int(float(5)) + 5";
  a(s);
  let s = "sin(3.14 *. 2.0)";
  a(s);
  let s = "int_plus(3, 2)";
  a(s);
}

fn a(s: &str) {
  let ast = backend::main(&frontend::get_ast(s));
  println!("{} => {}", s, ast)
}

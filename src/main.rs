//use std::fs::File;
//use std::io::prelude::*;

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
  a("let hoge = 1 in hoge + 3");
  a("let hoge = 1 let fuga = 3 in hoge + fuga");
  a("let hoge = 1 in let fuga = 3 in hoge + fuga");
  let s = "
  let x = 1 in
  let y = x + 2 in
  y * 4
  ";
  a(s);
  let s = "
  let x = 1
  let z = 1
  let y = x + 2 in
  y * 4
  ";
  a(s);
  let s = "
  let x = 1
  let z = 1
  let y = x + 2 in
  let a =
    let a1 = 2 in
    let a2 = 3 in
  a1 + a2
  in
  a
  ";
  a(s);
  let s = "
  let x = 1
  let z = 1
  let y = x + 2 in
  let a =
    let a1 = 2 in
    let a2 = 3 in
    let a3 = 4 in
  a1 + a2
  in
  a + x + y * z
  ";
  a(s);
  let s = "
    let f = fun x y -> x + y in
    let g = f 3 in
    g 5
  ";
  a(s);
  let s = "
  fun x y -> plus x y
  ";
  a(s);
  let s = "
    plus 3 5
  ";
  a(s);
  let s = "
  let f =
    let x = 2 in
    let addx = fun y z -> x + y * z in
    addx
  in
  f 4 3
  ";
  a(s);
  let s = "
  let f =
    let x = 2 in
    fun y z -> x + y + z
  in
  f
  ";
  a(s);
  let s = "
  let f =
    let x = 2 in
    fun y z -> x + y + z
  in
  let g = f 3 in
  g
  ";
  a(s);
  let s = "
  let f =
    let x = 2 in
    fun g y z -> x + g y + z
  in
  let g = fun x -> x * 3 in
  f g 4 5
  ";
  a(s);
  let s = "
  let f =
    let x = 2 in
    fun g y z -> x + g y + z
  in
  let g = fun x -> x * 3 in
  f g
  ";
  a(s);
  //b("e1 e2 + e3");
  //b("e1 + e2 e3");
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
  println!(" --- --- ---\n{}\n --- --- ---", s);
  let env = environment::extend(
    "i".to_owned(),
    eval::Exval::IntV(1),
    environment::extend(
      "v".to_owned(),
      eval::Exval::IntV(5),
      environment::extend(
        "x".to_owned(),
        eval::Exval::IntV(10),
        //eval::primitive_env(environment::empty()),
        environment::empty(),
      ),
    ),
  );
  let tokens = lexer::lex(s).unwrap();
  let ast_res = parser::parse(tokens);
  //println!("{:?}", ast_res);
  let ast = ast_res.unwrap();
  let (id, newenv, v) = eval::eval_decl(env, ast);
  println!("val {} =", id);
  println!("{:?}", v);
  println!("{:?}", newenv);
}

#[allow(dead_code)]
fn b(s: &str) {
  println!(" --- --- ---\n{}\n --- --- ---", s);
  let tokens = lexer::lex(s).unwrap();
  let ast_res = parser::parse(tokens);
  println!("{:?}", ast_res);
}

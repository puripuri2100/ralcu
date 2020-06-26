use super::types;

pub mod lexer;
pub mod parser;

pub fn get_ast(input: &str) -> types::UntypedAST {
  let tokens = lexer::lex(input).unwrap();
  let ast = parser::parse(tokens).unwrap();
  ast
}

#[allow(dead_code)]
fn show_ast(ast: &types::UntypedAST) -> String {
  let (astmain, _) = ast;
  match astmain {
    types::UntypedASTMain::BoolConst(b) => format!(" {:?} ", b),
    types::UntypedASTMain::IntConst(i) => format!(" {:?} ", i),
    types::UntypedASTMain::FloatConst(f) => format!(" {:?} ", f),
    types::UntypedASTMain::IfThenElse(b, t, f) => {
      let b_s = show_ast(b);
      let t_s = show_ast(t);
      let f_s = show_ast(f);
      format!(" if {} then {} else {} ", b_s, t_s, f_s)
    }
    types::UntypedASTMain::Apply(name, arg_lst) => {
      let (name_s, _) = name;
      let mut arg_s = String::new();
      for i in 0..arg_lst.len() {
        let v = &arg_lst[i];
        if i == arg_lst.len() {
          arg_s.push_str(&format!("{}", show_ast(&v)))
        } else {
          arg_s.push_str(&format!("{}, ", show_ast(&v)))
        }
      }
      format!(" {}({}) ", name_s, arg_s)
    }
    types::UntypedASTMain::BinApply(n, l, r) => {
      let (n_s, _) = n;
      let l_s = show_ast(l);
      let r_s = show_ast(r);
      format!(" ({} {} {}) ", n_s, l_s, r_s)
    }
    types::UntypedASTMain::ContentOf(s) => format!(" ({}) ", s),
  }
}

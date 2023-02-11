use super::types;

pub mod lexer;
pub mod parser;

pub fn get_ast(input: &str) -> types::UntypedAST {
  let tokens = lexer::lex(input).unwrap();
  let (_fn_lst, ast) = parser::parse(tokens).unwrap();
  ast
}

#[allow(dead_code)]
fn show_ast(ast: &types::UntypedAST) -> String {
  let (astmain, _) = ast;
  match astmain {
    types::UntypedASTMain::BoolConst(b) => format!(" {b:?} "),
    types::UntypedASTMain::IntConst(i) => format!(" {i:?} "),
    types::UntypedASTMain::FloatConst(f) => format!(" {f:?} "),
    types::UntypedASTMain::IfThenElse(b, t, f) => {
      let b_s = show_ast(b);
      let t_s = show_ast(t);
      let f_s = show_ast(f);
      format!(" if {b_s} then {t_s} else {f_s} ")
    }
    types::UntypedASTMain::App(name_s, args) => {
      let arg_s_v = args.iter().map(show_ast).collect::<Vec<_>>();
      let arg_s = arg_s_v.join(",");
      format!("{name_s} ({arg_s}) ")
    }
    types::UntypedASTMain::BinApply(n, l, r) => {
      let (n_s, _) = n;
      let l_s = show_ast(l);
      let r_s = show_ast(r);
      format!(" ({n_s} {l_s} {r_s}) ")
    }
    types::UntypedASTMain::ContentOf(_, s) => format!(" ({s}) "),
  }
}

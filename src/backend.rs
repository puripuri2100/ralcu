use super::types;

pub mod primitive;

pub fn main(input: &types::UntypedAST) -> String {
  let (get_ast, _) = primitive::apply(input);
  match get_ast {
    types::UntypedASTMain::IntConst(n) => format!("{}", n),
    types::UntypedASTMain::FloatConst(f) => format!("{}", f),
    types::UntypedASTMain::BoolConst(b) => format!("{}", b),
    types::UntypedASTMain::BinApply(name, left, right) => {
      format!("({:?} {:?} {:?})", name, left, right)
    }
    types::UntypedASTMain::App(name, right) => format!("({} {:?})", name, right),
    types::UntypedASTMain::IfThenElse(b, t, f) => format!("if {:?} then {:?} else {:?})", b, t, f),
    types::UntypedASTMain::ContentOf(_, s) => format!("{}", s),
  }
}

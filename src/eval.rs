use super::environment;
use super::types;

#[derive(Clone, PartialEq, Debug, PartialOrd)]
pub enum Exval {
  IntV(i64),
  FloatV(f64),
  BoolV(bool),
}

type Dnval = Exval;

pub fn string_of_exval(exval: Dnval) -> String {
  match exval {
    Exval::IntV(i) => format!("{:?}", i),
    Exval::FloatV(f) => format!("{:?}", f),
    Exval::BoolV(b) => format!("{:?}", b),
  }
}

pub fn apply_prim(op: String, arg1: Dnval, arg2: Dnval) -> Dnval {
  match (op.as_str(), arg1, arg2) {
    ("+", Exval::IntV(i1), Exval::IntV(i2)) => Exval::IntV(i1 + i2),
    ("+.", Exval::FloatV(f1), Exval::FloatV(f2)) => Exval::FloatV(f1 + f2),
    ("*", Exval::IntV(i1), Exval::IntV(i2)) => Exval::IntV(i1 * i2),
    ("*.", Exval::FloatV(f1), Exval::FloatV(f2)) => Exval::FloatV(f1 * f2),
    (">", Exval::IntV(i1), Exval::IntV(i2)) => Exval::BoolV(i1 > i2),
    ("<", Exval::IntV(i1), Exval::IntV(i2)) => Exval::BoolV(i1 < i2),
    (">.", Exval::FloatV(f1), Exval::FloatV(f2)) => Exval::BoolV(f1 > f2),
    ("<.", Exval::FloatV(f1), Exval::FloatV(f2)) => Exval::BoolV(f1 < f2),
    _ => panic!(),
  }
}

pub fn eval_exp(env: environment::Env<Exval>, ast: types::UntypedAST) -> Dnval {
  use types::UntypedASTMain;
  let (astmain, _) = ast;
  match astmain {
    UntypedASTMain::IntConst(i) => Exval::IntV(i),
    UntypedASTMain::FloatConst(f) => Exval::FloatV(f),
    UntypedASTMain::BoolConst(b) => Exval::BoolV(b),
    UntypedASTMain::IfThenElse(exp1, exp2, exp3) => {
      let test = eval_exp(env.clone(), *exp1);
      match test {
        Exval::BoolV(true) => eval_exp(env, *exp2),
        Exval::BoolV(false) => eval_exp(env, *exp3),
        _ => panic!(),
      }
    }
    UntypedASTMain::ContentOf(_, x) => environment::lookup(x, env).unwrap(),
    UntypedASTMain::BinApply(op, exp1, exp2) => {
      let (op_string, _) = op;
      let arg1 = eval_exp(env.clone(), *exp1);
      let arg2 = eval_exp(env.clone(), *exp2);
      apply_prim(op_string, arg1, arg2)
    }
    UntypedASTMain::LetExp(id, exp1, exp2) => {
      let value = eval_exp(env.clone(), *exp1);
      eval_exp(environment::extend(id, value, env), *exp2)
    }
    UntypedASTMain::FinishHeaderFile => panic!(),
    _ => panic!(),
  }
}

pub fn eval_decl(
  env: environment::Env<Exval>,
  ast: types::UntypedAST,
) -> (String, environment::Env<Exval>, Dnval) {
  let v = eval_exp(env.clone(), ast);
  ("-".to_owned(), env, v)
}

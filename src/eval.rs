use super::environment;
use super::types;

#[derive(Clone, PartialEq, Debug)]
pub enum Exval {
  IntV(i64),
  FloatV(f64),
  BoolV(bool),
  ProcV(String, types::UntypedAST, environment::Env<Dnval>),
  PrimitiveV(String, types::UntypedAST, environment::Env<Dnval>),
}

type Dnval = Exval;

fn get_exval_int(input: Exval) -> Option<i64> {
  match input {
    Exval::IntV(i) => Some(i),
    _ => None,
  }
}

fn get_exval_float(input: Exval) -> Option<f64> {
  match input {
    Exval::FloatV(f) => Some(f),
    _ => None,
  }
}

fn get_exval_bool(input: Exval) -> Option<bool> {
  match input {
    Exval::BoolV(b) => Some(b),
    _ => None,
  }
}

fn make_exval_int(i: i64) -> Exval {
  Exval::IntV(i)
}

fn make_exval_float(f: f64) -> Exval {
  Exval::FloatV(f)
}

fn make_exval_bool(b: bool) -> Exval {
  Exval::BoolV(b)
}

pub fn string_of_exval(exval: Dnval) -> String {
  match exval {
    Exval::IntV(i) => format!("{:?}", i),
    Exval::FloatV(f) => format!("{:?}", f),
    Exval::BoolV(b) => format!("{:?}", b),
    _ => format!("{:?}", exval),
  }
}

fn my_plus(env: environment::Env<Dnval>, x_str: String, y_str: String) -> types::UntypedAST {
  let x_eval_opt = environment::lookup(x_str.clone(), env.clone());
  let y_eval_opt = environment::lookup(y_str.clone(), env.clone());
  let mainast = match (x_eval_opt, y_eval_opt) {
    (Some(x_eval), Some(y_eval)) => {
      let x = get_exval_int(x_eval).unwrap();
      let y = get_exval_int(y_eval).unwrap();
      let i = x + y;
      types::UntypedASTMain::IntConst(i)
    }
    _ => types::UntypedASTMain::ContentOf(Vec::new(), "plus".to_owned()),
  };
  (
    types::UntypedASTMain::Apply(
      Box::new((
        types::UntypedASTMain::Apply(
          Box::new((mainast, types::Range::dummy())),
          Box::new((
            types::UntypedASTMain::ContentOf(Vec::new(), x_str),
            types::Range::dummy(),
          )),
        ),
        types::Range::dummy(),
      )),
      Box::new((
        types::UntypedASTMain::ContentOf(Vec::new(), y_str),
        types::Range::dummy(),
      )),
    ),
    types::Range::dummy(),
  )
}

pub fn primitive_env(id: String, env: environment::Env<Dnval>) -> Option<Dnval> {
  let primitive_vec = vec![(
    "plus".to_owned(),
    Exval::ProcV(
      "_x".to_owned(),
      (
        types::UntypedASTMain::FunExp(
          "_y".to_owned(),
          Box::new(my_plus(env.clone(), "_x".to_owned(), "_y".to_owned())),
        ),
        types::Range::dummy(),
      ),
      env.clone(),
    ),
  )];
  let v = primitive_vec
    .iter()
    .find(|(primitive_id, _)| primitive_id == &id);
  match v {
    None => None,
    Some((_, e)) => Some(e.clone()),
  }
}


fn apply_prim(op: String, arg1: Dnval, arg2: Dnval) -> Dnval {
  match (op.as_str(), arg1, arg2) {
    ("+", Exval::IntV(i1), Exval::IntV(i2)) => make_exval_int(i1 + i2),
    ("+.", Exval::FloatV(f1), Exval::FloatV(f2)) => make_exval_float(f1 + f2),
    ("*", Exval::IntV(i1), Exval::IntV(i2)) => make_exval_int(i1 * i2),
    ("*.", Exval::FloatV(f1), Exval::FloatV(f2)) => make_exval_float(f1 * f2),
    (">", Exval::IntV(i1), Exval::IntV(i2)) => make_exval_bool(i1 > i2),
    ("<", Exval::IntV(i1), Exval::IntV(i2)) => make_exval_bool(i1 < i2),
    (">.", Exval::FloatV(f1), Exval::FloatV(f2)) => make_exval_bool(f1 > f2),
    ("<.", Exval::FloatV(f1), Exval::FloatV(f2)) => make_exval_bool(f1 < f2),
    _ => panic!(),
  }
}

//(Apply(
//  (Apply(
//    (ContentOf([], "plus"), Range(5, 9)),
//    (IntConst(3), Range(10, 11)))
//  , Range(5, 11)),
//  (IntConst(5), Range(12, 13)))
//, Range(5, 13)
//)

pub fn eval_exp(env: environment::Env<Dnval>, ast: types::UntypedAST) -> Dnval {
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
    UntypedASTMain::ContentOf(_, x) => match environment::lookup(x.clone(), env.clone()) {
      Some(e) => e,
      None => primitive_env(x.clone(), env.clone()).unwrap(),
    },
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
    UntypedASTMain::FunExp(id, exp) => Exval::ProcV(id, *exp, env),
    UntypedASTMain::Apply(exp1, exp2) => {
      let funval = eval_exp(env.clone(), *exp1);
      let arg = eval_exp(env, *exp2);
      match funval {
        Exval::ProcV(id, body, env2) => {
          let newenv = environment::extend(id, arg, env2);
          //println!("newenv: {:?}", newenv);
          //println!("body: {:?}", body);
          eval_exp(newenv, body)
        }
        _ => funval,
      }
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

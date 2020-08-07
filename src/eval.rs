use super::environment;
use super::types;
use std::cell::RefCell;

#[derive(Clone, PartialEq, Debug)]
pub enum Exval {
  IntV(i64),
  FloatV(f64),
  BoolV(bool),
  ProcV(String, types::UntypedAST, RefCell<environment::Env<Dnval>>),
  PrimitiveV(String, usize, Vec<Dnval>),
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

fn get_primitive_exval(id: String) -> Option<Dnval> {
  let primitive_vec = vec![
    (
      "+".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "+.".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "-".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "-.".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "*".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "*.".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "/".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "/.".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      ">".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      ">.".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "<".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "<.".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "==".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "&&".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "||".to_owned(),
      Exval::PrimitiveV("+".to_owned(), 2, Vec::new()),
    ),
    (
      "sin".to_owned(),
      Exval::PrimitiveV("sin".to_owned(), 1, Vec::new()),
    ),
    (
      "not".to_owned(),
      Exval::PrimitiveV("not".to_owned(), 1, Vec::new()),
    ),
    (
      "int".to_owned(),
      Exval::PrimitiveV("int".to_owned(), 1, Vec::new()),
    ),
    (
      "float".to_owned(),
      Exval::PrimitiveV("float".to_owned(), 1, Vec::new()),
    ),
  ];
  let v = primitive_vec
    .iter()
    .find(|(primitive_id, _)| primitive_id == &id);
  match v {
    None => None,
    Some((_, e)) => Some(e.clone()),
  }
}

fn def_primitive(id: String, argvec: Vec<Dnval>) -> Option<Dnval> {
  match id.as_str() {
    "+" => {
      let x = get_exval_int(argvec[0].clone()).unwrap();
      let y = get_exval_int(argvec[1].clone()).unwrap();
      Some(make_exval_int(x + y))
    }
    "+." => {
      let x = get_exval_float(argvec[0].clone()).unwrap();
      let y = get_exval_float(argvec[1].clone()).unwrap();
      Some(make_exval_float(x + y))
    }
    "-" => {
      let x = get_exval_int(argvec[0].clone()).unwrap();
      let y = get_exval_int(argvec[1].clone()).unwrap();
      Some(make_exval_int(x - y))
    }
    "-." => {
      let x = get_exval_float(argvec[0].clone()).unwrap();
      let y = get_exval_float(argvec[1].clone()).unwrap();
      Some(make_exval_float(x - y))
    }
    "*" => {
      let x = get_exval_int(argvec[0].clone()).unwrap();
      let y = get_exval_int(argvec[1].clone()).unwrap();
      Some(make_exval_int(x * y))
    }
    "*." => {
      let x = get_exval_float(argvec[0].clone()).unwrap();
      let y = get_exval_float(argvec[1].clone()).unwrap();
      Some(make_exval_float(x * y))
    }
    "/" => {
      let x = get_exval_int(argvec[0].clone()).unwrap();
      let y = get_exval_int(argvec[1].clone()).unwrap();
      Some(make_exval_int(x / y))
    }
    "/." => {
      let x = get_exval_float(argvec[0].clone()).unwrap();
      let y = get_exval_float(argvec[1].clone()).unwrap();
      Some(make_exval_float(x / y))
    }
    ">" => {
      let x = get_exval_int(argvec[0].clone()).unwrap();
      let y = get_exval_int(argvec[1].clone()).unwrap();
      Some(make_exval_bool(x > y))
    }
    ">." => {
      let x = get_exval_float(argvec[0].clone()).unwrap();
      let y = get_exval_float(argvec[1].clone()).unwrap();
      Some(make_exval_bool(x > y))
    }
    "<" => {
      let x = get_exval_int(argvec[0].clone()).unwrap();
      let y = get_exval_int(argvec[1].clone()).unwrap();
      Some(make_exval_bool(x < y))
    }
    "<." => {
      let x = get_exval_float(argvec[0].clone()).unwrap();
      let y = get_exval_float(argvec[1].clone()).unwrap();
      Some(make_exval_bool(x < y))
    }
    "==" => {
      let x = get_exval_int(argvec[0].clone()).unwrap();
      let y = get_exval_int(argvec[1].clone()).unwrap();
      Some(make_exval_bool(x == y))
    }
    "&&" => {
      let x = get_exval_bool(argvec[0].clone()).unwrap();
      let y = get_exval_bool(argvec[1].clone()).unwrap();
      Some(make_exval_bool(x && y))
    }
    "||" => {
      let x = get_exval_bool(argvec[0].clone()).unwrap();
      let y = get_exval_bool(argvec[1].clone()).unwrap();
      Some(make_exval_bool(x || y))
    }
    "sin" => {
      let x = get_exval_float(argvec[0].clone()).unwrap();
      Some(make_exval_float(x.sin()))
    }
    "not" => {
      let x = get_exval_bool(argvec[0].clone()).unwrap();
      Some(make_exval_bool(!x))
    }
    "int" => {
      let x = get_exval_float(argvec[0].clone()).unwrap();
      Some(make_exval_int(x as i64))
    }
    "float" => {
      let x = get_exval_int(argvec[0].clone()).unwrap();
      Some(make_exval_float(x as f64))
    }
    _ => None,
  }
}

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
      Some(e) => e,                            //ProcVを返す
      None => get_primitive_exval(x).unwrap(), //PrimitiveVを返す
    },
    UntypedASTMain::BinApply(op, exp1, exp2) => {
      let (op_string, _) = op;
      let arg1 = eval_exp(env.clone(), *exp1);
      let arg2 = eval_exp(env.clone(), *exp2);
      match environment::lookup(op_string.clone(), env.clone()) {
        Some(e) => match e {
          Exval::ProcV(id, body, env2) => {
            let newenv = environment::extend(id, arg1, env2.into_inner());
            match eval_exp(newenv, body) {
              Exval::ProcV(id, body2, env3) => {
                let newenv = environment::extend(id, arg2, env3.into_inner());
                eval_exp(newenv, body2)
              }
              _ => panic!(),
            }
          }
          _ => panic!(),
        },
        None => def_primitive(op_string, vec![arg1, arg2]).unwrap(), //PrimitiveVを返す
      }
    }
    UntypedASTMain::LetExp(id, exp1, exp2) => {
      let value = eval_exp(env.clone(), *exp1);
      eval_exp(environment::extend(id, value, env), *exp2)
    }
    UntypedASTMain::LetRecExp(id, para, exp1, exp2) => {
      let dummyenv = RefCell::new(environment::empty());
      let newenv =
        environment::extend(id.clone(), Exval::ProcV(para, *exp1, dummyenv.clone()), env);
      println!("newenv1: {:?}\n", newenv);
      let _ = dummyenv.replace(newenv.clone());
      println!("newenv2: {:?}\n", newenv);
      eval_exp(newenv, *exp2)
    }
    UntypedASTMain::FunExp(id, exp) => Exval::ProcV(id, *exp, RefCell::new(env)),
    UntypedASTMain::Apply(exp1, exp2) => {
      let funval = eval_exp(env.clone(), *exp1);
      let arg = eval_exp(env, *exp2);
      match funval {
        Exval::ProcV(id, body, env2) => {
          let newenv = environment::extend(id, arg, env2.into_inner());
          println!("newenv: {:?}\n", newenv);
          //println!("body: {:?}", body);
          let newexval = eval_exp(newenv.clone(), body);
          //match newexval {
          //  Exval::ProcV(id, body, env) => eval_exp(
          //    newenv,
          //    (
          //      types::UntypedASTMain::FunExp(id, Box::new(body)),
          //      types::Range::dummy(),
          //    ),
          //  ),
          //  _ => newexval,
          //}
          newexval
        }
        Exval::PrimitiveV(id, len, argvec) => {
          if argvec.len() == len {
            //評価
            def_primitive(id, argvec).unwrap()
          } else {
            // 引数に値を入れる
            let mut argvec = argvec;
            argvec.push(arg);
            let newval = Exval::PrimitiveV(id.clone(), len, argvec.clone());
            if argvec.len() == len {
              //評価
              def_primitive(id, argvec).unwrap()
            } else {
              newval
            }
          }
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

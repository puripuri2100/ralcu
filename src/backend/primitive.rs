use super::types;

pub fn apply(input: &types::UntypedAST) -> types::UntypedAST {
  let (ast, _) = input;
  match ast {
    types::UntypedASTMain::IntConst(_) => input.clone(),
    types::UntypedASTMain::FloatConst(_) => input.clone(),
    types::UntypedASTMain::BoolConst(_) => input.clone(),
    types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
    types::UntypedASTMain::Apply(utast1, utast2) => input.clone(), //primitives_fn(utast1, utast2),
    types::UntypedASTMain::ContentOf(_, _) => input.clone(),
    types::UntypedASTMain::IfThenElse(b, t, f) => {
      let (b_bool_ast, _) = apply(b);
      let t_ast = apply(t);
      let f_ast = apply(f);
      let b_bool = match b_bool_ast {
        types::UntypedASTMain::BoolConst(b) => b,
        _ => panic!("err ifthenelse"),
      };
      if b_bool {
        t_ast
      } else {
        f_ast
      }
    }
  }
}

fn primitives_bin_fn(
  fn_name: &(String, types::Range),
  inputl: &types::UntypedAST,
  inputr: &types::UntypedAST,
) -> types::UntypedAST {
  let fn_name_str = fn_name.0.as_str();
  match fn_name_str {
    "+" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (types::UntypedASTMain::IntConst(v1), types::UntypedASTMain::IntConst(v2)) => {
          (types::UntypedASTMain::IntConst(v1 + v2), rng1.merge(&rng2))
        }
        _ => panic!(),
      }
    }
    "-" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (types::UntypedASTMain::IntConst(v1), types::UntypedASTMain::IntConst(v2)) => {
          (types::UntypedASTMain::IntConst(v1 - v2), rng1.merge(&rng2))
        }
        _ => panic!(),
      }
    }
    "*" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (types::UntypedASTMain::IntConst(v1), types::UntypedASTMain::IntConst(v2)) => {
          (types::UntypedASTMain::IntConst(v1 * v2), rng1.merge(&rng2))
        }
        _ => panic!(),
      }
    }
    "/" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (types::UntypedASTMain::IntConst(v1), types::UntypedASTMain::IntConst(v2)) => {
          (types::UntypedASTMain::IntConst(v1 / v2), rng1.merge(&rng2))
        }
        _ => panic!(),
      }
    }
    "+." => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (types::UntypedASTMain::FloatConst(v1), types::UntypedASTMain::FloatConst(v2)) => (
          types::UntypedASTMain::FloatConst(v1 + v2),
          rng1.merge(&rng2),
        ),
        _ => panic!(),
      }
    }
    "-." => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (types::UntypedASTMain::FloatConst(v1), types::UntypedASTMain::FloatConst(v2)) => (
          types::UntypedASTMain::FloatConst(v1 - v2),
          rng1.merge(&rng2),
        ),
        _ => panic!(),
      }
    }
    "*." => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (types::UntypedASTMain::FloatConst(v1), types::UntypedASTMain::FloatConst(v2)) => (
          types::UntypedASTMain::FloatConst(v1 * v2),
          rng1.merge(&rng2),
        ),
        _ => panic!(),
      }
    }
    "/." => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (types::UntypedASTMain::FloatConst(v1), types::UntypedASTMain::FloatConst(v2)) => (
          types::UntypedASTMain::FloatConst(v1 / v2),
          rng1.merge(&rng2),
        ),
        _ => panic!(),
      }
    }
    "<" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (types::UntypedASTMain::IntConst(v1), types::UntypedASTMain::IntConst(v2)) => {
          (types::UntypedASTMain::BoolConst(v1 < v2), rng1.merge(&rng2))
        }
        _ => panic!(),
      }
    }
    ">" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (types::UntypedASTMain::IntConst(v1), types::UntypedASTMain::IntConst(v2)) => {
          (types::UntypedASTMain::BoolConst(v1 > v2), rng1.merge(&rng2))
        }
        _ => panic!(),
      }
    }
    "==" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (types::UntypedASTMain::IntConst(v1), types::UntypedASTMain::IntConst(v2)) => (
          types::UntypedASTMain::BoolConst(v1 == v2),
          rng1.merge(&rng2),
        ),
        _ => panic!(),
      }
    }
    _ => panic!(),
  }
}

fn primitives_fn(utastl: &types::UntypedAST, utastr: &types::UntypedAST) -> types::UntypedAST {
  let utastl = apply(utastl);
  let utastr = apply(utastr);
  panic!()
}

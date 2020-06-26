use super::types;

pub fn apply(input: &types::UntypedAST) -> types::UntypedAST {
  let (ast, _) = input;
  match ast {
    types::UntypedASTMain::IntConst(_) => input.clone(),
    types::UntypedASTMain::FloatConst(_) => input.clone(),
    types::UntypedASTMain::BoolConst(_) => input.clone(),
    types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
    types::UntypedASTMain::Apply(name, arg_lst) => primitives_fn(name, arg_lst),
    types::UntypedASTMain::ContentOf(_) => input.clone(),
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
        (types::UntypedASTMain::FloatConst(v1), types::UntypedASTMain::FloatConst(v2)) => {
          (types::UntypedASTMain::FloatConst(v1 + v2), rng1.merge(&rng2))
        }
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
        (types::UntypedASTMain::FloatConst(v1), types::UntypedASTMain::FloatConst(v2)) => {
          (types::UntypedASTMain::FloatConst(v1 - v2), rng1.merge(&rng2))
        }
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
        (types::UntypedASTMain::FloatConst(v1), types::UntypedASTMain::FloatConst(v2)) => {
          (types::UntypedASTMain::FloatConst(v1 * v2), rng1.merge(&rng2))
        }
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
        (types::UntypedASTMain::FloatConst(v1), types::UntypedASTMain::FloatConst(v2)) => {
          (types::UntypedASTMain::FloatConst(v1 / v2), rng1.merge(&rng2))
        }
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
        (types::UntypedASTMain::IntConst(v1), types::UntypedASTMain::IntConst(v2)) => {
          (types::UntypedASTMain::BoolConst(v1 == v2), rng1.merge(&rng2))
        }
        _ => panic!(),
      }
    }
    _ => panic!(),
  }
}


fn primitives_fn(fn_name: &(String, types::Range), input_lst: &Vec<types::UntypedAST>) -> types::UntypedAST {
  let fn_name_str = fn_name.0.as_str();
  match fn_name_str {
    "int" => {
      if input_lst.len() == 1 {
        let ast = apply(&input_lst[0]);
        match ast {
          (types::UntypedASTMain::FloatConst(f),rng) => {
            (types::UntypedASTMain::IntConst(f as i64), rng)
          }
          _ => {panic!()}
        }
      } else {
        panic!()
      }
    }
    "float" => {
      if input_lst.len() == 1 {
        let ast = apply(&input_lst[0]);
        match ast {
          (types::UntypedASTMain::IntConst(i),rng) => {
            (types::UntypedASTMain::FloatConst(i as f64), rng)
          }
          _ => {panic!()}
        }
      } else {
        panic!()
      }
    }
    "sin" => {
      if input_lst.len() == 1 {
        let ast = apply(&input_lst[0]);
        match ast {
          (types::UntypedASTMain::FloatConst(f),rng) => {
            (types::UntypedASTMain::FloatConst(f.sin()), rng)
          }
          _ => {panic!()}
        }
      } else {
        panic!()
      }
    }
    "int_plus" => {
      if input_lst.len() == 2 {
        let ast1 = apply(&input_lst[0]);
        let ast2 = apply(&input_lst[1]);
        match (ast1,ast2) {
          ((types::UntypedASTMain::IntConst(i1),rng1), (types::UntypedASTMain::IntConst(i2),rng2)) => {
            let rng = rng1.merge(&rng2);
            (types::UntypedASTMain::IntConst(i1 + i2), rng)
          }
          _ => {panic!()}
        }
      } else {
        panic!()
      }
    }
    _ => panic!()
  }
}

use core::panic;

use super::types::{
  self, Range, UntypedAST,
  UntypedASTMain::{self, *},
};

pub fn apply(input: &types::UntypedAST) -> UntypedAST {
  let (ast, r) = input;
  match ast {
    UntypedASTMain::IntConst(_) => input.clone(),
    UntypedASTMain::FloatConst(_) => input.clone(),
    UntypedASTMain::BoolConst(_) => input.clone(),
    UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
    UntypedASTMain::App(name, args) => primitives_fn(name, args, r),
    UntypedASTMain::Var(_) => input.clone(),
    UntypedASTMain::IfThenElse(b, t, f) => {
      let (b_bool_ast, _) = apply(b);
      let t_ast = apply(t);
      let f_ast = apply(f);
      let b_bool = match b_bool_ast {
        BoolConst(b) => b,
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
  fn_name: &(String, Range),
  inputl: &UntypedAST,
  inputr: &UntypedAST,
) -> types::UntypedAST {
  let fn_name_str = fn_name.0.as_str();
  match fn_name_str {
    "+" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (IntConst(v1), IntConst(v2)) => (IntConst(v1 + v2), rng1.merge(rng2)),
        _ => panic!(),
      }
    }
    "-" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        types::UntypedASTMain::BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (IntConst(v1), IntConst(v2)) => (IntConst(v1 - v2), rng1.merge(rng2)),
        _ => panic!(),
      }
    }
    "*" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (IntConst(v1), types::UntypedASTMain::IntConst(v2)) => {
          (IntConst(v1 * v2), rng1.merge(rng2))
        }
        _ => panic!(),
      }
    }
    "/" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (IntConst(v1), IntConst(v2)) => (IntConst(v1 / v2), rng1.merge(rng2)),
        _ => panic!(),
      }
    }
    "+." => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (FloatConst(v1), FloatConst(v2)) => (FloatConst(v1 + v2), rng1.merge(rng2)),
        _ => panic!(),
      }
    }
    "-." => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (FloatConst(v1), FloatConst(v2)) => (FloatConst(v1 - v2), rng1.merge(rng2)),
        _ => panic!(),
      }
    }
    "*." => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (FloatConst(v1), FloatConst(v2)) => (FloatConst(v1 * v2), rng1.merge(rng2)),
        _ => panic!(),
      }
    }
    "/." => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (FloatConst(v1), FloatConst(v2)) => (FloatConst(v1 / v2), rng1.merge(rng2)),
        _ => panic!(),
      }
    }
    "<" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (IntConst(v1), IntConst(v2)) => (BoolConst(v1 < v2), rng1.merge(rng2)),
        _ => panic!(),
      }
    }
    ">" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (IntConst(v1), IntConst(v2)) => (BoolConst(v1 > v2), rng1.merge(rng2)),
        _ => panic!(),
      }
    }
    "==" => {
      let (v1_ast, rng1) = inputl;
      let (v2_ast, rng2) = inputr;
      let (v1, _) = match v1_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputl),
      };
      let (v2, _) = match v2_ast {
        BinApply(name, left, right) => primitives_bin_fn(name, left, right),
        _ => apply(inputr),
      };
      match (v1, v2) {
        (IntConst(v1), IntConst(v2)) => (BoolConst(v1 == v2), rng1.merge(rng2)),
        _ => panic!(),
      }
    }
    _ => panic!(),
  }
}

fn primitives_fn(name: &str, args: &[UntypedAST], range: &Range) -> UntypedAST {
  match name {
    "cos" => {
      if let Some(ast) = args.get(0) {
        match apply(ast) {
          (FloatConst(f), _) => (FloatConst(f.cos()), *range),
          _ => panic!("arg err : {}", name),
        }
      } else {
        panic!("arg err : {}", name)
      }
    }
    "sin" => {
      if let Some(ast) = args.get(0) {
        match apply(ast) {
          (FloatConst(f), _) => (FloatConst(f.sin()), *range),
          _ => panic!("arg err : {}", name),
        }
      } else {
        panic!("arg err : {}", name)
      }
    }
    "tab" => {
      if let Some(ast) = args.get(0) {
        match apply(ast) {
          (FloatConst(f), _) => (FloatConst(f.tan()), *range),
          _ => panic!("arg err : {}", name),
        }
      } else {
        panic!("arg err : {}", name)
      }
    }
    "int" => {
      if let Some(ast) = args.get(0) {
        match apply(ast) {
          (FloatConst(f), _) => (IntConst(f as i64), *range),
          _ => panic!("arg err : {}", name),
        }
      } else {
        panic!("arg err : {}", name)
      }
    }
    "float" => {
      if let Some(ast) = args.get(0) {
        match apply(ast) {
          (IntConst(i), _) => (FloatConst(i as f64), *range),
          _ => panic!("arg err : {}", name),
        }
      } else {
        panic!("arg err : {}", name)
      }
    }
    "add" => {
      if let (Some(ast1), Some(ast2)) = (args.get(0), args.get(1)) {
        match (apply(ast1), apply(ast2)) {
          ((IntConst(i1), _), (IntConst(i2), _)) => (IntConst(i1 + i2), *range),
          _ => panic!("arg err : {}", name),
        }
      } else {
        panic!("arg err : {}", name)
      }
    }
    _ => panic!("unsupported function : {}", name),
  }
}

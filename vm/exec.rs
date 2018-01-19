#![allow(unused)]

use std::rc::*;
use std::sync::*;

use std::collections::*;

use eval;
use eval::{LispValue, LispFunction};
use sexp;
use intrinsics::{self, MgIntrinsic, IntrinsicImpl};

#[derive(Clone)]
pub struct LispProgram {
    prelude: bool,
    env: HashMap<String, Rc<LispValue>>
}

impl LispProgram {

    fn new() -> LispProgram {
        LispProgram {
            prelude: true,
            env: HashMap::new()
        }
    }

    fn with_core(mut self) -> LispProgram {
        self
            .with_function("lambda", &intrinsics::core::mgi_lambda)
            .with_function("define", &intrinsics::core::mgi_define)
            .with_function("if", &intrinsics::core::mgi_if)
            .with_function("typeof", &intrinsics::core::mgi_typeof)
            .with_function("begin", &intrinsics::core::mgi_begin)
            .with_function("deepcopy", &intrinsics::core::mgi_hard_clone)
    }

    fn with_function(mut self, name: &str, func: &IntrinsicImpl) -> LispProgram {
        self.env.insert(
            String::from(name),
            Rc::new(LispValue::Func(Box::new(LispFunction::Intrinsic(MgIntrinsic::new(String::from(name), func))))));
        self
    }

    fn exec(self, sexp: Box<sexp::Sexp>) -> Result<Rc<LispValue>, eval::EvalError> {
        unimplemented!()
    }

}

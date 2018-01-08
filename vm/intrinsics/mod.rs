use std::rc::*;
use std::sync::*;

use sexp::Sexp;

use eval::Atom;
use eval::Env;
use eval::EvalError;

pub mod core;
pub mod lists;
pub mod math;
pub mod string;

pub type IntrinsicImpl = Fn(&Vec<Sexp>, &mut Env) -> Result<Rc<Atom>, EvalError>;

#[derive(Clone)]
pub struct MgIntrinsic {
    pub name: String,
    pub func: Arc<IntrinsicImpl>
}

impl MgIntrinsic {
    #[allow(unreachable_code)]
    pub fn new(name: String, _func: &IntrinsicImpl) -> MgIntrinsic {
        MgIntrinsic {
            name: name,
            func: unimplemented!() // getting the &IntrinsicImpl in the arc as not a reference is hard, do it later
        }
    }
}

impl ::std::fmt::Debug for MgIntrinsic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "[intrinsic {}]", self.name)
    }
}

impl PartialEq for MgIntrinsic {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name // This isn't exactly right but there's no other way.
    }
}

impl Eq for MgIntrinsic {}

#[inline]
pub fn intrinsic_error(err: &str) -> Result<Rc<Atom>, EvalError> {
    Err(EvalError::Msg(format!("error: {}", err)))
}

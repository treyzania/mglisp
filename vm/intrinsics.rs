use std::cell::*;
use std::sync::*;

use sexp;

use Atom;
use Env;
use EvalError;

#[derive(Clone)]
pub struct MgIntrinsic {
    pub name: String,
    pub func: Arc<Mutex<FnMut(Vec<sexp::Sexp>, &mut Env) -> Result<Atom, EvalError>>>
}

impl ::std::fmt::Debug for MgIntrinsic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str(format!("[intrinsic {}]", self.name).as_str());
        Ok(())
    }
}

impl PartialEq for MgIntrinsic {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name // This isn't exactly right but there's no other way.
    }
}

impl Eq for MgIntrinsic {}


#![allow(unused, dead_code)]

use std::rc::*;

use std::collections::*;

mod intrinsics;
mod sexp;

use sexp::*; // This `use` feels wrong.

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Env(HashMap<String, Rc<Atom>>);

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LispFunction {
    Lambda(Rc<Sexp>, Env), // the `Env` here is the local context of the function
    Intrinsic(intrinsics::MgIntrinsic)
}

impl From<Sexp> for Atom {
    fn from(s: Sexp) -> Atom {
        use sexp::Sexp::*;
        match s {
            Null => Atom::Null,
            Integer(i) => Atom::Integer(i),
            ByteArray(b) => Atom::ByteArray(b),
            Str(s) => Atom::Str(s),
            Symbol(s) => Atom::Symbol(s),
            List(mut l) => {
                let len = l.len();
                let mut v = Atom::Null;
                for i in 0..len {
                    // Build up the conses in reverse.
                    v = Atom::Cons(Rc::new((l[len - i - 1].clone().into(), v)));
                }
                v
            }
        }
    }
}

/// Some data value.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Atom {

    /// Just nothing.
    Null,

    /// 64-bit integer.
    Integer(i64),

    /// Byte array.
    ByteArray(Box<[i8]>),

    /// UTF-8 string.
    Str(String),

    /// A symbol that's not a string.
    Symbol(String),

    /// A pairing of two values, probably an atom and another cons.
    Cons(Rc<(Atom, Atom)>),

    /// Balls
    Func(LispFunction)

}

impl Env {

    pub fn new() -> Env {
        Env(HashMap::new())
    }

    pub fn compose(&self, top: &Env) -> Env {
        let mut dup = self.0.clone();
        for (k, v) in top.0.iter() {
            dup.insert(k.clone(), v.clone());
        }
        Env(dup)
    }

}

#[derive(Clone, Debug)]
enum EvalError {
    Msg(String),
    Chain(Vec<EvalError>)
}

fn eval(func: LispFunction, env: Env) -> Atom {
    unimplemented!(); // I changed the data model again and broke everything.
}

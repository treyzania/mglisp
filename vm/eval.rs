
#![allow(dead_code)]

use std::rc::*;
use std::collections::*;

use intrinsics;
use sexp::Sexp;

type BindingMap = HashMap<String, Rc<Atom>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Env {
    bindings: BindingMap,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LispFunction {
    Lambda(Rc<Sexp>, Env, Vec<String>), // the `Env` here is the local context of the function
    Intrinsic(intrinsics::MgIntrinsic)
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

    /// A boolean value.
    Boolean(bool),

    /// A symbol that's not a string.
    Symbol(String),

    /// A pairing of two values, probably an atom and another cons.
    Cons(Rc<(Atom, Atom)>),

    /// Balls
    Func(LispFunction)

}

impl Env {
    pub fn new() -> Env {
        Env {
            bindings: BindingMap::new()
        }
    }

    pub fn add_binding(&mut self, name: String, value: Rc<Atom>) {
        self.bindings.insert(name, value);
    }

    pub fn compose(&self, top: &Env) -> Env {
        let mut dup = self.bindings.clone();
        for (k, v) in top.bindings.iter() {
            dup.insert(k.clone(), v.clone());
        }
        Env {
            bindings: dup
        }
    }

    pub fn resolve(&self, name: &String) -> Option<Rc<Atom>> {
        self.bindings.get(name).cloned()
    }
}

impl From<BindingMap> for Env {
    fn from(v: BindingMap) -> Env {
        Env {
            bindings: v
        }
    }
}

#[derive(Clone, Debug)]
pub enum EvalError {
    Msg(String),
    Chain(Vec<EvalError>)
}

pub fn eval(sexp: &Sexp, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

    use sexp::Sexp::*;
    use self::LispFunction::*;
    use self::EvalError::*;
    let val: Rc<Atom> = match sexp {

        // Normal data conversions.
        &Null => Rc::new(Atom::Null),
        &Integer(i) => Rc::new(Atom::Integer(i)),
        &ByteArray(ref a) => Rc::new(Atom::ByteArray(a.clone())),
        &Str(ref s) => Rc::new(Atom::Str(s.clone())),
        &Boolean(b) => Rc::new(Atom::Boolean(b)),

        // Symbols are how variable binding works, outside of `quote` forms.
        &Symbol(ref s) => match env.resolve(&s) {
            Some(v) => v,
            None => return Err(Msg(format!("unbound name {}", s)))
        },

        // "Lists" in S-expressions are how function calls happen.  The first argument is the actual function being applied.
        &List(ref v) => { // This is where the fun part of evaling works.

            // First we evaluate the first element so that we can figure out what we should do.
            let func = eval(&v[0], &mut env.clone())?;

            // Depending on the type of function we have to do some more work.
            match func.as_ref() {

                // This is where most of the function calls will be.
                &Atom::Func(Lambda(ref tmplt, ref clos, ref names)) => {

                    // First we have to
                    // TODO Make this more f u n c t i o n a l.
                    let mut args = Vec::with_capacity(v.len() - 1);
                    for sx in v.iter().skip(1) {
                        args.push(eval(sx, &mut env.clone())?);
                    }

                    // If they aren't the same length then report that.
                    if args.len() != names.len() {
                        return Err(Msg(format!("function expeced {} arguments, got {}", names.len(), args.len())));
                    }

                    // Now we have to compute the partial environment based on the arguments.
                    let nenv: BindingMap = args
                        .iter()
                        .zip(names)
                        .map(|(a, n)| (n.clone(), a.clone())) // TODO Reduce all this cloning.
                        .collect();

                    // This is where all the hardcore magic happens.
                    eval(tmplt.as_ref(), &mut clos.compose(&nenv.into()))?

                },

                // Intrinsic functions are how we call out and do other special things outside the sandbox.
                &Atom::Func(Intrinsic(ref idat)) => {

                    // Similar thing to the above, just don't do any transformation.
                    let args = v.iter().map(|sx| sx.clone()).collect();
                    match idat.func.as_ref()(&args, env) {
                        Ok(a) => a,
                        Err(e) => return Err(Chain(vec![Msg(format!("error in intrinsic {}", idat.name)), e]))
                    }

                },
                _ => return Err(Msg("tried to call a non-function".into()))
            }
        },
        _ => return Err(Msg("unevaluatable S-expression".into()))
    };

    Ok(val)

}

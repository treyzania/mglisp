
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

impl From<HashMap<String, Rc<Atom>>> for Env {
    fn from(v: HashMap<String, Rc<Atom>>) -> Env {
        Env(v)
    }
}

#[derive(Clone, Debug)]
pub enum EvalError {
    Msg(String),
    Chain(Vec<EvalError>)
}

pub fn eval(sexp: Sexp, env: &mut Env) -> Result<Atom, EvalError> {

    use sexp::Sexp::*;
    use self::LispFunction::*;
    use self::EvalError::*;
    let val: Atom = match sexp {
        Null => Atom::Null,
        Integer(i) => Atom::Integer(i),
        ByteArray(a) => Atom::ByteArray(a),
        Str(s) => Atom::Str(s),
        Quote(sx) => unimplemented!(),
        List(ref v) => { // This is where the fun part of evaling works.

            // First we evaluate the first element so that we can figure out what we should do.
            let func = eval(v[0].clone(), &mut env.clone())?;

            // Depending on the type of function we have to do some more work.
            match func {
                Atom::Func(Lambda(tmplt, clos, names)) => {

                    // First we have to
                    // TODO Make this more f u n c t i o n a l.
                    let mut args = Vec::with_capacity(v.len() - 1);
                    for sx in v.iter().skip(1) {
                        args.push(eval(sx.clone(), &mut env.clone())?);
                    }

                    // If they aren't the same length then report that.
                    if args.len() != names.len() {
                        return Err(Msg(format!("function expeced {} arguments, got {}", names.len(), args.len())));
                    }

                    // Now we have to compute the partial environment based on the arguments.
                    let nenv: HashMap<String, Rc<Atom>> = args
                        .iter()
                        .zip(names)
                        .map(|(a, n)| (n, Rc::new(a.clone())))
                        .collect();

                    // This is where all the hardcore magic happens.
                    eval(tmplt.as_ref().clone(), &mut clos.compose(&nenv.into()))?

                },
                Atom::Func(Intrinsic(idat)) => {

                    // Similar thing to the above, just don't do any transformation.
                    let args = v.iter().skip(1).map(|sx| sx.clone()).collect();
                    match idat.func.as_ref()(args, env) {
                        Ok(a) => a,
                        Err(e) => return Err(Chain(vec![Msg(format!("error in intrinsic {}", idat.name)), e]))
                    }

                },
                _ => return Err(Msg("tried to call a non-function".into()))
            }
        },
        _ => return Err(EvalError::Msg("unevaluatable S-expression".into()))
    };

    Ok(val)

}

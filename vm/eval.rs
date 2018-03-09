
#![allow(dead_code)]

use std::rc::*;
use std::collections::*;

use intrinsics;
use parser::sexp::Sexp;

type BindingMap = HashMap<String, Rc<LispValue>>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Env {
    bindings: BindingMap,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LispFunction {
    Lambda(Rc<Sexp>, Env, Vec<String>), // the `Env` here is the local context of the function
    VariadicLambda(Rc<Sexp>, Env),
    Intrinsic(intrinsics::MgIntrinsic)
}

/// Some data value.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum LispValue {

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

    /// A pairing of two values, probably an LispValue and another cons.
    Cons(Rc<LispValue>, Rc<LispValue>),

    /// Callable functions.  In a `Box` to take up less space, as they're somewhat larger than we want them to be.
    Func(Box<LispFunction>)

}

impl LispValue {

    /// Returns a new, exact, but seperate copy of the LispValue.
    pub fn hard_clone(&self) -> Rc<LispValue> {
        use self::LispValue::*;
        match self {
            &Null => Rc::new(Null),
            &Integer(i) => Rc::new(Integer(i)),
            &ByteArray(ref a) => Rc::new(ByteArray(a.clone())),
            &Str(ref s) => Rc::new(Str(s.clone())),
            &Boolean(b) => Rc::new(Boolean(b)),
            &Symbol(ref s) => Rc::new(Symbol(s.clone())),
            &Cons(ref l, ref r) => Rc::new(Cons(l.hard_clone(), r.hard_clone())),
            &Func(ref f) => Rc::new(Func(f.clone()))
        }
    }

}

impl Env {
    pub fn new() -> Env {
        Env {
            bindings: BindingMap::new()
        }
    }

    pub fn add_binding(&mut self, name: String, value: Rc<LispValue>) {
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

    pub fn resolve(&self, name: &String) -> Option<Rc<LispValue>> {
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

#[allow(unreachable_patterns)]
pub fn eval(sexp: &Sexp, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    use parser::sexp::Sexp::*;
    use self::LispFunction::*;
    use self::EvalError::*;
    let val: Rc<LispValue> = match sexp {

        // Normal data conversions.
        &Null => Rc::new(LispValue::Null),
        &Integer(i) => Rc::new(LispValue::Integer(i)),
        &ByteArray(ref a) => Rc::new(LispValue::ByteArray(a.clone())),
        &Str(ref s) => Rc::new(LispValue::Str(s.clone())),
        &Boolean(b) => Rc::new(LispValue::Boolean(b)),

        // Symbols are how variable binding works, outside of `quote` forms.
        &Symbol(ref s) => match env.resolve(&s) {
            Some(v) => v,
            None => return Err(Msg(format!("unbound name {}", s)))
        },

        /*
         * "Lists" in S-expressions are how function calls happen.  The first argument is the
         * actual function being applied.  This is where the fun part of evaling happens!
         */
        &List(ref v) => match eval(&v[0], &mut env.clone()) {

            Ok(r) => {

                /*
                 * First we evaluate the first element so that we can figure out what we should do, and
                 * if it matches, we figure out which kind of function it is.
                 */
                if let &LispValue::Func(ref fb) = r.as_ref() {

                    match fb.as_ref() {

                        // Lambdas are for normal functions defined within the engine.
                        &Lambda(ref tmplt, ref clos, ref names) => {

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

                        &VariadicLambda(ref tmplt, ref clos) => {

                            // Construct a list out of the arguments, ignoring length.
                            let mut val = Rc::new(LispValue::Null);
                            for v in v.iter().skip(1) {
                                val = Rc::new(LispValue::Cons(eval(v, &mut env.clone())?, val));
                            }

                            // Create the eval override and then complete it.
                            let mut arg = Env::new();
                            arg.add_binding("args".into(), val); // Should we change the argument name?
                            eval(tmplt.as_ref(), &mut clos.compose(&arg))?

                        }

                        // Instrinsics are the things that actually reach out and do magic things.
                        &Intrinsic(ref idat) => {

                            // Similar thing to the above, just don't do any transformation.
                            let args = v.iter().map(|sx| sx.clone()).collect();
                            match idat.func.as_ref()(&args, env) {
                                Ok(a) => a,
                                Err(e) => return Err(Chain(vec![Msg(format!("error in intrinsic {}", idat.name)), e]))
                            }

                        }

                    }

                } else {
                    return Err(Msg("tried to call a non-function".into()))
                }

            },

            Err(e) => return Err(e)

        },
        _ => return Err(Msg("unevaluatable S-expression".into()))
    };

    Ok(val)

}

#[cfg(test)]
mod tests {

    use std::rc::Rc;
    use parser;
    use parser::sexp::Sexp::*;
    use super::{Env, LispFunction, LispValue};
    use intrinsics;

    #[test]
    fn test_eval_nothing() {
        let r = super::eval(&Null, &mut Env::new());
        assert_eq!(r.unwrap(), Rc::new(LispValue::Null));
    }

    #[test]
    fn test_eval_math() {
        use parser::Token;
        use parser::sexp::Sexp;
        use intrinsics::{self, MgIntrinsic};
        let expr: Sexp = parser::parse(&mut vec![
            Token::OpenParen,
            Token::Name("+".into()),
            Token::OpenParen,
            Token::Name("+".into()),
            Token::Number(2),
            Token::Number(3),
            Token::CloseParen,
            Token::Number(2),
            Token::CloseParen
            ].iter().cloned().peekable()).unwrap();
        println!("{:?}", expr);
        let mut env = Env::new();
        env.add_binding("+".into(), Rc::new(LispValue::Func(Box::new(LispFunction::Intrinsic(MgIntrinsic::new("+".into(), &intrinsics::math::mgi_plus))))));
        let r = super::eval(&expr, &mut env);
        assert_eq!(r.unwrap(), Rc::new(LispValue::Integer(7)));
    }

}

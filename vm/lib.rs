
#![allow(unused, dead_code)]

use std::rc::*;

use std::collections::*;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Env(HashMap<String, Rc<Atom>>);

/// Some data value.
#[derive(Clone, Eq, PartialEq, Debug)]
enum Atom {

    /// Just nothing.
    Null,

    /// 64-bit integer.
    Integer(i64),

    /// Byte array.
    ByteArray(Box<[i8]>),

    /// UTF-8 string.
    Str(String),

    /// Index into VM symbols table.
    Symbol(String),

    /// A pairing of two values, probably an atom and another cons.
    Cons(Rc<(Atom, Atom)>),

    /// Evaluation context thingy.
    Closure(Box<Atom>, Env) // probably not the right way to represent this

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

struct Closure {
    context: Env,
    expr: Atom // Probably a series of conses and things.
}

#[derive(Clone, Debug)]
enum EvalError {
    Msg(String),
    Chain(Vec<EvalError>)
}

fn eval(expr: Rc<Atom>, env: &Env) -> Result<Rc<Atom>, EvalError> {

    use self::Atom::*;

    let e = expr.as_ref();

    let val = match e.clone() {
        Null => Null,
        Integer(i) => Integer(i),
        ByteArray(a) => ByteArray(a),
        Str(s) => Str(s),
        Symbol(s) => Symbol(s),
        Cons(c) => Cons(c),
        _ => return Err(EvalError::Msg("this didn't work".into()))
    };

    Ok(Rc::new(val))

}

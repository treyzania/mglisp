
/*
 * This is needed since we don't actually directly use most of these functions.  Arguably they
 * could be moved to their own crate
 */
#![allow(dead_code)]

use std::rc::*;

use eval::{Atom, Env, eval, EvalError, LispFunction};
use sexp::Sexp;

use intrinsics::*;

pub fn mgi_lambda(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

    if args.len() != 2 {
        return intrinsic_error("invalid form for lambda, needs 2 expressions");
    }

    let mut names = Vec::new();
    match &args[0] {
        &Sexp::List(ref list) => for sexp in list {
            match sexp {
                &Sexp::Symbol(ref s) => names.push(s.clone()),
                _ => return intrinsic_error("invalid form for lambda, malformed argument names"),
            }
        },
        _ => return intrinsic_error("invalid form for lambda, first argument not list"),
    };

    Ok(Rc::new(Atom::Func(LispFunction::Lambda(Rc::new(args[1].clone()), env.clone(), names))))

}

pub fn mgi_define(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

    if args.len() != 2 {
        return intrinsic_error("invalid form for define, needs 2 expressions");
    }

    let binding = match &args[0] {
        &Sexp::Symbol(ref s) => s.clone(),
        _ => return intrinsic_error("invalid form for define, first argument is not symbol"),
    };

    let value = eval(&args[1], &mut env.clone())?;

    // This is where we actually mutate the environment.
    env.add_binding(binding, value);

    Ok(Rc::new(Atom::Null))

}

pub fn mgi_if(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

    if args.len() != 3 {
        return intrinsic_error("invalid form for if, needs 3 expressions");
    }

    let cond = match eval(&args[0], env) {
        Ok(v) => v,
        e @ Err(_) => return e
    };

    match cond.as_ref() {
        &Atom::Boolean(true) => eval(&args[1], env),
        &Atom::Boolean(false) => eval(&args[2], env),
        _ => return intrinsic_error("conditional expression in if is non-boolean")
    }

}

pub fn mgi_typeof(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

    use eval::Atom::*;

    if args.len() != 1 {
        return intrinsic_error("typeof takes 1 argument");
    }

    // TODO Does this need to be cloned?
    Ok(Rc::new(Symbol(match (eval(&args[0], &mut env.clone())?).as_ref() {
        &Null => "null",
        &Integer(_) => "integer",
        &ByteArray(_) => "bytearray",
        &Str(_) => "str",
        &Boolean(_) => "bool",
        &Symbol(_) => "symbol",
        &Cons(_, _) => "cons",
        &Func(_) => "function"
    }.into())))

}

pub fn mgi_begin(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

    let mut last = None;

    /*
     * We just have to evaluate each of the entries in order.  Using the same env because that's
     * just how the semantics of `begin` works.
     */
    for i in args {
        last = Some(eval(i, env)?);
    }

    match last {
        Some(v) => Ok(v),
        None => Ok(Rc::new(Atom::Null))
    }

}

pub fn mgi_hard_clone(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {
    if args.len() != 1 {
        intrinsic_error("invalid form for hardclone, needs 1 expression")
    } else {
        eval(&args[0], env).map(|v| v.as_ref().hard_clone())
    }
}

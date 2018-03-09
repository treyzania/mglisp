
/*
 * This is needed since we don't actually directly use most of these functions.  Arguably they
 * could be moved to their own crate
 */
#![allow(dead_code)]

use std::rc::*;

use eval::{LispValue, Env, eval, EvalError, LispFunction};
use parser::sexp::Sexp;

use intrinsics::*;

pub fn mgi_lambda(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    if args.len() != 3 {
        return intrinsic_error("invalid form for lambda, needs 2 expressions");
    }

    let mut names = Vec::new();
    match &args[1] {
        &Sexp::List(ref list) => for sexp in list {
            match sexp {
                &Sexp::Symbol(ref s) => names.push(s.clone()),
                _ => return intrinsic_error("invalid form for lambda, malformed argument names"),
            }
        },
        _ => return intrinsic_error("invalid form for lambda, first argument not list"),
    };

    // TODO Make this pretty to read.
    Ok(Rc::new(LispValue::Func(Box::new(LispFunction::Lambda(Rc::new(args[2].clone()), env.clone(), names)))))

}

pub fn mgi_variadic_lambda(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    if args.len() != 2 {
        return intrinsic_error("invalid form for vlambda, needs 1 expression");
    }

    // Interestingly, this is even simpler than the non-variadic one.  Although the line is longer.
    Ok(Rc::new(LispValue::Func(Box::new(LispFunction::VariadicLambda(Rc::new(args[1].clone()), env.clone())))))

}

pub fn mgi_define(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    if args.len() != 3 {
        return intrinsic_error("invalid form for define, needs 2 expressions");
    }

    let binding = match &args[1] {
        &Sexp::Symbol(ref s) => s.clone(),
        _ => return intrinsic_error("invalid form for define, first argument is not symbol"),
    };

    let value = eval(&args[2], &mut env.clone())?;

    // This is where we actually mutate the environment.
    env.add_binding(binding, value);

    Ok(Rc::new(LispValue::Null))

}

pub fn mgi_if(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    if args.len() != 4 {
        return intrinsic_error("invalid form for if, needs 3 expressions");
    }

    let cond = match eval(&args[1], env) {
        Ok(v) => v,
        e @ Err(_) => return e
    };

    match cond.as_ref() {
        &LispValue::Boolean(true) => eval(&args[2], env),
        &LispValue::Boolean(false) => eval(&args[3], env),
        _ => return intrinsic_error("conditional expression in if is non-boolean")
    }

}

pub fn mgi_typeof(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    use eval::LispValue::*;

    if args.len() != 2 {
        return intrinsic_error("typeof takes 1 argument");
    }

    // TODO Does this need to be cloned?
    Ok(Rc::new(Symbol(match (eval(&args[1], &mut env.clone())?).as_ref() {
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

pub fn mgi_begin(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

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
        None => Ok(Rc::new(LispValue::Null))
    }

}

pub fn mgi_hard_clone(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {
    if args.len() != 2 {
        intrinsic_error("invalid form for hardclone, needs 1 expression")
    } else {
        eval(&args[1], env).map(|v| v.as_ref().hard_clone())
    }
}

#[inline]
fn convert_sexp_to_lispvalue_literally(s: &Sexp) -> Rc<LispValue> {
    use parser::sexp::Sexp::*;
    Rc::new(match s {
        &Null => LispValue::Null,
        &Integer(i) => LispValue::Integer(i),
        &ByteArray(ref a) => LispValue::ByteArray(a.clone()),
        &Str(ref s) => LispValue::Str(s.clone()),
        &Boolean(b) => LispValue::Boolean(b),
        &Symbol(ref s) => LispValue::Symbol(s.clone()),
        &List(ref l) => {
            // Do some weird reverse-traversal to build up the list structure.
            let mut c = Rc::new(LispValue::Null);
            for i in 1..l.len() {
                let iv = &l[l.len() - i - 1];
                c = Rc::new(LispValue::Cons(convert_sexp_to_lispvalue_literally(iv), c));
            }
            return c; // Not pretty, but works.
        }
    })
}

pub fn mgi_quote(args: &Vec<Sexp>, _env: &mut Env) -> Result<Rc<LispValue>, EvalError> {
    if args.len() != 2 {
        intrinsic_error("invalid form for quote, needs 1 expression")
    } else {
        Ok(convert_sexp_to_lispvalue_literally(&args[1]))
    }
}

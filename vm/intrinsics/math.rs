
/*
 * This is needed since we don't actually directly use most of these functions.  Arguably they
 * could be moved to their own crate
 */
#![allow(dead_code)]

use std::rc::*;

use eval::*;
use intrinsics::*;

use eval::LispValue::*;

pub fn mgi_plus(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    if args.len() != 2 {
        return intrinsic_error("invalid form for '+', needs 2 expressions");
    }

    // TODO Do these need to be cloned?
    let av = eval(&args[0], &mut env.clone())?;
    let bv = eval(&args[1], &mut env.clone())?;

    match (av.as_ref(), bv.as_ref()) {
        (&Integer(a), &Integer(b)) => Ok(Rc::new(Integer(a + b))),
        (&Integer(_), _) => intrinsic_error("argument 2 for '+' is not an integer"),
        (_, &Integer(_)) => intrinsic_error("argument 1 for '+' is not an integer"),
        (_, _) => intrinsic_error("arguments 1 and 2 for '+' are not integers")
    }

}

pub fn mgi_subtract(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    if args.len() != 2 {
        return intrinsic_error("invalid form for '-', needs 2 expressions");
    }

    // TODO Do these need to be cloned?
    let av = eval(&args[0], &mut env.clone())?;
    let bv = eval(&args[1], &mut env.clone())?;

    match (av.as_ref(), bv.as_ref()) {
        (&Integer(a), &Integer(b)) => Ok(Rc::new(Integer(a - b))),
        (&Integer(_), _) => intrinsic_error("argument 2 for '-' is not an integer"),
        (_, &Integer(_)) => intrinsic_error("argument 1 for '-' is not an integer"),
        (_, _) => intrinsic_error("arguments 1 and 2 for '-' are not integers")
    }

}

pub fn mgi_multiply(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    if args.len() != 2 {
        return intrinsic_error("invalid form for '*', needs 2 expressions");
    }

    // TODO Do these need to be cloned?
    let av = eval(&args[0], &mut env.clone())?;
    let bv = eval(&args[1], &mut env.clone())?;

    match (av.as_ref(), bv.as_ref()) {
        (&Integer(a), &Integer(b)) => Ok(Rc::new(Integer(a * b))),
        (&Integer(_), _) => intrinsic_error("argument 2 for '*' is not an integer"),
        (_, &Integer(_)) => intrinsic_error("argument 1 for '*' is not an integer"),
        (_, _) => intrinsic_error("arguments 1 and 2 for '*' are not integers")
    }

}

pub fn mgi_divide(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    if args.len() != 2 {
        return intrinsic_error("invalid form for '/', needs 2 expressions");
    }

    // TODO Do these need to be cloned?
    let av = eval(&args[0], &mut env.clone())?;
    let bv = eval(&args[1], &mut env.clone())?;

    // TODO This will change eventually, since we're adding floating-point ops later.
    match (av.as_ref(), bv.as_ref()) {
        (&Integer(a), &Integer(b)) => Ok(Rc::new(Integer(a / b))),
        (&Integer(_), _) => intrinsic_error("argument 2 for '/' is not an integer"),
        (_, &Integer(_)) => intrinsic_error("argument 1 for '/' is not an integer"),
        (_, _) => intrinsic_error("arguments 1 and 2 for '/' are not integers")
    }

}

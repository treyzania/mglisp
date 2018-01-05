
/*
 * This is needed since we don't actually directly use most of these functions.  Arguably they
 * could be moved to their own crate
 */
#![allow(dead_code)]

use std::rc::*;

use eval::*;
use intrinsics::*;

pub fn mgi_cons(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

    if args.len() != 2 {
        return intrinsic_error("invalid form for cons, needs 2 expressions");
    }

    // We don't actually care if the "rest" is a list or not.
    // TODO Do these need to be cloned?
    let first = eval(&args[0], &mut env.clone())?;
    let rest = eval(&args[1], &mut env.clone())?;
    Ok(Rc::new(Atom::Cons(first, rest)))

}


pub fn mgi_first(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

    if args.len() != 1 {
        return intrinsic_error("first expects 1 argument");
    }

    match (eval(&args[0], &mut env.clone())?).as_ref() { // TODO Does this need to be cloned?
        &Atom::Cons(ref f, _) => Ok(f.clone()),
        _ => intrinsic_error("first expects a cons")
    }

}

pub fn mgi_rest(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

    if args.len() != 1 {
        return intrinsic_error("rest expects 1 argument");
    }

    match (eval(&args[0], &mut env.clone())?).as_ref() { // TODO Does this need to be cloned?
        &Atom::Cons(_, ref r) => Ok(r.clone()),
        _ => intrinsic_error("first expects a cons")
    }

}


/*
 * This is needed since we don't actually directly use most of these functions.  Arguably they
 * could be moved to their own crate
 */
#![allow(dead_code)]

use std::rc::*;

use eval::*;
use intrinsics::*;

use eval::LispValue::*;

pub fn mgi_str_len(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    if args.len() != 1 {
        return intrinsic_error("str-len takes 1 argument");
    }

    // TODO Does this need to be cloned?
    match (eval(&args[0], &mut env.clone())?).as_ref() {
        &Str(ref s) => Ok(Rc::new(Integer(s.len() as i64))),
        _ => intrinsic_error("argument to str-len must be str")
    }

}

pub fn mgi_str_app(args: &Vec<Sexp>, env: &mut Env) -> Result<Rc<LispValue>, EvalError> {

    if args.len() != 2 {
        return intrinsic_error("str-app takes 2 arguments");
    }

    // TODO Do these need to be cloned?
    let av = eval(&args[0], &mut env.clone())?;
    let bv = eval(&args[1], &mut env.clone())?;

    match (av.as_ref(), bv.as_ref()) {
        (&Str(ref a), &Str(ref b)) => Ok(Rc::new(Str({
            let mut c = a.clone();
            c.push_str(b.as_str());
            c
        }))),
        (&Str(_), _) => intrinsic_error("argument 2 for str-app is not a str"),
        (_, &Str(_)) => intrinsic_error("argument 1 for str-app is not a str"),
        (_, _) => intrinsic_error("arguments 1 and 2 for str-app are not str")
    }

}

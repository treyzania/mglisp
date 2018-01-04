use std::cell::*;
use std::rc::*;
use std::sync::*;

use sexp::Sexp;

use Atom;
use Env;
use EvalError;

type IntrinsicImpl = Fn(Vec<Sexp>, &mut Env) -> Result<Rc<Atom>, EvalError>;

#[derive(Clone)]
pub struct MgIntrinsic {
    pub name: String,
    pub func: Arc<IntrinsicImpl>
}

impl ::std::fmt::Debug for MgIntrinsic {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str(format!("[intrinsic {}]", self.name).as_str());
        Ok(())
    }
}

impl PartialEq for MgIntrinsic {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name // This isn't exactly right but there's no other way.
    }
}

impl Eq for MgIntrinsic {}

mod core {

    use std::rc::*;

    use sexp::Sexp;

    use Atom;
    use LispFunction;
    use Env;
    use EvalError;
    use eval;

    #[inline]
    fn eval_error(err: &str) -> Result<Rc<Atom>, EvalError> {
        Err(EvalError::Msg(String::from(err)))
    }

    pub fn mgi_lambda(args: Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

        if args.len() != 2 {
            return eval_error("invalid form for lambda: needs 2 expressions");
        }

        let mut names = Vec::new();
        match args[0].clone() {
            Sexp::List(list) => for sexp in list {
                match sexp {
                    Sexp::Symbol(s) => names.push(s),
                    _ => return eval_error("invalid form for lambda: malformed argument names"),
                }
            },
            _ => return eval_error("invalid form for lambda: first argument not list"),
        };

        Ok(Rc::new(Atom::Func(LispFunction::Lambda(Rc::new(args[1].clone()), env.clone(), names))))

    }

    pub fn mgi_define(args: Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

        if args.len() != 2 {
            return eval_error("invalid form for define: needs 2 expressions");
        }

        let binding = match args[0].clone() {
            Sexp::Symbol(s) => s,
            _ => return eval_error("invalid form for define: first argument is not symbol"),
        };

        let value = eval(args[1].clone(), &mut env.clone())?;

        // This is where we actually mutate the environment.
        env.add_binding(binding, value);

        Ok(Rc::new(Atom::Null))

    }

    pub fn mgi_if(args: Vec<Sexp>, env: &mut Env) -> Result<Rc<Atom>, EvalError> {

        if args.len() != 3 {
            return eval_error("invalid form for if: needs 3 expressions");
        }

        let cond = match eval(args[0].clone(), env) {
            Ok(v) => v,
            e @ Err(_) => return e
        };

        match cond.as_ref() {
            &Atom::Boolean(true) => eval(args[1].clone(), env),
            &Atom::Boolean(false) => eval(args[2].clone(), env),
            _ => return eval_error("conditional expression in if is non-boolean")
        }

    }

}

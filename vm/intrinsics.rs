use std::cell::*;
use std::sync::*;

use sexp;

use Atom;
use Env;
use EvalError;

#[derive(Clone)]
pub struct MgIntrinsic {
    pub name: String,
    pub func: Arc<Fn(Vec<sexp::Sexp>, &mut Env) -> Result<Atom, EvalError>>
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

    pub fn mgi_lambda(args: Vec<Sexp>, env: &mut Env) -> Result<Atom, EvalError> {

        if args.len() != 2 {
            return Err(EvalError::Msg("invalid form for lambda".into()));
        }

        let mut names = Vec::new();
        match args[0].clone() {
            Sexp::List(l) => for a in l {
                match a {
                    Sexp::Symbol(s) => names.push(s),
                    _ => return Err(EvalError::Msg("invalid form for lambda".into()))
                }
            },
            _ => return Err(EvalError::Msg("invalid form for lambda".into()))
        };

        Ok(Atom::Func(LispFunction::Lambda(Rc::new(args[1].clone()), env.clone(), names)))

    }

    pub fn mgi_define(args: Vec<Sexp>, env: &mut Env) -> Result<Atom, EvalError> {

        if args.len() != 2 {
            return Err(EvalError::Msg("invalid form for define".into()));
        }

        let binding = match args[0].clone() {
            Sexp::Symbol(s) => s,
            _ => return Err(EvalError::Msg("invalid form for define".into()))
        };
        let value = eval(args[1].clone(), &mut env.clone())?;

        // This is where we actually mutate the environment.
        env.0.insert(binding, Rc::new(value)); // FIXME Make this not shit.

        Ok(Atom::Null)

    }

}

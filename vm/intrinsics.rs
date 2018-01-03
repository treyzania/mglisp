use std::cell::*;
use std::sync::*;

use Atom;

#[derive(Clone)]
pub struct MgIntrinsic {
    name: String,
    func: Arc<Mutex<FnMut(Vec<Atom>) -> Atom>>
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

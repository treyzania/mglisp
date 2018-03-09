
#![allow(unused)]

/// Some data value.
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum Sexp {

    /// Just nothing.
    Null,

    /// 64-bit integer.
    Integer(i64),

    /// Byte array.
    ByteArray(Box<[i8]>),

    /// UTF-8 string.
    Str(String),

    /// A boolean value
    Boolean(bool),

    /// A symbol that's not a string.
    Symbol(String),

    /// List of S-expresions.
    List(Vec<Sexp>),

}

impl Sexp {

    pub fn symb_str(s: &str) -> Sexp {
        Sexp::Symbol(String::from(s))
    }

    pub fn str_str(s: &str) -> Sexp {
        Sexp::Str(String::from(s))
    }

}

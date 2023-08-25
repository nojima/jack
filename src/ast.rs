use std::{
    collections::HashMap,
    fmt::{Debug, Error, Formatter},
};

pub enum Expr {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Expr>),
    Dict(HashMap<String, Expr>),
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self {
            Expr::Null => write!(fmt, "null"),
            Expr::Bool(b) => write!(fmt, "{:?}", b),
            Expr::Number(n) => write!(fmt, "{:?}", n),
            Expr::String(s) => write!(fmt, "{s:?}"),

            Expr::Array(v) => {
                write!(fmt, "[")?;
                let mut first = true;
                for x in v {
                    if first {
                        first = false;
                    } else {
                        write!(fmt, ", ")?;
                    }
                    write!(fmt, "{x:?}")?;
                }
                write!(fmt, "]")
            }

            Expr::Dict(dict) => {
                write!(fmt, "{{")?;

                // sort elements by key to fix iteration order.
                let mut seq: Vec<(&String, &Expr)> = dict.iter().collect();
                seq.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));

                let mut first = true;
                for (k, v) in seq {
                    if first {
                        first = false;
                    } else {
                        write!(fmt, ", ")?;
                    }
                    write!(fmt, "{k:?}: {v:?}")?;
                }
                write!(fmt, "}}")
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Opcode {
    Add,
    Sub,
    Mul,
    Div,
}

impl Debug for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
        }
    }
}

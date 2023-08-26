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

    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Expr::Null => write!(f, "null"),
            Expr::Bool(b) => write!(f, "{:?}", b),
            Expr::Number(n) => write!(f, "{:?}", n),
            Expr::String(s) => write!(f, "{s:?}"),

            Expr::Array(v) => {
                write!(f, "[")?;
                let mut first = true;
                for x in v {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{x:?}")?;
                }
                write!(f, "]")
            }

            Expr::Dict(dict) => {
                write!(f, "{{")?;

                // sort elements by key to fix iteration order.
                let mut seq: Vec<(&String, &Expr)> = dict.iter().collect();
                seq.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));

                let mut first = true;
                for (k, v) in seq {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{k:?}: {v:?}")?;
                }
                write!(f, "}}")
            }

            Expr::UnaryOp(op, expr) => write!(f, "{op:?}({expr:?})"),
            Expr::BinaryOp(op, lhs, rhs) => write!(f, "{op:?}({lhs:?}, {rhs:?})"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

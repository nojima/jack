use std::fmt::{Debug, Formatter, Error};

pub enum Expr {
    Null,
    Bool(bool),
    Number(f64),
    Array(Vec<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Expr::*;
        match *self {
            Null => write!(fmt, "null"),
            Bool(b) => write!(fmt, "{:?}", b),
            Number(n) => write!(fmt, "{:?}", n),
            Array(ref v) => {
                write!(fmt, "[")?;
                let mut first = true;
                for x in v {
                    if first {
                        first = false;
                    } else {
                        write!(fmt, ", ")?;
                    }
                    write!(fmt, "{:?}", x)?;
                }
                write!(fmt, "]")
            },
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

use std::fmt::{Debug, Error, Formatter, Display};
use std::rc::Rc;

use compact_str::CompactString;

use crate::symbol::Symbol;

#[derive(Clone)]
pub enum Expr {
    Null,
    Bool(bool),
    Number(f64),
    String(Rc<String>),
    Array(Vec<Expr>),
    Dict(Vec<(CompactString, Expr)>),
    Function(Vec<(Symbol, TypeExpr)>, Box<Expr>),

    Variable(Symbol),

    UnaryOp(UnaryOp, Box<Expr>),
    BinaryOp(BinaryOp, Box<Expr>, Box<Expr>),

    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Local(Symbol, Box<Expr>, Box<Expr>),

    FunctionCall(Box<Expr>, Vec<Expr>),
    FieldAccess(Box<Expr>, Symbol),
    IndexAccess(Box<Expr>, Box<Expr>),
}

impl Debug for Expr {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            Expr::Null => write!(f, "null"),
            Expr::Bool(b) => write!(f, "{:?}", b),
            Expr::Number(n) => write!(f, "{:?}", n),
            Expr::String(s) => write!(f, "{s:?}"),

            Expr::Variable(name) => write!(f, "{name}"),

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

            Expr::Dict(key_values) => {
                write!(f, "{{")?;
                let mut first = true;
                for (k, v) in key_values {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{k:?}: {v:?}")?;
                }
                write!(f, "}}")
            }

            Expr::Function(args, expr) => write!(f, "function{args:?} {expr:?}"),

            Expr::UnaryOp(op, expr) => write!(f, "{op:?}({expr:?})"),
            Expr::BinaryOp(op, lhs, rhs) => write!(f, "{op:?}({lhs:?}, {rhs:?})"),

            Expr::If(cond, then, else_) => write!(f, "if {cond:?} then {then:?} else {else_:?}"),
            Expr::Local(name, expr1, expr2) => write!(f, "local {name} = {expr1:?};\n{expr2:?}"),

            Expr::FunctionCall(func, args) => {
                write!(f, "{func:?}(")?;
                let mut first = true;
                for arg in args {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{arg:?}")?;
                }
                write!(f, ")")
            }

            Expr::FieldAccess(expr, name) => write!(f, "{expr:?}.{name}"),
            Expr::IndexAccess(expr, index) => write!(f, "{expr:?}[{index:?}]"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Not,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    NotEq,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeExpr {
    Simple(CompactString),
    Constructor(CompactString, Vec<TypeExpr>),
}

impl Display for TypeExpr {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            TypeExpr::Simple(name) => write!(f, "{name}"),
            TypeExpr::Constructor(name, params) => {
                write!(f, "{name}[")?;
                let mut first = true;
                for param in params {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    write!(f, "{param}")?;
                }
                write!(f, "]")
            }
        }
    }
}

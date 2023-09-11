use std::fmt::{self, Display, Formatter};

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Null,
    Bool,
    Number,
    String,
    Array(Box<Type>),
    Dict(Box<Type>),
    Function(Vec<Type>, Box<Type>),
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Type::Null => write!(f, "Null"),
            Type::Bool => write!(f, "Bool"),
            Type::Number => write!(f, "Number"),
            Type::String => write!(f, "String"),
            Type::Array(t) => write!(f, "Array[{t}]"),
            Type::Dict(t) => write!(f, "Dict[{t}]"),
            Type::Function(params, ret) => {
                write!(f, "(")?;
                let mut first = true;
                for t in params {
                    if first {
                        first = false;
                    } else {
                        write!(f, ", ")?;
                    }
                    t.fmt(f)?;
                }
                write!(f, ") => {ret}")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Erasure {
    Null,
    Bool,
    Number,
    String,
    Array,
    Dict,
    Function,
}

impl Display for Erasure {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

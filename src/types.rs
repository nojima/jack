use std::fmt::{self, Display, Formatter};

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

use compact_str::CompactString;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(Arc<String>),
    Array(im::Vector<Value>),
    Dict(im::HashMap<CompactString, Value>),
}

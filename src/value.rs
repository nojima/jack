use compact_str::CompactString;
use indexmap::IndexMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Dict(IndexMap<CompactString, Value>),
}

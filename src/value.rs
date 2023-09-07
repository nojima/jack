use std::cell::OnceCell;
use std::fmt::{self, Debug, Formatter};
use std::rc::Rc;

use compact_str::CompactString;
use serde::ser::{SerializeMap, SerializeSeq};

use crate::ast::Expr;
use crate::eval::{self, Env, EvalError};
use crate::symbol::Symbol;
use crate::types::Erasure;

#[derive(Debug, Clone, enum_assoc::Assoc)]
#[func(pub fn erasure(&self) -> Erasure)]
pub enum Value {
    #[assoc(erasure = Erasure::Null)]
    Null,

    #[assoc(erasure = Erasure::Bool)]
    Bool(bool),

    #[assoc(erasure = Erasure::Number)]
    Number(f64),

    #[assoc(erasure = Erasure::String)]
    String(Rc<String>),

    #[assoc(erasure = Erasure::Array)]
    Array(im_rc::Vector<Rc<Thunk>>),

    #[assoc(erasure = Erasure::Dict)]
    Dict(im_rc::HashMap<CompactString, Rc<Thunk>>),

    #[assoc(erasure = Erasure::Function)]
    Closure(Env, Vec<Symbol>, Rc<Expr>),
}

impl Value {
    pub fn try_eq(lhs: &Value, rhs: &Value) -> eval::Result<bool> {
        match (lhs, rhs) {
            (Value::Null, Value::Null) => Ok(true),
            (Value::Bool(b1), Value::Bool(b2)) => Ok(b1 == b2),
            (Value::Number(n1), Value::Number(n2)) => Ok(n1 == n2),
            (Value::String(s1), Value::String(s2)) => Ok(s1 == s2),
            (Value::Array(a1), Value::Array(a2)) => {
                if a1.len() != a2.len() {
                    return Ok(false);
                }
                for i in 0..a1.len() {
                    let a1v = a1.get(i).unwrap().force()?;
                    let a2v = a2.get(i).unwrap().force()?;
                    if !Value::try_eq(&a1v, &a2v)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            (Value::Dict(d1), Value::Dict(d2)) => {
                if d1.len() != d2.len() {
                    return Ok(false);
                }
                for (k, v1) in d1 {
                    let Some(v2) = d2.get(k) else {
                        return Ok(false);
                    };
                    let v1v = v1.force()?;
                    let v2v = v2.force()?;
                    if !Value::try_eq(&v1v, &v2v)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            (Value::Closure(_, _, _), _) => Err(EvalError::CannotCompare),
            (_, Value::Closure(_, _, _)) => Err(EvalError::CannotCompare),
            _ => Ok(false),
        }
    }
}

impl serde::Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;
        match self {
            Value::Null => serializer.serialize_none(),
            Value::Bool(b) => serializer.serialize_bool(*b),
            Value::Number(n) => serializer.serialize_f64(*n),
            Value::String(s) => serializer.serialize_str(&*s),
            Value::Array(array) => {
                let mut seq = serializer.serialize_seq(Some(array.len()))?;
                for thunk in array {
                    let value = thunk.force().map_err(|e| Error::custom(e.to_string()))?;
                    seq.serialize_element(&value)?;
                }
                seq.end()
            }
            Value::Dict(dict) => {
                let mut map = serializer.serialize_map(Some(dict.len()))?;
                let mut items: Vec<(_, _)> = dict.iter().collect();
                items.sort_unstable_by(|(k1, _), (k2, _)| k1.cmp(k2));
                for (key, thunk) in items {
                    let value = thunk.force().map_err(|e| Error::custom(e.to_string()))?;
                    map.serialize_entry(key, &value)?;
                }
                map.end()
            }
            Value::Closure(_, _, _) => Err(Error::custom("closure is not serializable")),
        }
    }
}

pub struct Thunk {
    env: OnceCell<Env>,
    expr: Box<Expr>,
    value: OnceCell<Value>,
}

impl Thunk {
    pub fn new(env: Env, expr: Box<Expr>) -> Self {
        Self {
            env: OnceCell::from(env),
            expr,
            value: OnceCell::new(),
        }
    }

    pub fn partial_new(expr: Box<Expr>) -> Self {
        Self {
            env: OnceCell::new(),
            expr,
            value: OnceCell::new(),
        }
    }

    pub fn set_env(&self, env: Env) {
        let _ = self.env.set(env);
    }

    pub fn force(&self) -> eval::Result<Value> {
        if let Some(v) = self.value.get() {
            return Ok(v.clone());
        }
        let Some(env) = self.env.get() else {
            panic!("env is not set")
        };
        let v = eval::eval_expr(env, &self.expr)?;
        let _ = self.value.set(v.clone());
        Ok(v)
    }
}

impl Debug for Thunk {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self.force())
    }
}

use crate::ast::Expr;
use crate::eval::{self, Env};
use crate::symbol::Symbol;
use compact_str::CompactString;
use std::cell::OnceCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(Rc<String>),
    Array(im_rc::Vector<Value>),
    Dict(im_rc::HashMap<CompactString, Value>),
    Closure(Env, Vec<Symbol>, Rc<Expr>),
}

impl Value {
    pub fn try_eq(lhs: &Value, rhs: &Value) -> Option<bool> {
        match (lhs, rhs) {
            (Value::Null, Value::Null) => Some(true),
            (Value::Bool(b1), Value::Bool(b2)) => Some(b1 == b2),
            (Value::Number(n1), Value::Number(n2)) => Some(n1 == n2),
            (Value::String(s1), Value::String(s2)) => Some(s1 == s2),
            (Value::Array(a1), Value::Array(a2)) => {
                if a1.len() != a2.len() {
                    return Some(false);
                }
                for i in 0..a1.len() {
                    match Value::try_eq(a1.get(i).unwrap(), a2.get(i).unwrap()) {
                        None => return None,
                        Some(false) => return Some(false),
                        _ => {}
                    }
                }
                Some(true)
            }
            (Value::Dict(d1), Value::Dict(d2)) => {
                if d1.len() != d2.len() {
                    return Some(false);
                }
                for (k, v1) in d1 {
                    let Some(v2) = d2.get(k) else {
                        return Some(false);
                    };
                    match Value::try_eq(v1, v2) {
                        None => return None,
                        Some(false) => return Some(false),
                        _ => {}
                    }
                }
                Some(true)
            }
            (Value::Closure(_, _, _), _) => None,
            (_, Value::Closure(_, _, _)) => None,
            _ => Some(false),
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

    pub fn unwrap(&self) -> eval::Result<Value> {
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.unwrap())
    }
}

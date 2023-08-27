use crate::ast::Expr;
use crate::eval::{self, Env, EvalError};
use crate::symbol::Symbol;
use compact_str::CompactString;
use std::cell::OnceCell;
use std::fmt::{Debug, Formatter, self};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(Rc<String>),
    Array(im_rc::Vector<Rc<Thunk>>),
    Dict(im_rc::HashMap<CompactString, Rc<Thunk>>),
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

use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::symbol::Symbol;
use crate::value::{Thunk, Value};
use compact_str::{CompactString, ToCompactString};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Clone, thiserror::Error)]
pub enum EvalError {
    #[error("bad operand type")]
    BadOperandType,

    #[error("condition of if-expression must be a bool")]
    ConditionMustBeBool,

    #[error("undefined variable: {0}")]
    UndefinedVariable(Symbol),

    #[error("field does not exit: {0}")]
    FieldDoesNotExist(Symbol),

    #[error("index out of bounds: {0}")]
    IndexOutOfBounds(usize),

    #[error("cannot compare")]
    CannotCompare,

    #[error("not callable")]
    NotCallable,

    #[error("wrong number of arguments")]
    WrongNumberOfArguments,
}

#[derive(Clone, Debug)]
pub struct Env {
    variables: im::HashMap<Symbol, Arc<Thunk>>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            variables: im::HashMap::new(),
        }
    }

    pub fn with_variable(&self, name: Symbol, thunk: Arc<Thunk>) -> Env {
        Self {
            variables: self.variables.update(name, thunk),
        }
    }

    pub fn lookup(&self, name: &Symbol) -> Option<Arc<Thunk>> {
        self.variables.get(name).cloned()
    }
}

pub type Result<T> = std::result::Result<T, EvalError>;

pub fn eval_expr(env: &Env, expr: &Expr) -> Result<Value> {
    match expr {
        Expr::Null => Ok(Value::Null),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(Arc::clone(s))),
        Expr::Array(array) => eval_array(env, array),
        Expr::Dict(key_values) => eval_dict(env, key_values),
        Expr::Function(args, expr) => eval_function_literal(env, args, expr),
        Expr::Variable(name) => eval_variable(env, name),
        Expr::UnaryOp(op, expr) => eval_unary_op(env, *op, expr),
        Expr::BinaryOp(op, lhs, rhs) => eval_binary_op(env, *op, lhs, rhs),
        Expr::If(cond, then, else_) => eval_if(env, cond, then, else_),
        Expr::Local(name, expr1, expr2) => eval_local(env, name, expr1, expr2),
        Expr::FunctionCall(func, args) => eval_function_call(env, func, args),
        Expr::FieldAccess(expr, name) => eval_field_access(env, expr, name),
        Expr::IndexAccess(expr, index) => eval_index_access(env, expr, index),
    }
}

fn eval_array(env: &Env, array: &[Expr]) -> Result<Value> {
    let mut values = Vec::new();
    for expr in array {
        let value = eval_expr(env, expr)?;
        values.push(value);
    }
    Ok(Value::Array(values.into()))
}

fn eval_dict(env: &Env, key_values: &[(CompactString, Expr)]) -> Result<Value> {
    let mut dict = HashMap::new();
    for (key, expr) in key_values {
        let value = eval_expr(env, expr)?;
        dict.insert(key.clone(), value);
    }
    Ok(Value::Dict(dict.into()))
}

fn eval_function_literal(env: &Env, args: &[Symbol], expr: &Expr) -> Result<Value> {
    Ok(Value::Closure(
        env.clone(),
        args.to_vec(),
        Arc::new(expr.clone()),
    ))
}

fn eval_variable(env: &Env, name: &Symbol) -> Result<Value> {
    match env.lookup(name) {
        Some(value) => Ok(value.unwrap()?),
        None => Err(EvalError::UndefinedVariable(name.clone())),
    }
}

fn eval_unary_op(env: &Env, op: UnaryOp, expr: &Expr) -> Result<Value> {
    match op {
        UnaryOp::Neg => {
            let value = eval_expr(env, expr)?;
            return match value {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => Err(EvalError::BadOperandType),
            };
        }
    }
}

fn eval_binary_op(env: &Env, op: BinaryOp, lhs: &Expr, rhs: &Expr) -> Result<Value> {
    match op {
        BinaryOp::Add => eval_add(env, lhs, rhs),
        BinaryOp::Sub => eval_sub(env, lhs, rhs),
        BinaryOp::Mul => eval_mul(env, lhs, rhs),
        BinaryOp::Div => eval_div(env, lhs, rhs),
        BinaryOp::Mod => eval_mod(env, lhs, rhs),
        BinaryOp::Eq => eval_eq(env, lhs, rhs),
        BinaryOp::NotEq => eval_not_eq(env, lhs, rhs),
    }
}

fn eval_add(env: &Env, lhs: &Expr, rhs: &Expr) -> Result<Value> {
    let l = eval_expr(env, lhs)?;
    let r = eval_expr(env, rhs)?;
    match (l, r) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l + r)),
        (Value::String(l), Value::String(r)) => {
            let ret = (*l).clone() + &r;
            Ok(Value::String(Arc::new(ret)))
        }
        _ => Err(EvalError::BadOperandType),
    }
}

fn eval_sub(env: &Env, lhs: &Expr, rhs: &Expr) -> Result<Value> {
    let l = eval_expr(env, lhs)?;
    let r = eval_expr(env, rhs)?;
    match (l, r) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l - r)),
        _ => Err(EvalError::BadOperandType),
    }
}

fn eval_mul(env: &Env, lhs: &Expr, rhs: &Expr) -> Result<Value> {
    let l = eval_expr(env, lhs)?;
    let r = eval_expr(env, rhs)?;
    match (l, r) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l * r)),
        _ => Err(EvalError::BadOperandType),
    }
}

fn eval_div(env: &Env, lhs: &Expr, rhs: &Expr) -> Result<Value> {
    let l = eval_expr(env, lhs)?;
    let r = eval_expr(env, rhs)?;
    match (l, r) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l / r)),
        _ => Err(EvalError::BadOperandType),
    }
}

fn eval_mod(env: &Env, lhs: &Expr, rhs: &Expr) -> Result<Value> {
    let l = eval_expr(env, lhs)?;
    let r = eval_expr(env, rhs)?;
    match (l, r) {
        (Value::Number(l), Value::Number(r)) => Ok(Value::Number(l % r)),
        _ => Err(EvalError::BadOperandType),
    }
}

fn eval_eq(env: &Env, lhs: &Expr, rhs: &Expr) -> Result<Value> {
    let l = eval_expr(env, lhs)?;
    let r = eval_expr(env, rhs)?;
    let Some(ret) = Value::try_eq(&l, &r) else {
        return Err(EvalError::CannotCompare);
    };
    Ok(Value::Bool(ret))
}

fn eval_not_eq(env: &Env, lhs: &Expr, rhs: &Expr) -> Result<Value> {
    let l = eval_expr(env, lhs)?;
    let r = eval_expr(env, rhs)?;
    let Some(ret) = Value::try_eq(&l, &r) else {
        return Err(EvalError::CannotCompare);
    };
    Ok(Value::Bool(!ret))
}

fn eval_if(env: &Env, cond: &Expr, then: &Expr, else_: &Expr) -> Result<Value> {
    let Value::Bool(cond_value) = eval_expr(env, cond)? else {
        return Err(EvalError::ConditionMustBeBool);
    };
    if cond_value {
        eval_expr(env, then)
    } else {
        eval_expr(env, else_)
    }
}

fn eval_local(env: &Env, name: &Symbol, expr1: &Expr, expr2: &Expr) -> Result<Value> {
    let thunk = Arc::new(Thunk::partial_new(Box::new(expr1.clone())));
    let new_env = env.with_variable(name.clone(), thunk.clone());
    thunk.set_env(new_env.clone());
    eval_expr(&new_env, expr2)
}

fn eval_function_call(env: &Env, func: &Expr, args: &[Expr]) -> Result<Value> {
    let func_value = eval_expr(env, func)?;
    match func_value {
        Value::Closure(closure_env, params, expr) => {
            if args.len() != params.len() {
                return Err(EvalError::WrongNumberOfArguments);
            }
            let mut new_env = closure_env;
            for (param, arg) in params.iter().zip(args) {
                let thunk = Thunk::new(env.clone(), Box::new(arg.clone()));
                new_env = new_env.with_variable(param.clone(), Arc::new(thunk));
            }
            eval_expr(&new_env, &expr)
        }
        _ => Err(EvalError::NotCallable),
    }
}

fn eval_field_access(env: &Env, expr: &Expr, name: &Symbol) -> Result<Value> {
    let value1 = eval_expr(env, expr)?;
    match value1 {
        Value::Dict(dict) => match dict.get(name) {
            Some(value2) => Ok(value2.clone()),
            None => Err(EvalError::FieldDoesNotExist(name.clone())),
        },
        _ => Err(EvalError::BadOperandType),
    }
}

fn eval_index_access(env: &Env, expr: &Expr, index: &Expr) -> Result<Value> {
    let collection_value = eval_expr(env, expr)?;
    let index_value = eval_expr(env, index)?;
    match collection_value {
        Value::Array(array) => match index_value {
            Value::Number(i) => {
                let index = i as usize;
                match array.get(index) {
                    Some(ret) => Ok(ret.clone()),
                    None => Err(EvalError::IndexOutOfBounds(index)),
                }
            }
            _ => Err(EvalError::BadOperandType),
        },
        Value::String(str) => match index_value {
            Value::Number(i) => {
                let index = i as usize;
                match str.chars().nth(index) {
                    Some(ret) => Ok(Value::String(Arc::new(String::from(ret)))),
                    None => Err(EvalError::IndexOutOfBounds(index)),
                }
            }
            _ => Err(EvalError::BadOperandType),
        },
        Value::Dict(dict) => match index_value {
            Value::String(s) => {
                let s = s.to_compact_string();
                match dict.get(&s) {
                    Some(ret) => Ok(ret.clone()),
                    None => Err(EvalError::FieldDoesNotExist(s)),
                }
            }
            _ => Err(EvalError::BadOperandType),
        },
        _ => Err(EvalError::BadOperandType),
    }
}

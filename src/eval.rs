use compact_str::CompactString;
use indexmap::IndexMap;
use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::symbol::Symbol;
use crate::value::Value;

#[derive(Debug, Clone, thiserror::Error)]
pub enum EvalError {
    #[error("bad operand type")]
    BadOperandType,

    #[error("condition of if-expression must be a bool")]
    ConditionMustBeBool,
}

#[derive(Clone)]
pub struct Env {
    variables: im::HashMap<Symbol, Value>,
}

impl Env {
    pub fn new() -> Self {
        Self {
            variables: im::HashMap::new(),
        }
    }
}

type Result<T> = std::result::Result<T, EvalError>;

pub fn eval_expr(env: &Env, expr: &Expr) -> Result<Value> {
    Ok(match expr {
        Expr::Null => Value::Null,
        Expr::Bool(b) => Value::Bool(*b),
        Expr::Number(n) => Value::Number(*n),
        Expr::String(s) => Value::String(s.clone()),
        Expr::Array(array) => eval_array(env, array)?,
        Expr::Dict(key_values) => eval_dict(env, key_values)?,
        Expr::UnaryOp(op, expr) => eval_unary_op(env, *op, expr)?,
        Expr::BinaryOp(op, lhs, rhs) => eval_binary_op(env, *op, lhs, rhs)?,
        Expr::If(cond, then, else_) => eval_if(env, cond, then, else_)?,
    })
}

fn eval_array(env: &Env, array: &[Expr]) -> Result<Value> {
    let mut values = Vec::new();
    for expr in array {
        let value = eval_expr(env, expr)?;
        values.push(value);
    }
    Ok(Value::Array(values))
}

fn eval_dict(env: &Env, key_values: &[(CompactString, Expr)]) -> Result<Value> {
    let mut dict = IndexMap::new();
    for (key, expr) in key_values {
        let value = eval_expr(env, expr)?;
        dict.insert(key.clone(), value);
    }
    Ok(Value::Dict(dict))
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
        (Value::String(l), Value::String(r)) => Ok(Value::String(l + &r)),
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
    Ok(Value::Bool(l == r))
}

fn eval_not_eq(env: &Env, lhs: &Expr, rhs: &Expr) -> Result<Value> {
    let l = eval_expr(env, lhs)?;
    let r = eval_expr(env, rhs)?;
    Ok(Value::Bool(l != r))
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

use crate::ast::{BinaryOp, Expr, UnaryOp};
use crate::symbol::Symbol;
use crate::value::Value;
use compact_str::CompactString;
use indexmap::IndexMap;

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

    pub fn with_variable(&self, name: Symbol, value: Value) -> Env {
        Self {
            variables: self.variables.update(name, value),
        }
    }

    pub fn lookup(&self, name: &Symbol) -> Option<Value> {
        self.variables.get(name).cloned()
    }
}

type Result<T> = std::result::Result<T, EvalError>;

pub fn eval_expr(env: &Env, expr: &Expr) -> Result<Value> {
    match expr {
        Expr::Null => Ok(Value::Null),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::Number(n) => Ok(Value::Number(*n)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Array(array) => eval_array(env, array),
        Expr::Dict(key_values) => eval_dict(env, key_values),
        Expr::Variable(name) => eval_variable(env, name),
        Expr::UnaryOp(op, expr) => eval_unary_op(env, *op, expr),
        Expr::BinaryOp(op, lhs, rhs) => eval_binary_op(env, *op, lhs, rhs),
        Expr::If(cond, then, else_) => eval_if(env, cond, then, else_),
        Expr::Local(name, expr1, expr2) => eval_local(env, name, expr1, expr2),
        Expr::FieldAccess(expr, name) => eval_field_access(env, expr, name),
    }
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

fn eval_variable(env: &Env, name: &Symbol) -> Result<Value> {
    match env.lookup(name) {
        Some(value) => Ok(value),
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

fn eval_local(env: &Env, name: &Symbol, expr1: &Expr, expr2: &Expr) -> Result<Value> {
    let value = eval_expr(env, expr1)?;
    let new_env = env.with_variable(name.clone(), value);
    eval_expr(&new_env, expr2)
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

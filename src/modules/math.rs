use serde_json::Value;
use crate::interpreter::context::Context;
use super::Module;
use crate::interpreter::error::{InterpreterError, Result};
use crate::interpreter::error::error_messages::math;
use crate::interpreter::variable_reference::VariableReference;
use std::panic;

pub struct MathModule;

impl MathModule {
    pub fn new() -> Self {
        MathModule
    }

    fn get_number(value: &Value, context: &Context) -> Result<f64> {
        match value {
            Value::Number(n) => n.as_f64().ok_or_else(|| 
                InterpreterError::RuntimeError(math::INVALID_NUMBER_CONVERSION.to_string())
            ),
            Value::String(s) => {
                if VariableReference::is_reference(s) {
                    if let Some(resolved) = context.get_value(s) {
                        match &resolved {
                            Value::Number(n) => n.as_f64().ok_or_else(|| 
                                InterpreterError::RuntimeError(math::INVALID_NUMBER_CONVERSION.to_string())
                            ),
                            Value::String(s) => s.parse().map_err(|_| 
                                InterpreterError::RuntimeError(format!("无法将字符串 '{}' 转换为数字", s))
                            ),
                            Value::Bool(b) => {
                                let bool_value = *b;
                                Ok(if bool_value { 1.0 } else { 0.0 })
                            },
                            Value::Null => Ok(0.0),
                            Value::Array(arr) => {
                                if arr.is_empty() {
                                    Ok(0.0)
                                } else {
                                    Err(InterpreterError::RuntimeError(
                                        format!("无法将非空数组 '{}' 转换为数字", resolved)
                                    ))
                                }
                            },
                            Value::Object(obj) => {
                                if obj.is_empty() {
                                    Ok(0.0)
                                } else {
                                    Err(InterpreterError::RuntimeError(
                                        format!("无法将非空对象 '{}' 转换为数字", resolved)
                                    ))
                                }
                            }
                        }
                    } else {
                        s.parse().map_err(|_| 
                            InterpreterError::RuntimeError(format!("无法将字符串 '{}' 转换为数字", s))
                        )
                    }
                } else {
                    s.parse().map_err(|_| 
                        InterpreterError::RuntimeError(format!("无法将字符串 '{}' 转换为数字", s))
                    )
                }
            },
            Value::Bool(b) => {
                let bool_value = *b;
                Ok(if bool_value { 1.0 } else { 0.0 })
            },
            Value::Null => Ok(0.0),
            Value::Array(arr) => {
                if arr.is_empty() {
                    Ok(0.0)
                } else {
                    Err(InterpreterError::RuntimeError(
                        format!("无法将非空数组 '{}' 转换为数字", value)
                    ))
                }
            },
            Value::Object(obj) => {
                if obj.is_empty() {
                    Ok(0.0)
                } else {
                    Err(InterpreterError::RuntimeError(
                        format!("无法将非空对象 '{}' 转换为数字", value)
                    ))
                }
            }
        }
    }

    fn add(args: &[Value], context: &mut Context) -> Value {
        let mut result = 0.0;
        for arg in args {
            match Self::get_number(arg, context) {
                Ok(num) => result += num,
                Err(err) => {
                    eprintln!("错误: {}", err);
                    return Value::Number(serde_json::Number::from_f64(0.0).unwrap());
                }
            }
        }
        Value::Number(serde_json::Number::from_f64(result).unwrap_or(serde_json::Number::from_f64(0.0).unwrap()))
    }

    fn subtract(args: &[Value], context: &mut Context) -> Value {
        match args.first().map(|v| Self::get_number(v, context)) {
            Some(Ok(first)) => {
                let result = args[1..].iter()
                    .filter_map(|v| Self::get_number(v, context).ok())
                    .fold(first, |acc, x| acc - x);
                Value::Number(serde_json::Number::from_f64(result).unwrap_or(serde_json::Number::from_f64(0.0).unwrap()))
            }
            Some(Err(err)) => {
                eprintln!("错误: {}", err);
                Value::Number(serde_json::Number::from_f64(0.0).unwrap())
            }
            None => Value::Number(serde_json::Number::from_f64(0.0).unwrap())
        }
    }

    fn multiply(args: &[Value], context: &mut Context) -> Value {
        let mut result = 1.0;
        for arg in args {
            match Self::get_number(arg, context) {
                Ok(num) => result *= num,
                Err(err) => {
                    eprintln!("错误: {}", err);
                    return Value::Number(serde_json::Number::from_f64(0.0).unwrap());
                }
            }
        }
        Value::Number(serde_json::Number::from_f64(result).unwrap_or(serde_json::Number::from_f64(0.0).unwrap()))
    }

    fn divide(args: &[Value], context: &mut Context) -> Value {
        if args.is_empty() {
            return Value::Number(serde_json::Number::from_f64(0.0).unwrap());
        }
        
        match Self::get_number(&args[0], context) {
            Ok(first) => {
                let mut result = first;
                
                // 尝试将所有后续参数转换为数字并进行除法运算
                for arg in &args[1..] {
                    match Self::get_number(arg, context) {
                        Ok(divisor) => {
                            if divisor == 0.0 {
                                // 除以零错误
                                panic!("{}", math::DIVISION_BY_ZERO);
                            }
                            result /= divisor;
                        },
                        Err(err) => {
                            eprintln!("错误: {}", err);
                            return Value::Number(serde_json::Number::from_f64(0.0).unwrap());
                        }
                    }
                }
                
                Value::Number(serde_json::Number::from_f64(result).unwrap_or(serde_json::Number::from_f64(0.0).unwrap()))
            },
            Err(err) => {
                eprintln!("错误: {}", err);
                Value::Number(serde_json::Number::from_f64(0.0).unwrap())
            }
        }
    }

    fn pow(args: &[Value], context: &mut Context) -> Value {
        match (args.get(0).map(|v| Self::get_number(v, context)), args.get(1).map(|v| Self::get_number(v, context))) {
            (Some(Ok(base)), Some(Ok(exp))) => {
                let result = base.powf(exp);
                Value::Number(serde_json::Number::from_f64(result).unwrap_or(serde_json::Number::from_f64(0.0).unwrap()))
            }
            (Some(Err(err)), _) | (_, Some(Err(err))) => {
                eprintln!("错误: {}", err);
                Value::Number(serde_json::Number::from_f64(0.0).unwrap())
            }
            _ => Value::Number(serde_json::Number::from_f64(0.0).unwrap())
        }
    }

    fn sqrt(args: &[Value], context: &mut Context) -> Value {
        match args.first().map(|v| Self::get_number(v, context)) {
            Some(Ok(num)) => {
                let result = num.sqrt();
                Value::Number(serde_json::Number::from_f64(result).unwrap_or(serde_json::Number::from_f64(0.0).unwrap()))
            }
            Some(Err(err)) => {
                eprintln!("错误: {}", err);
                Value::Number(serde_json::Number::from_f64(0.0).unwrap())
            }
            None => Value::Number(serde_json::Number::from_f64(0.0).unwrap())
        }
    }

    fn round(args: &[Value], context: &mut Context) -> Value {
        match args.first().map(|v| Self::get_number(v, context)) {
            Some(Ok(num)) => {
                let result = num.round();
                Value::Number(serde_json::Number::from_f64(result).unwrap_or(serde_json::Number::from_f64(0.0).unwrap()))
            }
            Some(Err(err)) => {
                eprintln!("错误: {}", err);
                Value::Number(serde_json::Number::from_f64(0.0).unwrap())
            }
            None => Value::Number(serde_json::Number::from_f64(0.0).unwrap())
        }
    }
}

impl Module for MathModule {
    fn get_name(&self) -> &'static str {
        "math"
    }

    fn get_functions(&self) -> Vec<(&'static str, fn(&[Value], &mut Context) -> Value)> {
        vec![
            ("add", Self::add),
            ("subtract", Self::subtract),
            ("multiply", Self::multiply),
            ("divide", Self::divide),
            ("pow", Self::pow),
            ("sqrt", Self::sqrt),
            ("round", Self::round)
        ]
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
} 
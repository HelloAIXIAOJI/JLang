use serde_json::Value;
use super::super::context::Context;
use super::super::error::{InterpreterError, Result};
use super::super::error::error_messages::statement;
use super::super::variable_reference::{VariableReference, ReferenceType};
use crate::is_debug_mode;

// 执行var语句 - 变量定义
pub fn execute_var_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(vars_obj) = args.as_object() {
        for (var_name, value) in vars_obj {
            let resolved_value = if let Some(text) = value.as_str() {
                if VariableReference::is_reference(text) {
                    // 使用VariableReference::parse和resolve_value来处理所有类型的变量引用
                    let var_ref = VariableReference::parse(text);
                    var_ref.resolve_value(&context.variables, &context.constants)
                } else {
                    Value::String(text.to_string())
                }
            } else {
                value.clone()
            };
            context.set_variable(var_name.clone(), resolved_value)?;
        }
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_obj("var")
        ))
    }
}

// 执行echo语句 - 输出内容
pub fn execute_echo_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(parts) = args.as_array() {
        for part in parts {
            // 使用可能抛出错误的版本处理变量引用
            let text = context.resolve_value_with_error(part)?;
            print!("{}", text);
        }
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("echo")
        ))
    }
}

// 执行concat语句 - 字符串拼接
pub fn execute_concat_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(obj) = args.as_object() {
        if let (Some(target), Some(parts)) = (obj.get("target"), obj.get("parts")) {
            if let Some(parts_array) = parts.as_array() {
                let mut result = String::new();
                for part in parts_array {
                    result.push_str(&context.resolve_value(part));
                }
                context.set_variable(
                    target.as_str().unwrap_or("result").to_string(),
                    Value::String(result)
                )?;
            }
        }
    }
    Ok(())
}

// 评估条件表达式
pub fn evaluate_condition(condition: &Value, context: &Context) -> bool {
    if let Some(obj) = condition.as_object() {
        if let (Some(op), Some(left), Some(right)) = (obj.get("op"), obj.get("left"), obj.get("right")) {
            let left_val = context.resolve_value(left);
            let right_val = context.resolve_value(right);
            
            // 获取原始值进行特殊类型比较
            let left_raw = if let Some(left_str) = left.as_str() {
                if VariableReference::is_reference(left_str) {
                    context.get_value(left_str)
                } else {
                    Some(left.clone())
                }
            } else {
                Some(left.clone())
            };
            
            let right_raw = if let Some(right_str) = right.as_str() {
                if VariableReference::is_reference(right_str) {
                    context.get_value(right_str)
                } else {
                    Some(right.clone())
                }
            } else {
                Some(right.clone())
            };
            
            // 布尔值与数字的特殊比较
            if let (Some(left_raw), Some(right_raw)) = (left_raw, right_raw) {
                // 布尔值和数字的比较
                if let (Value::Bool(left_bool), Value::Number(right_num)) = (&left_raw, &right_raw) {
                    let left_num = if *left_bool { 1.0 } else { 0.0 };
                    if let Some(right_num) = right_num.as_f64() {
                        match op.as_str().unwrap_or("") {
                            "eq" => return (left_num - right_num).abs() < std::f64::EPSILON,
                            "neq" => return (left_num - right_num).abs() >= std::f64::EPSILON,
                            "gt" => return left_num > right_num,
                            "lt" => return left_num < right_num,
                            "gte" => return left_num >= right_num,
                            "lte" => return left_num <= right_num,
                            _ => {}
                        }
                    }
                }
                
                // 数字和布尔值的比较
                if let (Value::Number(left_num), Value::Bool(right_bool)) = (&left_raw, &right_raw) {
                    if let Some(left_num) = left_num.as_f64() {
                        let right_num = if *right_bool { 1.0 } else { 0.0 };
                        match op.as_str().unwrap_or("") {
                            "eq" => return (left_num - right_num).abs() < std::f64::EPSILON,
                            "neq" => return (left_num - right_num).abs() >= std::f64::EPSILON,
                            "gt" => return left_num > right_num,
                            "lt" => return left_num < right_num,
                            "gte" => return left_num >= right_num,
                            "lte" => return left_num <= right_num,
                            _ => {}
                        }
                    }
                }
                
                // 字符串与数字比较
                if let (Value::String(left_str), Value::Number(right_num)) = (&left_raw, &right_raw) {
                    if let Ok(left_num) = left_str.parse::<f64>() {
                        if let Some(right_num) = right_num.as_f64() {
                            match op.as_str().unwrap_or("") {
                                "eq" => return (left_num - right_num).abs() < std::f64::EPSILON,
                                "neq" => return (left_num - right_num).abs() >= std::f64::EPSILON,
                                "gt" => return left_num > right_num,
                                "lt" => return left_num < right_num,
                                "gte" => return left_num >= right_num,
                                "lte" => return left_num <= right_num,
                                _ => {}
                            }
                        }
                    }
                }
                
                // 数字与字符串比较
                if let (Value::Number(left_num), Value::String(right_str)) = (&left_raw, &right_raw) {
                    if let Some(left_num) = left_num.as_f64() {
                        if let Ok(right_num) = right_str.parse::<f64>() {
                            match op.as_str().unwrap_or("") {
                                "eq" => return (left_num - right_num).abs() < std::f64::EPSILON,
                                "neq" => return (left_num - right_num).abs() >= std::f64::EPSILON,
                                "gt" => return left_num > right_num,
                                "lt" => return left_num < right_num,
                                "gte" => return left_num >= right_num,
                                "lte" => return left_num <= right_num,
                                _ => {}
                            }
                        }
                    }
                }
                
                // null特殊处理
                if let Value::Null = &left_raw {
                    if let Value::Number(right_num) = &right_raw {
                        if let Some(right_num) = right_num.as_f64() {
                            match op.as_str().unwrap_or("") {
                                "eq" => return right_num == 0.0,
                                "neq" => return right_num != 0.0,
                                "gt" => return 0.0 > right_num,
                                "lt" => return 0.0 < right_num,
                                "gte" => return 0.0 >= right_num,
                                "lte" => return 0.0 <= right_num,
                                _ => {}
                            }
                        }
                    }
                }
                
                if let Value::Null = &right_raw {
                    if let Value::Number(left_num) = &left_raw {
                        if let Some(left_num) = left_num.as_f64() {
                            match op.as_str().unwrap_or("") {
                                "eq" => return left_num == 0.0,
                                "neq" => return left_num != 0.0,
                                "gt" => return left_num > 0.0,
                                "lt" => return left_num < 0.0,
                                "gte" => return left_num >= 0.0,
                                "lte" => return left_num <= 0.0,
                                _ => {}
                            }
                        }
                    }
                }
            }
            
            // 尝试将两边都转换为数字进行比较
            let left_num = left_val.parse::<f64>();
            let right_num = right_val.parse::<f64>();
            
            match (left_num, right_num) {
                (Ok(ln), Ok(rn)) => {
                    // 两边都是数字，进行数值比较
                    match op.as_str().unwrap_or("") {
                        "eq" => (ln - rn).abs() < std::f64::EPSILON,
                        "neq" => (ln - rn).abs() >= std::f64::EPSILON,
                        "gt" => ln > rn,
                        "lt" => ln < rn,
                        "gte" => ln >= rn,
                        "lte" => ln <= rn,
                        _ => false
                    }
                },
                _ => {
                    // 字符串比较或其他类型
                    match op.as_str().unwrap_or("") {
                        "eq" => left_val == right_val,
                        "neq" => left_val != right_val,
                        "gt" => left_val > right_val,
                        "lt" => left_val < right_val,
                        "gte" => left_val >= right_val,
                        "lte" => left_val <= right_val,
                        _ => false
                    }
                }
            }
        } else {
            false
        }
    } else {
        false
    }
}

// 执行注释语句 - 不做任何操作，仅在调试模式下显示注释内容
pub fn execute_comment_statement(args: &Value, context: &mut Context) -> Result<()> {
    if is_debug_mode() {
        println!("执行注释: {:?}", args);
        
        if let Some(comment_text) = args.as_str() {
            println!("// 注释: {}", comment_text);
        } else if let Some(comment_array) = args.as_array() {
            print!("// 注释: ");
            for part in comment_array {
                // 解析变量引用
                print!("{}", context.resolve_value(part));
            }
            println!();
        }
    }
    Ok(())
} 
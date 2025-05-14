use serde_json::Value;
use super::super::context::Context;
use super::super::error::{InterpreterError, Result};
use super::super::error::error_messages::statement;
use super::super::variable_reference::{VariableReference, ReferenceType};
use crate::is_debug_mode;

// 执行var语句 - 变量定义
pub fn execute_var_statement(args: &Value, context: &mut Context) -> Result<Value> {
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
        Ok(Value::Null) // 变量定义没有实际返回值，返回Null
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_obj("var")
        ))
    }
}

// 执行echo语句 - 输出内容
pub fn execute_echo_statement(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(parts) = args.as_array() {
        for part in parts {
            // 预处理步骤：首先尝试执行part中的函数调用
            let processed_part = process_expression(part, context)?;
            
            // 根据是否启用详细显示决定输出格式
            if crate::is_show_values() {
                match &processed_part {
                    Value::String(s) if VariableReference::is_reference(s) => {
                        // 如果是变量引用，先获取值，再递归处理可能的表达式
                        if let Some(val) = context.get_value(s) {
                            // 递归处理变量值中可能包含的表达式
                            let processed_val = process_expression(&val, context)?;
                            match &processed_val {
                                Value::Object(_) | Value::Array(_) => {
                                    // 对对象和数组使用格式化JSON
                                    if let Ok(json) = serde_json::to_string_pretty(&processed_val) {
                                        print!("{}", json);
                                    } else {
                                        print!("{}", processed_val);
                                    }
                                },
                                // 简单值直接打印，不用引号
                                Value::String(str_val) => print!("{}", str_val),
                                Value::Number(num) => print!("{}", num),
                                Value::Bool(b) => print!("{}", b),
                                Value::Null => print!("null"),
                                _ => print!("{}", processed_val),
                            }
                        } else {
                            print!("{}", s);
                        }
                    },
                    Value::Object(_) | Value::Array(_) => {
                        // 对对象和数组使用格式化JSON输出，不包含额外引号
                        if let Ok(json) = serde_json::to_string_pretty(&processed_part) {
                            print!("{}", json);
                        } else {
                            print!("{:?}", processed_part);
                        }
                    },
                    // 简单值直接输出，不用引号
                    Value::String(s) => print!("{}", s),
                    Value::Number(n) => print!("{}", n),
                    Value::Bool(b) => print!("{}", b),
                    Value::Null => print!("null"),
                    _ => print!("{}", processed_part),
                }
            } else {
                // 普通模式下提取纯文本值
                match &processed_part {
                    Value::String(s) => print!("{}", s),
                    Value::Number(n) => print!("{}", n),
                    Value::Bool(b) => print!("{}", b),
                    Value::Null => print!("null"),
                    Value::Object(_) => print!("<object>"),
                    Value::Array(_) => print!("<array>"),
                    _ => print!("{}", processed_part),
                }
            }
        }
        Ok(Value::Null) // echo语句没有实际返回值，返回Null
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("echo")
        ))
    }
}

// 递归处理表达式，执行函数调用并返回结果
fn process_expression(expr: &Value, context: &mut Context) -> Result<Value> {
    match expr {
        Value::Object(obj) => {
            // 检查是否是函数调用（单键对象）
            if obj.len() == 1 {
                let (func_name, func_args) = obj.iter().next().unwrap();
                
                if super::is_builtin_statement(func_name) || func_name.contains('.') {
                    // 处理函数参数
                    let processed_args = match func_args {
                        Value::Array(arr) => {
                            let mut new_arr = Vec::new();
                            for item in arr {
                                new_arr.push(process_expression(item, context)?);
                            }
                            Value::Array(new_arr)
                        },
                        Value::Object(obj_args) => {
                            let mut new_map = serde_json::Map::new();
                            for (key, val) in obj_args {
                                new_map.insert(key.clone(), process_expression(val, context)?);
                            }
                            Value::Object(new_map)
                        },
                        _ => func_args.clone()
                    };
                    
                    // 执行函数调用
                    return super::execute_statement(func_name, &processed_args, context);
                }
            }
            
            // 不是函数调用，处理每个字段
            let mut new_obj = serde_json::Map::new();
            for (key, val) in obj {
                new_obj.insert(key.clone(), process_expression(val, context)?);
            }
            Ok(Value::Object(new_obj))
        },
        Value::Array(arr) => {
            // 处理数组中的每个元素
            let mut new_arr = Vec::new();
            for item in arr {
                new_arr.push(process_expression(item, context)?);
            }
            Ok(Value::Array(new_arr))
        },
        Value::String(s) => {
            // 处理变量引用
            if VariableReference::is_reference(s) {
                if let Some(val) = context.get_value(s) {
                    // 如果变量引用指向的是一个对象或数组，需要进一步处理其中的表达式
                    match &val {
                        Value::Object(obj) => {
                            // 检查是否是函数表达式对象
                            if obj.len() == 1 {
                                let (func_name, func_args) = obj.iter().next().unwrap();
                                
                                if super::is_builtin_statement(func_name) || func_name.contains('.') {
                                    // 处理并执行嵌套的函数调用
                                    return process_expression(&val, context);
                                }
                            }
                            // 处理一般对象
                            let mut new_obj = serde_json::Map::new();
                            for (key, val) in obj {
                                new_obj.insert(key.clone(), process_expression(val, context)?);
                            }
                            return Ok(Value::Object(new_obj));
                        },
                        Value::Array(arr) => {
                            // 处理数组中可能包含的表达式
                            let mut new_arr = Vec::new();
                            for item in arr {
                                new_arr.push(process_expression(item, context)?);
                            }
                            return Ok(Value::Array(new_arr));
                        },
                        _ => return Ok(val.clone())
                    }
                }
            }
            Ok(expr.clone())
        },
        _ => Ok(expr.clone())
    }
}

// 执行concat语句 - 字符串拼接
pub fn execute_concat_statement(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(obj) = args.as_object() {
        if let (Some(target), Some(parts)) = (obj.get("target"), obj.get("parts")) {
            if let Some(parts_array) = parts.as_array() {
                let mut result = String::new();
                for part in parts_array {
                    result.push_str(&context.resolve_value(part));
                }
                
                let result_value = Value::String(result.clone());
                
                // 设置目标变量
                context.set_variable(
                    target.as_str().unwrap_or("result").to_string(),
                    result_value.clone()
                )?;
                
                // 返回拼接结果
                return Ok(result_value);
            }
        }
    }
    Ok(Value::Null) // 如果参数格式不正确，返回Null
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
pub fn execute_comment_statement(args: &Value, context: &mut Context) -> Result<Value> {
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
    Ok(Value::Null) // 注释没有返回值，返回Null
} 
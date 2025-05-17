use serde_json::Value;
use super::super::context::Context;
use super::super::error::{InterpreterError, Result};
use super::super::error::error_messages::statement;
use super::super::variable_reference::{VariableReference, ReferenceType};
use crate::is_debug_mode;
use crate::is_print_full_values;
use super::store_result_with_compatibility;

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
            } else if let Some(obj) = value.as_object() {
                // 检查是否是嵌套函数调用（单键对象）
                if obj.len() == 1 {
                    let (func_type, func_args) = obj.iter().next().unwrap();
                    
                    // 处理嵌套函数调用
                    if func_type.contains('.') {
                        // 可能是模块函数调用
                        let parts: Vec<&str> = func_type.split('.').collect();
                        if parts.len() == 2 {
                            let module_name = parts[0];
                            let function_name = parts[1];
                            
                            if is_debug_mode() {
                                println!("检测到嵌套模块函数调用: {}.{}", module_name, function_name);
                            }
                            
                            // 解析函数参数
                            let args_list = match func_args {
                                Value::Array(arr) => arr.clone(),
                                _ => vec![func_args.clone()],
                            };
                            
                            // 直接调用模块函数并获取结果
                            match context.call_module_function(module_name, function_name, &args_list) {
                                Ok(result) => {
                                    if is_debug_mode() {
                                        println!("嵌套模块函数调用成功: {} => {:?}", func_type, result);
                                    }
                                    result
                                },
                                Err(e) => {
                                    if is_debug_mode() {
                                        println!("嵌套模块函数调用失败: {}", e);
                                    }
                                    // 如果调用失败，作为普通对象处理
                                    value.clone()
                                }
                            }
                        } else {
                            // 点号格式不正确，作为普通对象处理
                            value.clone()
                        }
                    } else if super::is_builtin_statement(func_type) {
                        // 内置语句
                        if is_debug_mode() {
                            println!("检测到嵌套内置语句: {}", func_type);
                        }
                        
                        // 执行嵌套的内置语句并获取结果
                        match super::execute_statement(func_type, func_args, context, Some(value)) {
                            Ok(result) => {
                                if is_debug_mode() {
                                    println!("嵌套内置语句成功执行: {} => {:?}", func_type, result);
                                }
                                result
                            },
                            Err(e) => {
                                // 如果执行失败，作为普通对象处理
                                if is_debug_mode() {
                                    println!("嵌套内置语句执行失败: {}", e);
                                }
                                value.clone()
                            }
                        }
                    } else {
                        // 可能是自定义函数调用或者普通对象
                        // 尝试在program中查找函数定义
                        if let Some(program_obj) = context.program.get("program") {
                            if program_obj.get(func_type).is_some() {
                                if is_debug_mode() {
                                    println!("检测到嵌套自定义函数调用: {}", func_type);
                                }
                                
                                // 执行嵌套的自定义函数调用并获取结果
                                match super::execute_statement(func_type, func_args, context, Some(value)) {
                                    Ok(result) => {
                                        if is_debug_mode() {
                                            println!("嵌套自定义函数调用成功: {} => {:?}", func_type, result);
                                        }
                                        result
                                    },
                                    Err(e) => {
                                        // 如果执行失败，作为普通对象处理
                                        if is_debug_mode() {
                                            println!("嵌套自定义函数调用执行失败: {}", e);
                                        }
                                        value.clone()
                                    }
                                }
                            } else {
                                // 不是函数调用，普通对象
                                value.clone()
                            }
                        } else {
                            // 没有program定义，普通对象
                            value.clone()
                        }
                    }
                } else {
                    // 多键对象，不是函数调用
                    value.clone()
                }
            } else {
                value.clone()
            };
            context.set_variable(var_name.clone(), resolved_value)?;
        }
        // 返回定义的最后一个变量的值，如果没有则返回null
        let result = if let Some((_, last_value)) = vars_obj.iter().last() {
            if let Some(text) = last_value.as_str() {
                if VariableReference::is_reference(text) {
                    let var_ref = VariableReference::parse(text);
                    var_ref.resolve_value(&context.variables, &context.constants)
                } else {
                    Value::String(text.to_string())
                }
            } else if let Some(obj) = last_value.as_object() {
                // 使用与上面相同的逻辑检查最后一个值是否是嵌套函数调用
                if obj.len() == 1 {
                    let (func_type, func_args) = obj.iter().next().unwrap();
                    
                    // 处理嵌套函数调用
                    if func_type.contains('.') {
                        // 可能是模块函数调用
                        let parts: Vec<&str> = func_type.split('.').collect();
                        if parts.len() == 2 {
                            let module_name = parts[0];
                            let function_name = parts[1];
                            
                            // 解析函数参数
                            let args_list = match func_args {
                                Value::Array(arr) => arr.clone(),
                                _ => vec![func_args.clone()],
                            };
                            
                            // 直接调用模块函数并获取结果
                            match context.call_module_function(module_name, function_name, &args_list) {
                                Ok(result) => result,
                                Err(_) => last_value.clone()
                            }
                        } else {
                            last_value.clone()
                        }
                    } else if super::is_builtin_statement(func_type) {
                        // 执行内置语句并获取结果
                        match super::execute_statement(func_type, func_args, context, Some(last_value)) {
                            Ok(result) => result,
                            Err(_) => last_value.clone()
                        }
                    } else {
                        // 尝试在program中查找函数定义
                        if let Some(program_obj) = context.program.get("program") {
                            if program_obj.get(func_type).is_some() {
                                // 执行自定义函数并获取结果
                                match super::execute_statement(func_type, func_args, context, Some(last_value)) {
                                    Ok(result) => result,
                                    Err(_) => last_value.clone()
                                }
                            } else {
                                last_value.clone()
                            }
                        } else {
                            last_value.clone()
                        }
                    }
                } else {
                    last_value.clone()
                }
            } else {
                last_value.clone()
            }
        } else {
            Value::Null
        };
        
        // 存储结果并返回
        store_result_with_compatibility(args, &result, context)?;
        Ok(result)
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_obj("var")
        ))
    }
}

// 执行echo语句 - 输出内容
pub fn execute_echo_statement(args: &Value, context: &mut Context) -> Result<Value> {
    // 处理数组格式
    if let Some(parts) = args.as_array() {
        let mut output = String::new();
        for part in parts {
            // 使用可能抛出错误的版本处理变量引用
            let text = if let Some(s) = part.as_str() {
                if s.contains("@var.") || s.contains("@param.") {
                    // 处理包含变量引用的字符串
                    let mut result = String::new();
                    let mut start = 0;
                    
                    // 查找并替换字符串中的所有变量引用
                    while let Some(ref_start) = s[start..].find('@') {
                        // 添加变量引用之前的文本
                        result.push_str(&s[start..start + ref_start]);
                        
                        // 确定变量引用的起始位置
                        let abs_ref_start = start + ref_start;
                        
                        // 查找变量引用的结束位置
                        let mut ref_end = abs_ref_start;
                        let mut dot_found = false;
                        
                        // 扫描完整的变量引用（@var.xxx或@param.xxx格式）
                        for (i, c) in s[abs_ref_start..].char_indices() {
                            let current_pos = abs_ref_start + i;
                            
                            // 如果还没找到点，检查前缀部分（@var或@param）
                            if !dot_found {
                                if c == '.' {
                                    dot_found = true;
                                    ref_end = current_pos + 1; // 包含点
                                    continue;
                                }
                                
                                if current_pos - abs_ref_start < 5 && (c == '@' || c.is_alphabetic()) {
                                    ref_end = current_pos + 1;
                                    continue;
                                }
                                
                                // 无效的变量引用前缀
                                break;
                            } else {
                                // 点之后是变量名部分，可以包含字母、数字、下划线
                                if c.is_alphanumeric() || c == '_' {
                                    ref_end = current_pos + 1;
                                    continue;
                                }
                                
                                // 遇到非法字符，变量名结束
                                break;
                            }
                        }
                        
                        // 提取完整的变量引用
                        let var_ref = &s[abs_ref_start..ref_end];
                        
                        if crate::is_debug_mode() {
                            println!("处理嵌入式变量引用: {}", var_ref);
                        }
                        
                        // 如果变量引用合法，解析并添加其值
                        if VariableReference::is_reference(var_ref) {
                            if crate::is_debug_mode() {
                                println!("有效的变量引用: {}", var_ref);
                            }
                            
                            let ref_obj = VariableReference::parse(var_ref);
                            
                            if crate::is_debug_mode() {
                                println!("解析为: {:?}, 名称: {}", ref_obj.ref_type, ref_obj.name);
                            }
                            
                            // 直接获取变量值
                            if let Some(value) = context.get_value(var_ref) {
                                if crate::is_debug_mode() {
                                    println!("变量值: {:?}", value);
                                }
                                
                                // 根据值类型进行格式化
                                match value {
                                    Value::String(s) => result.push_str(&s),
                                    Value::Number(n) => result.push_str(&n.to_string()),
                                    Value::Bool(b) => result.push_str(&b.to_string()),
                                    Value::Null => result.push_str("null"),
                                    Value::Array(arr) => {
                                        if crate::is_print_full_values() {
                                            // 完整显示数组内容
                                            let mut arr_str = String::new();
                                            arr_str.push('[');
                                            for (i, item) in arr.iter().enumerate() {
                                                if i > 0 {
                                                    arr_str.push_str(", ");
                                                }
                                                arr_str.push_str(&context.resolve_value(item));
                                            }
                                            arr_str.push(']');
                                            result.push_str(&arr_str);
                                        } else {
                                            result.push_str("<array>");
                                        }
                                    },
                                    Value::Object(obj) => {
                                        if crate::is_print_full_values() {
                                            // 完整显示对象内容
                                            let mut obj_str = String::new();
                                            obj_str.push('{');
                                            for (i, (key, val)) in obj.iter().enumerate() {
                                                if i > 0 {
                                                    obj_str.push_str(", ");
                                                }
                                                obj_str.push_str(&format!("\"{}\": {}", key, context.resolve_value(val)));
                                            }
                                            obj_str.push('}');
                                            result.push_str(&obj_str);
                                        } else {
                                            result.push_str("<object>");
                                        }
                                    },
                                }
                            } else {
                                // 变量不存在，保留原始文本
                                if crate::is_debug_mode() {
                                    println!("变量未找到，保留原始引用");
                                }
                                result.push_str(var_ref);
                            }
                        } else {
                            // 不是合法的变量引用，保留原始文本
                            result.push_str(var_ref);
                        }
                        
                        // 更新起始位置
                        start = ref_end;
                    }
                    
                    // 添加剩余文本
                    if start < s.len() {
                        result.push_str(&s[start..]);
                    }
                    
                    result
                } else {
                    // 不包含变量引用的普通字符串
                    s.to_string()
                }
            } else {
                // 非字符串类型，使用正常的解析方法
                context.resolve_value_with_error(part)?
            };
            
            output.push_str(&text);
            print!("{}", text);
        }
        // 返回输出的完整字符串
        let result = Value::String(output);
        
        // 始终将结果存储到result变量（与concat语句数组格式行为一致）
        context.set_variable("result".to_string(), result.clone())?;
        
        return Ok(result);
    }
    
    // 处理对象格式（包含output参数）
        if let Some(obj) = args.as_object() {
        let mut parts = Vec::new();
        let mut i = 0;
        
        // 收集所有数字索引参数，按顺序放入parts数组
        while let Some(part) = obj.get(&i.to_string()) {
            parts.push(part.clone());
            i += 1;
        }
        
        if parts.is_empty() {
            return Err(InterpreterError::RuntimeError(
                statement::param_must_be_array("echo")
            ));
        }
        
        // 处理parts数组
        let mut output = String::new();
        for part in parts {
            let text = if let Some(s) = part.as_str() {
                if s.contains("@var.") || s.contains("@param.") {
                    // 处理包含变量引用的字符串，使用与上面相同的逻辑
                    let mut result = String::new();
                    let mut start = 0;
                    
                    while let Some(ref_start) = s[start..].find('@') {
                        result.push_str(&s[start..start + ref_start]);
                        
                        let abs_ref_start = start + ref_start;
                        let mut ref_end = abs_ref_start;
                        let mut dot_found = false;
                        
                        // 扫描完整的变量引用（使用相同的逻辑）
                        for (i, c) in s[abs_ref_start..].char_indices() {
                            let current_pos = abs_ref_start + i;
                            
                            if !dot_found {
                                if c == '.' {
                                    dot_found = true;
                                    ref_end = current_pos + 1;
                                    continue;
                                }
                                
                                if current_pos - abs_ref_start < 5 && (c == '@' || c.is_alphabetic()) {
                                    ref_end = current_pos + 1;
                                    continue;
                                }
                                
                                break;
                            } else {
                                if c.is_alphanumeric() || c == '_' {
                                    ref_end = current_pos + 1;
                                    continue;
                                }
                                
                                break;
                            }
                        }
                        
                        let var_ref = &s[abs_ref_start..ref_end];
                        
                        if VariableReference::is_reference(var_ref) {
                            if let Some(value) = context.get_value(var_ref) {
                                match value {
                                    Value::String(s) => result.push_str(&s),
                                    Value::Number(n) => result.push_str(&n.to_string()),
                                    Value::Bool(b) => result.push_str(&b.to_string()),
                                    Value::Null => result.push_str("null"),
                                    Value::Array(arr) => {
                                        if crate::is_print_full_values() {
                                            // 完整显示数组内容
                                            let mut arr_str = String::new();
                                            arr_str.push('[');
                                            for (i, item) in arr.iter().enumerate() {
                                                if i > 0 {
                                                    arr_str.push_str(", ");
                                                }
                                                arr_str.push_str(&context.resolve_value(item));
            }
                                            arr_str.push(']');
                                            result.push_str(&arr_str);
                                        } else {
                                            result.push_str("<array>");
                                        }
                                    },
                                    Value::Object(obj) => {
                                        if crate::is_print_full_values() {
                                            // 完整显示对象内容
                                            let mut obj_str = String::new();
                                            obj_str.push('{');
                                            for (i, (key, val)) in obj.iter().enumerate() {
                                                if i > 0 {
                                                    obj_str.push_str(", ");
                                                }
                                                obj_str.push_str(&format!("\"{}\": {}", key, context.resolve_value(val)));
                                            }
                                            obj_str.push('}');
                                            result.push_str(&obj_str);
                                        } else {
                                            result.push_str("<object>");
                                        }
                                    },
                                }
                            } else {
                                result.push_str(var_ref);
                            }
                        } else {
                            result.push_str(var_ref);
                        }
                        
                        start = ref_end;
                    }
                    
                    if start < s.len() {
                        result.push_str(&s[start..]);
                    }
                    
                    result
                } else {
                    s.to_string()
                }
            } else {
                context.resolve_value_with_error(&part)?
            };
            
            output.push_str(&text);
            print!("{}", text);
        }
        
        // 返回输出的完整字符串
        let result = Value::String(output);
        
        // 始终将结果存储到result变量
        context.set_variable("result".to_string(), result.clone())?;
        
        // 如果指定了output参数，则额外存储到该变量
        if let Some(output_var) = obj.get("output").and_then(|v| v.as_str()) {
            if output_var != "result" {  // 避免重复设置result
                context.set_variable(output_var.to_string(), result.clone())?;
            }
        }
        
        return Ok(result);
    }
    
    // 既不是数组也不是对象，返回错误
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("echo")
        ))
}

// 执行concat语句 - 字符串拼接
pub fn execute_concat_statement(args: &Value, context: &mut Context) -> Result<Value> {
    // 处理数组格式 (新的简化格式)
    if let Some(parts_array) = args.as_array() {
        let mut result_str = String::new();
        for part in parts_array {
            let resolved_value = context.resolve_value_with_error(part)?;
            result_str.push_str(&resolved_value);
        }
        let result = Value::String(result_str);
        
        // 存储结果到默认的result变量
        context.set_variable("result".to_string(), result.clone())?;
        
        return Ok(result);
    }
    
    // 处理对象格式 (原有格式)
    if let Some(obj) = args.as_object() {
        if let (Some(target), Some(parts)) = (obj.get("target"), obj.get("parts")) {
            if let Some(parts_array) = parts.as_array() {
                let mut result_str = String::new();
                for part in parts_array {
                    result_str.push_str(&context.resolve_value(part));
                }
                let result = Value::String(result_str);
                
                // 存储到指定目标变量
                context.set_variable(
                    target.as_str().unwrap_or("result").to_string(),
                    result.clone()
                )?;
                
                // 只有明确指定output参数时才额外存储结果
                if obj.get("output").is_some() {
                    store_result_with_compatibility(args, &result, context)?;
                }
                return Ok(result);
            }
        }
    }
    // 如果参数无效，返回空字符串
    let result = Value::String(String::new());
    
    // 只有明确指定output参数时才存储结果
    if let Some(obj) = args.as_object() {
        if obj.get("output").is_some() {
            store_result_with_compatibility(args, &result, context)?;
        }
    }
    Ok(result)
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
            
            if let (Ok(ln), Ok(rn)) = (left_num, right_num) {
                match op.as_str().unwrap_or("") {
                    "eq" => return (ln - rn).abs() < std::f64::EPSILON,
                    "neq" => return (ln - rn).abs() >= std::f64::EPSILON,
                    "gt" => return ln > rn,
                    "lt" => return ln < rn,
                    "gte" => return ln >= rn,
                    "lte" => return ln <= rn,
                    _ => {}
                }
            }
            
            // 当数字比较不可行时，使用字符串比较
            match op.as_str().unwrap_or("") {
                "eq" => return left_val == right_val,
                "neq" => return left_val != right_val,
                "gt" => return left_val > right_val,
                "lt" => return left_val < right_val,
                "gte" => return left_val >= right_val,
                "lte" => return left_val <= right_val,
                "and" => return left_val != "0" && left_val != "false" && left_val != "" && right_val != "0" && right_val != "false" && right_val != "",
                "or" => return left_val != "0" && left_val != "false" && left_val != "" || right_val != "0" && right_val != "false" && right_val != "",
                _ => return false
            }
        }
    }
    false
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
    // 注释语句返回null值
    let result = Value::Null;
    
    // 只有明确指定output参数时才存储结果
    if let Some(obj) = args.as_object() {
        if obj.get("output").is_some() {
            store_result_with_compatibility(args, &result, context)?;
        }
    }
    Ok(result)
}

// 执行return语句 - 从函数返回值
pub fn execute_return_statement(args: &Value, context: &mut Context) -> Result<Value> {
    // 解析return的值 - 可以是任何类型
    let return_value = if let Some(text) = args.as_str() {
        if VariableReference::is_reference(text) {
            // 变量引用
            let var_ref = VariableReference::parse(text);
            var_ref.resolve_value(&context.variables, &context.constants)
        } else {
            // 普通字符串
            Value::String(text.to_string())
        }
    } else if let Some(obj) = args.as_object() {
        // 检查是否是函数调用（单键对象）
        if obj.len() == 1 {
            let (func_type, func_args) = obj.iter().next().unwrap();
            
            // 模块函数调用
            if func_type.contains('.') {
                let parts: Vec<&str> = func_type.split('.').collect();
                if parts.len() == 2 {
                    let module_name = parts[0];
                    let function_name = parts[1];
                    
                    // 解析函数参数
                    let args_list = match func_args {
                        Value::Array(arr) => arr.clone(),
                        _ => vec![func_args.clone()],
                    };
                    
                    // 调用模块函数并返回结果
                    match context.call_module_function(module_name, function_name, &args_list) {
                        Ok(result) => result,
                        Err(e) => {
                            // 如果函数调用失败，作为普通对象处理
                            if is_debug_mode() {
                                println!("Return中的函数调用失败: {}", e);
                            }
                            args.clone()
                        }
                    }
                } else {
                    // 不是有效的模块函数调用
                    args.clone()
                }
            } else if super::is_builtin_statement(func_type) {
                // 内置语句
                match super::execute_statement(func_type, func_args, context, Some(args)) {
                    Ok(result) => result,
                    Err(_) => args.clone() // 如果执行失败，返回原始参数
                }
            } else if let Some(program_obj) = context.program.get("program") {
                // 自定义函数
                if program_obj.get(func_type).is_some() {
                    match super::execute_statement(func_type, func_args, context, Some(args)) {
                        Ok(result) => result,
                        Err(_) => args.clone()
                    }
                } else {
                    args.clone()
                }
            } else {
                args.clone()
            }
        } else {
            args.clone()
        }
    } else {
        // 值类型（数字、布尔、null、数组）直接返回
        args.clone()
    };
    
    // 设置函数的返回值状态
    context.set_return_value(Some(return_value.clone()));
    
    // 返回处理后的值
    Ok(return_value)
} 
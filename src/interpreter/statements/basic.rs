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

// 实现一个更健壮的变量引用提取函数
fn extract_variable_references(text: &str, context: &Context) -> String {
    if !text.contains('@') && !text.contains('$') && !text.contains('￥') {
        return text.to_string();
    }
    
    if crate::is_debug_mode() {
        println!("处理嵌入式变量引用文本: {}", text);
    }
    
    // 检查是否有嵌套变量引用如 @var.array[@var.index]
    if text.contains('[') && text.contains('@') && text.contains(']') {
        // 检查是否是数组索引中包含变量引用的情况
        let open_bracket_pos = text.find('[');
        let close_bracket_pos = text.find(']');
        
        if let (Some(open_pos), Some(close_pos)) = (open_bracket_pos, close_bracket_pos) {
            if open_pos < close_pos && 
               // 检查方括号内是否有变量引用符号
               text[open_pos..close_pos].contains('@') {
                
                // 尝试使用新的嵌套变量解析函数
                use super::super::variable_reference::resolve_nested_variable_reference;
                
                match resolve_nested_variable_reference(text, &context.variables, &context.constants) {
                    Ok(result) => return result,
                    Err(err) => {
                        if crate::is_debug_mode() {
                            println!("解析嵌套变量引用失败: {}", err);
                        }
                        // 如果解析失败，使用标准解析流程
                    }
                }
            }
        }
    }
    
    let mut result = String::new();
    let mut current_pos = 0;
    let text_chars: Vec<char> = text.chars().collect();
    
    while current_pos < text_chars.len() {
        // 查找变量引用标记
        let mut ref_start_pos = None;
        for i in current_pos..text_chars.len() {
            let c = text_chars[i];
            if c == '@' || c == '$' || c == '￥' {
                ref_start_pos = Some(i);
                                break;
                            }
        }
        
        if let Some(start_idx) = ref_start_pos {
            // 先添加变量引用前的文本
            for i in current_pos..start_idx {
                result.push(text_chars[i]);
                        }
                        
                        // 提取完整的变量引用
            let mut var_ref = String::new();
            let mut i = start_idx;
                        
            // 特殊处理前缀
            if text_chars[i] == '@' {
                var_ref.push('@');
                i += 1;
                
                // 检测变量类型前缀 (var/param/const/env)
                let mut prefix = String::new();
                while i < text_chars.len() && text_chars[i] != '.' {
                    prefix.push(text_chars[i]);
                    i += 1;
                }
                
                if !vec!["var", "param", "params", "const", "env"].contains(&prefix.as_str()) {
                    // 无效的前缀，不处理为变量引用
                    result.push_str(&var_ref);
                    result.push_str(&prefix);
                    current_pos = i;
                    continue;
                }
                
                var_ref.push_str(&prefix);
                
                // 添加点号分隔符
                if i < text_chars.len() && text_chars[i] == '.' {
                    var_ref.push('.');
                    i += 1;
                }
            } else {
                // 简单前缀 $ 或 ￥
                var_ref.push(text_chars[i]);
                i += 1;
            }
            
            // 收集变量名和任何嵌套访问(包括数组索引)
            let mut nesting_level = 0;
            while i < text_chars.len() {
                let c = text_chars[i];
                
                // 处理中文和非ASCII字符
                if c.is_alphanumeric() || c == '_' || c == '.' || !c.is_ascii() {
                    var_ref.push(c);
                    i += 1;
                }
                // 处理数组索引
                else if c == '[' {
                    var_ref.push(c);
                    i += 1;
                    nesting_level += 1;
                }
                else if c == ']' && nesting_level > 0 {
                    var_ref.push(c);
                    i += 1;
                    nesting_level -= 1;
                }
                else {
                    break;
                }
            }
            
            if crate::is_debug_mode() {
                println!("提取的变量引用: {}", var_ref);
                                        }
            
            // 解析并替换变量引用
            if super::super::variable_reference::VariableReference::is_reference(&var_ref) {
                if crate::is_debug_mode() {
                    println!("有效变量引用: {}", var_ref);
                }
                
                // 创建变量引用对象
                let var_ref_obj = super::super::variable_reference::VariableReference::parse(&var_ref);
                
                // 尝试解析变量值
                match var_ref_obj.resolve_value_with_error(&context.variables, &context.constants) {
                    Ok(value) => {
                                if crate::is_debug_mode() {
                            println!("变量解析成功: {:?}", value);
                        }
                        // 使用新的格式化方法
                        result.push_str(&context.format_value(&value));
                    },
                    Err(err) => {
                        if crate::is_debug_mode() {
                            println!("变量解析失败: {}", err);
                                }
                        // 保留原始文本
                        result.push_str(&var_ref);
                    }
                            }
                        } else {
                // 不是完整/有效的变量引用，保留原始文本
                result.push_str(&var_ref);
                        }
                        
            // 更新当前位置
            current_pos = i;
        } else {
            // 没有更多的变量引用，添加剩余文本
            for i in current_pos..text_chars.len() {
                result.push(text_chars[i]);
            }
            break;
        }
                    }
                    
                    result
}

// 执行echo语句 - 输出内容
pub fn execute_echo_statement(args: &Value, context: &mut Context) -> Result<Value> {
    // 处理数组格式
    if let Some(parts) = args.as_array() {
        let mut output = String::new();
        for part in parts {
            // 解析部分的值
            let text = if let Some(s) = part.as_str() {
                // 处理可能包含变量引用的字符串
                extract_variable_references(s, context)
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
        let mut output_var = None;
        let mut i = 0;
        
        // 收集所有数字索引参数，按顺序放入parts数组
        while let Some(part) = obj.get(&i.to_string()) {
            parts.push(part.clone());
            i += 1;
        }
        
        // 检查是否指定了输出变量
        if let Some(output) = obj.get("output") {
            if let Some(var_name) = output.as_str() {
                output_var = Some(var_name.to_string());
            }
        }
        
        if parts.is_empty() {
            return Err(InterpreterError::RuntimeError(
                statement::param_must_be_array("echo")
            ));
        }
        
        // 处理parts数组
        let mut output_text = String::new();
        for part in parts {
            let text = if let Some(s) = part.as_str() {
                // 处理可能包含变量引用的字符串
                extract_variable_references(s, context)
                            } else {
                // 非字符串类型，使用正常的解析方法
                context.resolve_value_with_error(&part)?
            };
            
            output_text.push_str(&text);
            print!("{}", text);
        }
        
        // 创建输出值
        let result = Value::String(output_text);
        
        // 如果指定了输出变量，设置它
        if let Some(var_name) = output_var {
            context.set_variable(var_name, result.clone())?;
        } else {
            // 否则存储到默认的result变量
        context.set_variable("result".to_string(), result.clone())?;
        }
        
        return Ok(result);
    }
    
    // 格式无效
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

// 执行get_property语句 - 动态属性访问
pub fn execute_get_property_statement(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(obj) = args.as_object() {
        // 获取对象引用
        let object_ref = obj.get("object").ok_or_else(|| {
            InterpreterError::RuntimeError(
                "get_property语句缺少'object'字段，需要指定要访问的对象".to_string()
            )
        })?;
        
        // 解析对象
        let mut current_value = context.resolve_value_raw(object_ref)?;
        
        // 获取路径数组
        let path = obj.get("path").ok_or_else(|| {
            InterpreterError::RuntimeError(
                "get_property语句缺少'path'字段，需要指定访问路径".to_string()
            )
        })?;
        
        // 路径可以是字符串或数组
        let path_elements = if let Some(path_str) = path.as_str() {
            // 如果是字符串，使用点分隔解析
            path_str.split('.').map(|s| Value::String(s.to_string())).collect::<Vec<_>>()
        } else if let Some(path_array) = path.as_array() {
            // 如果是数组，直接使用
            path_array.clone()
        } else {
            return Err(InterpreterError::RuntimeError(
                "get_property的'path'字段必须是字符串或数组".to_string()
            ));
        };
        
        if is_debug_mode() {
            println!("动态属性访问 - 基础对象: {:?}", current_value);
            println!("动态属性访问 - 路径: {:?}", path_elements);
        }
        
        // 遍历路径，逐层访问属性
        for path_element in path_elements {
            // 解析路径元素值
            let prop_name_value = context.resolve_value_raw(&path_element)?;
            
            if is_debug_mode() {
                println!("访问属性: {:?}", prop_name_value);
            }
            
            // 根据当前值的类型进行处理
            if current_value.is_array() {
                let arr = current_value.as_array().unwrap();
                
                // 特殊处理：数组的length属性
                if let Value::String(ref s) = prop_name_value {
                    if s == "length" {
                        current_value = Value::Number(serde_json::Number::from(arr.len()));
                        continue;
                    }
                }
                
                // 解析索引
                let index = match prop_name_value {
                    Value::Number(ref n) => {
                        if let Some(i) = n.as_u64() {
                            i as usize
                        } else {
                            return Err(InterpreterError::RuntimeError(
                                format!("无效的数组索引: {:?}", n)
                            ));
                        }
                    },
                    Value::String(ref s) => {
                        match s.parse::<usize>() {
                            Ok(i) => i,
                            Err(_) => return Err(InterpreterError::RuntimeError(
                                format!("无法将字符串 '{}' 解析为数组索引", s)
                            )),
                        }
                    },
                    _ => return Err(InterpreterError::RuntimeError(
                        format!("无效的数组索引类型: {:?}", prop_name_value)
                    )),
                };
                
                // 访问数组元素
                if index < arr.len() {
                    current_value = arr[index].clone();
                } else {
                    return Err(InterpreterError::RuntimeError(
                        format!("数组索引 {} 超出范围 (0-{})", index, arr.len() - 1)
                    ));
                }
            } 
            else if current_value.is_object() {
                let obj = current_value.as_object().unwrap();
                
                // 获取属性名
                let key = match prop_name_value {
                    Value::String(ref s) => s.clone(),
                    _ => prop_name_value.to_string(),
                };
                
                // 访问对象属性
                if let Some(val) = obj.get(&key) {
                    current_value = val.clone();
                } else {
                    return Err(InterpreterError::RuntimeError(
                        format!("对象中不存在属性 '{}'", key)
                    ));
                }
            }
            else {
                return Err(InterpreterError::RuntimeError(
                    format!("无法访问非对象或数组类型的属性: {:?}", current_value)
                ));
            }
        }
        
        // 将结果存储到output变量（如果指定）或result
        store_result_with_compatibility(args, &current_value, context)?;
        
        Ok(current_value)
    } else {
        Err(InterpreterError::RuntimeError(
            "get_property语句参数必须是一个对象".to_string()
        ))
    }
} 
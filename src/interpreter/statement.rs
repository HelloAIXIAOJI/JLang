use serde_json::Value;
use super::context::Context;
use super::error::{InterpreterError, Result};
use crate::modules::jl_module;
use crate::is_debug_mode;
use std::collections::HashMap;
use std::process::Command;

pub fn execute_statement(stmt_type: &str, args: &Value, context: &mut Context) -> Result<()> {
    if is_debug_mode() {
        println!("执行语句: {}", stmt_type);
    }
    
    match stmt_type {
        "comment" => execute_comment_statement(args, context),
        "var" => execute_var_statement(args, context),
        "echo" => execute_echo_statement(args, context),
        "concat" => execute_concat_statement(args, context),
        "if" => execute_if_statement(args, context),
        "call" => execute_call_statement(args, context),
        "while" => execute_while_statement(args, context),
        "for" => execute_for_statement(args, context),
        "exec" => execute_exec_statement(args, context),
        _ => {
            // 检查是否是新的函数调用语法（例如：math.add 或 自定义模块.函数）
            if stmt_type.contains('.') {
                let parts: Vec<&str> = stmt_type.split('.').collect();
                if parts.len() == 2 {
                    let module_name = parts[0];
                    let function_name = parts[1];
                    let args_array = if args.is_array() {
                        args.as_array().unwrap().to_vec()
                    } else {
                        vec![args.clone()]
                    };
                    
                    // 先检查是否是自定义.jl模块
                    if let Some(module) = context.modules.get(module_name) {
                        // 检查是否是 .jl 模块
                        if let Some(jl_module) = module.as_any().downcast_ref::<jl_module::JlModule>() {
                            if let Some(func_def) = jl_module.get_function(function_name) {
                                // 克隆函数定义，避免借用冲突
                                let func_def_clone = func_def.clone();
                                let mut params_map = serde_json::Map::new();
                                
                                // 获取函数参数定义
                                if let Some(params) = func_def.get("params") {
                                    if let Some(params_obj) = params.as_object() {
                                        for (i, (param_name, _)) in params_obj.iter().enumerate() {
                                            if let Some(arg) = args_array.get(i) {
                                                params_map.insert(param_name.clone(), arg.clone() as Value);
                                            } else {
                                                return Err(InterpreterError::FunctionError(
                                                    format!("杂鱼~缺少参数 '{}'", param_name)
                                                ));
                                            }
                                        }
                                    }
                                }

                                let params = Value::Object(params_map);
                                return execute_function(&func_def_clone, context, Some(&params));
                            }
                        }
                        
                        // 如果不是自定义模块或找不到函数，尝试调用标准模块函数
                        let result = context.call_module_function(module_name, function_name, &args_array)?;
                        context.set_variable("result".to_string(), result)?;
                        return Ok(());
                    } else {
                        // 模块不存在
                        return Err(InterpreterError::ModuleError(
                            format!("杂鱼~未找到模块 '{}'", module_name)
                        ));
                    }
                }
            }

            // 检查是否是自定义函数调用
            if let Some(program_obj) = context.program.get("program") {
                if let Some(func) = program_obj.get(stmt_type) {
                    // 检查是否是内置语句或模块函数
                    if is_builtin_statement(stmt_type) {
                        return Err(InterpreterError::FunctionError(
                            format!("杂鱼~函数名 '{}' 与内置语句冲突", stmt_type)
                        ));
                    }

                    // 检查模块函数冲突
                    if stmt_type.contains('.') {
                        let parts: Vec<&str> = stmt_type.split('.').collect();
                        if parts.len() == 2 {
                            let module_name = parts[0];
                            let function_name = parts[1];
                            if let Some(module) = context.modules.get(module_name) {
                                for (fname, _) in module.get_functions() {
                                    if fname == function_name {
                                        return Err(InterpreterError::FunctionError(
                                            format!("杂鱼~函数名 '{}' 与模块函数冲突", stmt_type)
                                        ));
                                    }
                                }
                            }
                        }
                    }

                    let func = func.clone();
                    let args_array = if args.is_array() {
                        args.as_array().unwrap().to_vec()
                    } else {
                        vec![args.clone()]
                    };

                    // 获取函数参数定义
                    let params_obj = if let Some(params) = func.get("params") {
                        if let Some(params_obj) = params.as_object() {
                            params_obj
                        } else {
                            return Err(InterpreterError::FunctionError(
                                "杂鱼~函数参数定义必须是对象".to_string()
                            ));
                        }
                    } else {
                        return Err(InterpreterError::FunctionError(
                            "杂鱼~函数缺少参数定义".to_string()
                        ));
                    };

                    // 创建参数对象
                    let mut params_map = serde_json::Map::new();
                    for (i, (param_name, _)) in params_obj.iter().enumerate() {
                        if let Some(arg) = args_array.get(i) {
                            params_map.insert(param_name.clone(), arg.clone() as Value);
                        } else {
                            return Err(InterpreterError::FunctionError(
                                format!("杂鱼~缺少参数 '{}'", param_name)
                            ));
                        }
                    }

                    let params = Value::Object(params_map);
                    return execute_function(&func, context, Some(&params));
                }
            }

            Err(InterpreterError::RuntimeError(
                format!("杂鱼~未知的语句类型: {}", stmt_type)
            ))
        }
    }
}

// 检查是否是内置语句
fn is_builtin_statement(name: &str) -> bool {
    matches!(name, "var" | "echo" | "concat" | "if" | "call" | "while" | "for" | "comment" | "exec")
}

fn execute_var_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(vars_obj) = args.as_object() {
        for (var_name, value) in vars_obj {
            let resolved_value = if let Some(text) = value.as_str() {
                if text.starts_with("@") {
                    if let Some(val) = context.get_value(text) {
                        val.clone()
                    } else {
                        Value::String(text.to_string())
                    }
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
            "杂鱼~'var' 语句的参数必须是一个对象".to_string()
        ))
    }
}

fn execute_echo_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        for arg in args_array {
            let text = context.resolve_value(arg);
            print!("{}", text);
        }
    } else if let Some(text) = args.as_str() {
        if text.starts_with("@") {
            if let Some(val) = context.get_value(text) {
                match val {
                    Value::String(s) => print!("{}", context.process_special_chars(s)),
                    Value::Number(n) => print!("{}", n),
                    Value::Bool(b) => print!("{}", b),
                    Value::Null => print!("null"),
                    Value::Array(_) => print!("{}", val),
                    Value::Object(_) => print!("{}", val),
                }
            } else {
                print!("{}", text);
            }
        } else {
            print!("{}", context.process_special_chars(text));
        }
    } else {
        print!("{}", context.resolve_value(args));
    }
    Ok(())
}

fn execute_concat_statement(args: &Value, context: &mut Context) -> Result<()> {
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

fn execute_if_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(obj) = args.as_object() {
        if let (Some(condition), Some(then_block)) = (obj.get("condition"), obj.get("then")) {
            let condition_result = evaluate_condition(condition, context);
            let block = if condition_result {
                then_block
            } else {
                obj.get("else").unwrap_or(then_block)
            };

            if let Some(statements) = block.as_array() {
                for stmt in statements {
                    if let Some(obj) = stmt.as_object() {
                        if let Some((stmt_type, args)) = obj.iter().next() {
                            execute_statement(stmt_type, args, context)?;
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

fn execute_call_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.is_empty() {
            return Err(InterpreterError::FunctionError(
                "杂鱼~函数调用缺少函数名".to_string()
            ));
        }

        if let Some(func_name) = args_array[0].as_str() {
            // 检查是否是模块函数调用
            if let Some(dot_pos) = func_name.find('.') {
                let (module_name, function_name) = func_name.split_at(dot_pos);
                let function_name = &function_name[1..]; // 去掉点号
                let args_array = args_array[1..].to_vec();

                // 构造一个新的语句类型，使用简化语法格式
                let new_stmt_type = format!("{}.{}", module_name, function_name);
                
                // 直接调用 execute_statement，复用简化语法的处理逻辑
                return execute_statement(&new_stmt_type, &Value::Array(args_array), context);
            }

            // 检查是否是自定义函数调用
            if let Some(program_obj) = context.program.get("program") {
                if let Some(func) = program_obj.get(func_name) {
                    let func = func.clone();
                    let params = args_array.get(1).cloned();
                    return execute_function(&func, context, params.as_ref());
                }
            }
        }
    }
    Err(InterpreterError::RuntimeError(
        "杂鱼~无效的函数调用".to_string()
    ))
}

fn evaluate_condition(condition: &Value, context: &Context) -> bool {
    if let Some(obj) = condition.as_object() {
        if let (Some(op), Some(left), Some(right)) = (obj.get("op"), obj.get("left"), obj.get("right")) {
            let left_val = context.resolve_value(left);
            let right_val = context.resolve_value(right);
            
            // 获取原始值进行特殊类型比较
            let left_raw = if let Some(left_str) = left.as_str() {
                if left_str.starts_with("@") {
                    context.get_value(left_str)
                } else {
                    Some(left)
                }
            } else {
                Some(left)
            };
            
            let right_raw = if let Some(right_str) = right.as_str() {
                if right_str.starts_with("@") {
                    context.get_value(right_str)
                } else {
                    Some(right)
                }
            } else {
                Some(right)
            };
            
            // 布尔值与数字的特殊比较
            if let (Some(left_raw), Some(right_raw)) = (left_raw, right_raw) {
                // 布尔值和数字的比较
                if let (Value::Bool(left_bool), Value::Number(right_num)) = (left_raw, right_raw) {
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
                if let (Value::Number(left_num), Value::Bool(right_bool)) = (left_raw, right_raw) {
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
                if let (Value::String(left_str), Value::Number(right_num)) = (left_raw, right_raw) {
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
                if let (Value::Number(left_num), Value::String(right_str)) = (left_raw, right_raw) {
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
                if let Value::Null = *left_raw {
                    if let Value::Number(right_num) = right_raw {
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
                
                if let Value::Null = *right_raw {
                    if let Value::Number(left_num) = left_raw {
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

pub fn execute_function(func: &Value, context: &mut Context, params: Option<&Value>) -> Result<()> {
    // 创建函数的局部变量作用域
    // 支持嵌套函数调用和递归函数
    let mut function_scope = HashMap::new();
    
    // 验证函数结构
    let body = func.get("body").ok_or_else(|| {
        InterpreterError::InvalidProgramStructure(
            "杂鱼~函数缺少 'body' 字段".to_string()
        )
    })?;

    let statements = body.as_array().ok_or_else(|| {
        InterpreterError::InvalidProgramStructure(
            "杂鱼~函数 'body' 必须是一个数组".to_string()
        )
    })?;

    // 备份当前上下文中的变量
    // 这样可以在函数执行结束后恢复原始变量状态
    // 同时保留函数返回值和创建的全局变量
    let original_variables = context.variables.clone();

    // 处理参数
    if let Some(params) = params {
        if let Some(params_obj) = params.as_object() {
            for (param_name, param_value) in params_obj {
                let value = if let Some(text) = param_value.as_str() {
                    if text.starts_with("@") {
                        if let Some(val) = context.get_value(text) {
                            val.clone()
                        } else {
                            Value::String(text.to_string())
                        }
                    } else {
                        Value::String(text.to_string())
                    }
                } else {
                    param_value.clone()
                };
                // 将参数存储在函数作用域中
                // 这允许在函数执行过程中访问参数值
                function_scope.insert(param_name.clone(), value.clone());
                context.set_variable(param_name.clone(), value)?;
            }
        } else {
            return Err(InterpreterError::FunctionError(
                "杂鱼~函数参数必须是一个对象".to_string()
            ));
        }
    }

    // 执行函数体
    for stmt in statements {
        if let Some(obj) = stmt.as_object() {
            if let Some((stmt_type, args)) = obj.iter().next() {
                execute_statement(stmt_type, args, context)?;
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~语句对象为空".to_string()
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~语句必须是一个对象".to_string()
            ));
        }
    }

    // 获取函数的返回值
    // 首先检查通用的result变量
    // 如果没有，检查特定的factorial_result变量(为阶乘函数特别处理)
    // 如果都没有，返回null
    let function_result = if let Some(result) = context.variables.get("result") {
        result.clone()
    } else if let Some(factorial_result) = context.variables.get("factorial_result") {
        // 特殊处理阶乘函数的结果变量
        factorial_result.clone()
    } else {
        Value::Null
    };

    // 恢复原始上下文变量，但保留函数的返回值和全局变量
    // 这是函数调用的关键部分，确保：
    // 1. 函数的本地变量不会污染全局作用域
    // 2. 函数的返回值可以正确传递给调用者
    // 3. 函数中创建的新全局变量仍然保留
    let mut new_context_variables = HashMap::new();
    
    // 1. 先复制原始变量
    for (key, value) in &original_variables {
        new_context_variables.insert(key.clone(), value.clone());
    }
    
    // 2. 更新全局作用域中新创建的变量
    // 但排除临时结果变量和内部使用的变量
    for (key, value) in &context.variables {
        if !original_variables.contains_key(key) && key != "result" && key != "factorial_result" {
            new_context_variables.insert(key.clone(), value.clone());
        }
    }
    
    // 3. 设置函数结果
    // 统一使用"result"变量存储函数返回值
    new_context_variables.insert("result".to_string(), function_result);
    
    // 更新上下文
    context.variables = new_context_variables;

    Ok(())
}

fn execute_while_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(obj) = args.as_object() {
        if let (Some(condition), Some(body)) = (obj.get("condition"), obj.get("body")) {
            while evaluate_condition(condition, context) {
                if let Some(statements) = body.as_array() {
                    for stmt in statements {
                        if let Some(obj) = stmt.as_object() {
                            if let Some((stmt_type, args)) = obj.iter().next() {
                                execute_statement(stmt_type, args, context)?;
                            }
                        }
                    }
                }
            }
            Ok(())
        } else {
            Err(InterpreterError::RuntimeError(
                "杂鱼~'while' 语句缺少 'condition' 或 'body' 字段".to_string()
            ))
        }
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'while' 语句的参数必须是一个对象".to_string()
        ))
    }
}

fn execute_for_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(obj) = args.as_object() {
        // 支持两种for循环语法
        if let Some(range) = obj.get("range") {
            // 新的范围语法: {"for": {"var": "i", "range": [1, 5], "body": [...]}}
            if let (Some(var_name), Some(range_array), Some(body)) = (
                obj.get("var").and_then(|v| v.as_str()),
                range.as_array(),
                obj.get("body")
            ) {
                if range_array.len() == 2 {
                    let start = get_number_value(&range_array[0], context).unwrap_or(0.0);
                    let end = get_number_value(&range_array[1], context).unwrap_or(0.0);
                    let step = obj.get("step").and_then(|v| get_number_value(v, context)).unwrap_or(1.0);
                    
                    // 只在调试模式下输出调试信息
                    if is_debug_mode() {
                        println!("DEBUG: for 循环从 {} 到 {} 步长 {}", start, end, step);
                    }
                    
                    let mut current = start;
                    // 修改循环条件，包含等于情况
                    while (step > 0.0 && current <= end) || (step < 0.0 && current >= end) {
                        context.set_variable(var_name.to_string(), Value::Number(serde_json::Number::from_f64(current).unwrap()))?;
                        
                        if let Some(statements) = body.as_array() {
                            for stmt in statements {
                                if let Some(obj) = stmt.as_object() {
                                    if let Some((stmt_type, args)) = obj.iter().next() {
                                        execute_statement(stmt_type, args, context)?;
                                    }
                                }
                            }
                        }
                        
                        current += step;
                    }
                    Ok(())
                } else {
                    Err(InterpreterError::RuntimeError(
                        "杂鱼~'range' 必须是一个包含两个数字的数组".to_string()
                    ))
                }
            } else {
                Err(InterpreterError::RuntimeError(
                    "杂鱼~'for' 语句缺少必要的字段".to_string()
                ))
            }
        } else {
            // 原有的for循环语法
            if let (Some(var_name), Some(from), Some(to), Some(body)) = (
                obj.get("var").and_then(|v| v.as_str()),
                obj.get("from"),
                obj.get("to"),
                obj.get("body")
            ) {
                let start = get_number_value(from, context).unwrap_or(0.0);
                let end = get_number_value(to, context).unwrap_or(0.0);
                let step = obj.get("step").and_then(|v| get_number_value(v, context)).unwrap_or(1.0);
                
                // 只在调试模式下输出调试信息
                if is_debug_mode() {
                    println!("DEBUG: for 循环从 {} 到 {} 步长 {}", start, end, step);
                }
                
                let mut current = start;
                // 修改循环条件，包含等于情况
                while (step > 0.0 && current <= end) || (step < 0.0 && current >= end) {
                    context.set_variable(var_name.to_string(), Value::Number(serde_json::Number::from_f64(current).unwrap()))?;
                    
                    if let Some(statements) = body.as_array() {
                        for stmt in statements {
                            if let Some(obj) = stmt.as_object() {
                                if let Some((stmt_type, args)) = obj.iter().next() {
                                    execute_statement(stmt_type, args, context)?;
                                }
                            }
                        }
                    }
                    
                    current += step;
                }
                Ok(())
            } else {
                Err(InterpreterError::RuntimeError(
                    "杂鱼~'for' 语句缺少必要的字段".to_string()
                ))
            }
        }
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'for' 语句的参数必须是一个对象".to_string()
        ))
    }
}

fn get_number_value(value: &Value, context: &Context) -> Option<f64> {
    match value {
        Value::Number(n) => n.as_f64(),
        Value::String(s) if s.starts_with("@") => {
            context.get_value(s)
                .and_then(|v| match v {
                    Value::Number(n) => n.as_f64(),
                    Value::String(s) => s.parse().ok(),
                    Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
                    Value::Null => Some(0.0),
                    Value::Array(arr) => if arr.is_empty() { Some(0.0) } else { None },
                    Value::Object(obj) => if obj.is_empty() { Some(0.0) } else { None }
                })
        }
        Value::String(s) => s.parse().ok(),
        Value::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        Value::Null => Some(0.0),
        Value::Array(arr) => if arr.is_empty() { Some(0.0) } else { None },
        Value::Object(obj) => if obj.is_empty() { Some(0.0) } else { None }
    }
}

// 执行注释语句 - 不做任何操作，仅在调试模式下显示注释内容
fn execute_comment_statement(args: &Value, context: &mut Context) -> Result<()> {
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

// 执行系统命令
fn execute_exec_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(obj) = args.as_object() {
        // 获取命令
        let cmd = if let Some(cmd) = obj.get("cmd") {
            context.resolve_value(cmd)
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'exec' 语句缺少 'cmd' 字段".to_string()
            ));
        };
        
        // 获取参数（可选）
        let args_arr = if let Some(arr) = obj.get("args").and_then(|a| a.as_array()) {
            arr.iter()
                .map(|arg| context.resolve_value(arg))
                .collect::<Vec<String>>()
        } else {
            Vec::new()
        };
        
        // 获取输出变量名（可选）
        let output_var = obj.get("output")
            .and_then(|v| v.as_str())
            .unwrap_or("result");
        
        // 执行命令
        let output = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(&["/C", &cmd])
                .args(&args_arr)
                .output()
        } else {
            Command::new("sh")
                .args(&["-c", &format!("{} {}", cmd, args_arr.join(" "))])
                .output()
        };
        
        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let status = output.status.code().unwrap_or(-1);
                
                // 创建结果对象
                let mut result_obj = serde_json::Map::new();
                result_obj.insert("stdout".to_string(), Value::String(stdout));
                result_obj.insert("stderr".to_string(), Value::String(stderr));
                result_obj.insert("status".to_string(), Value::Number(serde_json::Number::from(status)));
                
                // 保存结果
                context.set_variable(output_var.to_string(), Value::Object(result_obj))?;
                
                Ok(())
            },
            Err(e) => {
                Err(InterpreterError::RuntimeError(
                    format!("杂鱼~执行命令失败: {}", e)
                ))
            }
        }
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'exec' 语句的参数必须是一个对象".to_string()
        ))
    }
} 
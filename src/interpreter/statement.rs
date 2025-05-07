use serde_json::Value;
use super::context::Context;
use super::error::{InterpreterError, Result};
use super::error::error_messages::statement::{self, switch, control_flow, array, exec};
use crate::modules::jl_module;
use crate::is_debug_mode;
use std::collections::HashMap;
use std::process::Command;
use regex::Regex;

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
        "switch" => execute_switch_statement(args, context),
        "array.create" => execute_array_create(args, context),
        "array.push" => execute_array_push(args, context),
        "array.pop" => execute_array_pop(args, context),
        "array.get" => execute_array_get(args, context),
        "array.set" => execute_array_set(args, context),
        "array.length" => execute_array_length(args, context),
        "array.slice" => execute_array_slice(args, context),
        "object.create" => execute_object_create(args, context),
        "object.get" => execute_object_get(args, context),
        "object.set" => execute_object_set(args, context),
        "object.has" => execute_object_has(args, context),
        "object.keys" => execute_object_keys(args, context),
        "object.values" => execute_object_values(args, context),
        "object.delete" => execute_object_delete(args, context),
        "regex.match" => execute_regex_match(args, context),
        "regex.test" => execute_regex_test(args, context),
        "regex.replace" => execute_regex_replace(args, context),
        "regex.split" => execute_regex_split(args, context),
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
                                                    statement::missing_parameter(param_name)
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
                            super::error::error_messages::context::module_not_found(module_name)
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
                            super::error::error_messages::context::function_name_conflict_builtin(stmt_type)
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
                                            super::error::error_messages::context::function_name_conflict_module(stmt_type)
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
                                statement::FUNCTION_PARAMS_MUST_BE_OBJ.to_string()
                            ));
                        }
                    } else {
                        return Err(InterpreterError::FunctionError(
                            statement::FUNCTION_MISSING_PARAMS.to_string()
                        ));
                    };

                    // 创建参数对象
                    let mut params_map = serde_json::Map::new();
                    for (i, (param_name, _)) in params_obj.iter().enumerate() {
                        if let Some(arg) = args_array.get(i) {
                            params_map.insert(param_name.clone(), arg.clone() as Value);
                        } else {
                            return Err(InterpreterError::FunctionError(
                                statement::missing_parameter(param_name)
                            ));
                        }
                    }

                    let params = Value::Object(params_map);
                    return execute_function(&func, context, Some(&params));
                }
            }

            Err(InterpreterError::RuntimeError(
                statement::unknown_statement_type(stmt_type)
            ))
        }
    }
}

// 检查是否是内置语句
fn is_builtin_statement(name: &str) -> bool {
    matches!(name, "var" | "echo" | "concat" | "if" | "call" | "while" | "for" | "comment" | "exec" | "switch" 
             | "array.create" | "array.push" | "array.pop" | "array.get" | "array.set" | "array.length" | "array.slice"
             | "object.create" | "object.get" | "object.set" | "object.has" | "object.keys" | "object.values" | "object.delete"
             | "regex.match" | "regex.test" | "regex.replace" | "regex.split")
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
            statement::param_must_be_obj("var")
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
                statement::FUNCTION_CALL_MISSING_NAME.to_string()
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
        statement::INVALID_FUNCTION_CALL.to_string()
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
            statement::FUNCTION_MISSING_BODY.to_string()
        )
    })?;

    let statements = body.as_array().ok_or_else(|| {
        InterpreterError::InvalidProgramStructure(
            statement::FUNCTION_BODY_NOT_ARRAY.to_string()
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
                statement::FUNCTION_PARAMS_MUST_BE_OBJ.to_string()
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
                    statement::STATEMENT_EMPTY.to_string()
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                statement::STATEMENT_NOT_OBJECT.to_string()
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
                control_flow::WHILE_MISSING_FIELDS.to_string()
            ))
        }
    } else {
        Err(InterpreterError::RuntimeError(
            control_flow::WHILE_ARGS_NOT_OBJ.to_string()
        ))
    }
}

fn execute_for_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(obj) = args.as_object() {
        // 支持数组遍历语法
        if let Some(array_expr) = obj.get("in") {
            // 数组遍历语法: {"for": {"var": "item", "in": "@var.array", "body": [...]}}
            if let (Some(var_name), Some(body)) = (
                obj.get("var").and_then(|v| v.as_str()),
                obj.get("body")
            ) {
                // 获取要遍历的数组
                let array_value = if let Some(array_ref) = array_expr.as_str() {
                    if array_ref.starts_with("@") {
                        if let Some(val) = context.get_value(array_ref) {
                            val.clone()
                        } else {
                            return Err(InterpreterError::RuntimeError(
                                format!("杂鱼~变量 '{}' 不存在", array_ref)
                            ));
                        }
                    } else {
                        Value::String(array_ref.to_string())
                    }
                } else {
                    array_expr.clone()
                };
                
                // 确保是数组类型
                let array = if let Value::Array(arr) = array_value {
                    arr
                } else {
                    return Err(InterpreterError::RuntimeError(
                        "杂鱼~'for..in' 的in参数必须是一个数组".to_string()
                    ));
                };
                
                // 遍历数组的每个元素
                for item in array {
                    // 设置循环变量
                    context.set_variable(var_name.to_string(), item.clone())?;
                    
                    // 执行循环体
                    if let Some(statements) = body.as_array() {
                        for stmt in statements {
                            if let Some(obj) = stmt.as_object() {
                                if let Some((stmt_type, args)) = obj.iter().next() {
                                    execute_statement(stmt_type, args, context)?;
                                }
                            }
                        }
                    } else {
                        return Err(InterpreterError::RuntimeError(
                            "杂鱼~循环体必须是语句数组".to_string()
                        ));
                    }
                }
                
                return Ok(());
            } else {
                return Err(InterpreterError::RuntimeError(
                    control_flow::FOR_MISSING_FIELDS.to_string()
                ));
            }
        }
        // 支持两种for循环语法
        else if let Some(range) = obj.get("range") {
            // 以下是原有代码
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
                        control_flow::FOR_RANGE_INVALID.to_string()
                    ))
                }
            } else {
                Err(InterpreterError::RuntimeError(
                    control_flow::FOR_MISSING_FIELDS.to_string()
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
                    control_flow::FOR_MISSING_FIELDS.to_string()
                ))
            }
        }
    } else {
        Err(InterpreterError::RuntimeError(
            control_flow::FOR_ARGS_NOT_OBJ.to_string()
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
                exec::MISSING_CMD.to_string()
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
                    exec::execution_failed(&e.to_string())
                ))
            }
        }
    } else {
        Err(InterpreterError::RuntimeError(
            exec::ARGS_NOT_OBJ.to_string()
        ))
    }
}

// 执行switch语句
fn execute_switch_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(obj) = args.as_object() {
        if let (Some(expr), Some(cases)) = (obj.get("expr"), obj.get("cases")) {
            let expr_value = context.resolve_value(expr);
            
            if let Some(cases_array) = cases.as_array() {
                let mut default_case = None;
                let mut match_found = false;
                
                // 遍历所有case
                for case in cases_array {
                    if let Some(case_obj) = case.as_object() {
                        // 检查是否是default case
                        if case_obj.contains_key("default") {
                            default_case = case_obj.get("body");
                            continue;
                        }
                        
                        // 正常case处理
                        if let (Some(value), Some(body)) = (case_obj.get("value"), case_obj.get("body")) {
                            let case_value = context.resolve_value(value);
                            
                            // 如果case值匹配
                            if case_value == expr_value {
                                match_found = true;
                                
                                // 执行case的语句体
                                if let Some(statements) = body.as_array() {
                                    for stmt in statements {
                                        if let Some(obj) = stmt.as_object() {
                                            if let Some((stmt_type, args)) = obj.iter().next() {
                                                execute_statement(stmt_type, args, context)?;
                                            }
                                        }
                                    }
                                } else {
                                    return Err(InterpreterError::RuntimeError(
                                        switch::CASE_BODY_NOT_ARRAY.to_string()
                                    ));
                                }
                                
                                // 检查是否需要break（默认行为是break）
                                if !case_obj.contains_key("fallthrough") || 
                                   !case_obj.get("fallthrough").unwrap().as_bool().unwrap_or(false) {
                                    break;
                                }
                            }
                        } else {
                            return Err(InterpreterError::RuntimeError(
                                switch::CASE_MISSING_FIELDS.to_string()
                            ));
                        }
                    } else {
                        return Err(InterpreterError::RuntimeError(
                            switch::CASE_NOT_OBJ.to_string()
                        ));
                    }
                }
                
                // 如果没有匹配的case但有默认case，执行默认case
                if !match_found && default_case.is_some() {
                    if let Some(statements) = default_case.unwrap().as_array() {
                        for stmt in statements {
                            if let Some(obj) = stmt.as_object() {
                                if let Some((stmt_type, args)) = obj.iter().next() {
                                    execute_statement(stmt_type, args, context)?;
                                }
                            }
                        }
                    } else {
                        return Err(InterpreterError::RuntimeError(
                            switch::DEFAULT_BODY_NOT_ARRAY.to_string()
                        ));
                    }
                }
                
                Ok(())
            } else {
                Err(InterpreterError::RuntimeError(
                    switch::CASES_NOT_ARRAY.to_string()
                ))
            }
        } else {
            Err(InterpreterError::RuntimeError(
                switch::MISSING_EXPR_OR_CASES.to_string()
            ))
        }
    } else {
        Err(InterpreterError::RuntimeError(
            switch::ARGS_NOT_OBJ.to_string()
        ))
    }
}

// 数组相关操作函数

// 创建新数组
fn execute_array_create(args: &Value, context: &mut Context) -> Result<()> {
    let result = if let Some(args_array) = args.as_array() {
        // 如果提供了初始元素，则使用它们创建数组
        // 处理每个元素，解析变量引用
        let resolved_elements: Vec<Value> = args_array.iter()
            .map(|elem| {
                if let Some(text) = elem.as_str() {
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
                    elem.clone()
                }
            })
            .collect();
        
        Value::Array(resolved_elements)
    } else if let Some(obj) = args.as_object() {
        if let Some(size) = obj.get("size") {
            // 如果提供了size参数，则创建指定大小的空数组
            let size_value = context.resolve_value(size);
            if let Ok(size) = size_value.parse::<usize>() {
                // 如果提供了初始值，则使用它填充数组
                let initial_value = if let Some(init) = obj.get("initial") {
                    if let Some(text) = init.as_str() {
                        if text.starts_with("@") {
                            if let Some(val) = context.get_value(text) {
                                val.clone()
                            } else {
                                Value::Null
                            }
                        } else {
                            Value::String(text.to_string())
                        }
                    } else {
                        init.clone()
                    }
                } else {
                    Value::Null
                };
                
                let elements = vec![initial_value; size];
                Value::Array(elements)
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::CREATE_SIZE_NOT_NUMBER.to_string()
                ));
            }
        } else {
            // 如果没有指定size，则创建一个空数组
            Value::Array(Vec::new())
        }
    } else {
        // 如果参数不是数组也不是对象，则创建一个空数组
        Value::Array(Vec::new())
    };
    
    context.set_variable("result".to_string(), result)?;
    Ok(())
}

// 向数组末尾添加元素
fn execute_array_push(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                array::PUSH_MISSING_ARGS.to_string()
            ));
        }
        
        // 第一个参数是数组
        let array_ref = &args_array[0];
        let array_var_name = if let Some(var_name) = array_ref.as_str() {
            if var_name.starts_with("@var.") {
                &var_name[5..] // 去掉 "@var." 前缀
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::PUSH_FIRST_ARG_NOT_ARRAY_REF.to_string()
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                array::PUSH_FIRST_ARG_NOT_STRING_REF.to_string()
            ));
        };
        
        // 获取数组变量
        let array_value = if let Some(val) = context.get_value(&format!("@var.{}", array_var_name)) {
            val.clone()
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_found(array_var_name)
            ));
        };
        
        // 确保变量是一个数组
        let mut array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_array(array_var_name)
            ));
        };
        
        // 添加其余的参数到数组
        for item in &args_array[1..] {
            let resolved_item = if let Some(text) = item.as_str() {
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
                item.clone()
            };
            
            array.push(resolved_item);
        }
        
        // 更新数组变量
        context.set_variable(array_var_name.to_string(), Value::Array(array.clone()))?;
        
        // 将修改后的数组也存储在result变量中
        context.set_variable("result".to_string(), Value::Array(array))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.push")
        ))
    }
}

// 从数组末尾移除元素
fn execute_array_pop(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.is_empty() {
            return Err(InterpreterError::RuntimeError(
                array::POP_MISSING_ARGS.to_string()
            ));
        }
        
        // 获取数组变量引用
        let array_ref = &args_array[0];
        let array_var_name = if let Some(var_name) = array_ref.as_str() {
            if var_name.starts_with("@var.") {
                &var_name[5..] // 去掉 "@var." 前缀
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::POP_ARG_NOT_ARRAY_REF.to_string()
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                array::POP_ARG_NOT_STRING_REF.to_string()
            ));
        };
        
        // 获取数组变量
        let array_value = if let Some(val) = context.get_value(&format!("@var.{}", array_var_name)) {
            val.clone()
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_found(array_var_name)
            ));
        };
        
        // 确保变量是一个数组
        let mut array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_array(array_var_name)
            ));
        };
        
        // 从数组末尾移除元素
        let popped = if !array.is_empty() {
            array.pop().unwrap()
        } else {
            Value::Null
        };
        
        // 更新数组变量
        context.set_variable(array_var_name.to_string(), Value::Array(array))?;
        
        // 将弹出的元素存储在result变量中
        context.set_variable("result".to_string(), popped)?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.pop")
        ))
    }
}

// 获取数组指定索引的元素
fn execute_array_get(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                array::GET_MISSING_ARGS.to_string()
            ));
        }
        
        // 获取数组
        let array_value = if let Some(array_ref_str) = args_array[0].as_str() {
            if array_ref_str.starts_with("@") {
                if let Some(val) = context.get_value(array_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        array::var_not_found(array_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::GET_FIRST_ARG_NOT_ARRAY_REF.to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是数组类型
        let array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::GET_FIRST_ARG_NOT_ARRAY.to_string()
            ));
        };
        
        // 获取索引
        let index_value = context.resolve_value(&args_array[1]);
        let index = if let Ok(idx) = index_value.parse::<usize>() {
            idx
        } else {
            // 尝试直接从变量中获取值
            if let Some(index_str) = args_array[1].as_str() {
                if index_str.starts_with("@") {
                    if let Some(val) = context.get_value(index_str) {
                        if let Some(num) = val.as_u64() {
                            num as usize
                        } else if let Some(num) = val.as_i64() {
                            num as usize
                        } else if let Some(num) = val.as_f64() {
                            num as usize
                        } else {
                            return Err(InterpreterError::RuntimeError(
                                array::GET_SECOND_ARG_NOT_INDEX.to_string()
                            ));
                        }
                    } else {
                        return Err(InterpreterError::RuntimeError(
                            array::var_not_found(index_str)
                        ));
                    }
                } else {
                    return Err(InterpreterError::RuntimeError(
                        array::GET_SECOND_ARG_NOT_INDEX.to_string()
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::GET_SECOND_ARG_NOT_INDEX.to_string()
                ));
            }
        };
        
        // 获取元素
        let element = if index < array.len() {
            array[index].clone()
        } else {
            Value::Null
        };
        
        // 存储结果
        context.set_variable("result".to_string(), element)?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.get")
        ))
    }
}

// 设置数组指定索引的元素
fn execute_array_set(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 3 {
            return Err(InterpreterError::RuntimeError(
                array::SET_MISSING_ARGS.to_string()
            ));
        }
        
        // 获取数组变量引用
        let array_ref = &args_array[0];
        let array_var_name = if let Some(var_name) = array_ref.as_str() {
            if var_name.starts_with("@var.") {
                &var_name[5..] // 去掉 "@var." 前缀
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::set_first_arg_not_array_ref(var_name)
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                array::set_first_arg_not_string_ref()
            ));
        };
        
        // 获取数组变量
        let array_value = if let Some(val) = context.get_value(&format!("@var.{}", array_var_name)) {
            val.clone()
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_found(array_var_name)
            ));
        };
        
        // 确保变量是一个数组
        let mut array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_array(array_var_name)
            ));
        };
        
        // 获取索引
        let index_value = context.resolve_value(&args_array[1]);
        let index = if let Ok(idx) = index_value.parse::<usize>() {
            idx
        } else {
            return Err(InterpreterError::RuntimeError(
                array::set_second_arg_must_be_number()
            ));
        };
        
        // 获取新值
        let new_value = if let Some(text) = args_array[2].as_str() {
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
            args_array[2].clone()
        };
        
        // 如果索引超出范围，扩展数组
        if index >= array.len() {
            array.resize(index + 1, Value::Null);
        }
        
        // 设置元素
        array[index] = new_value;
        
        // 更新数组变量
        context.set_variable(array_var_name.to_string(), Value::Array(array.clone()))?;
        
        // 将修改后的数组也存储在result变量中
        context.set_variable("result".to_string(), Value::Array(array))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.set")
        ))
    }
}

// 获取数组长度
fn execute_array_length(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.is_empty() {
            return Err(InterpreterError::RuntimeError(
                array::LENGTH_MISSING_ARGS.to_string()
            ));
        }
        
        // 获取数组
        let array_value = if let Some(array_ref_str) = args_array[0].as_str() {
            if array_ref_str.starts_with("@") {
                if let Some(val) = context.get_value(array_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        array::var_not_found(array_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::LENGTH_ARG_NOT_ARRAY_REF.to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是数组类型
        let array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::LENGTH_ARG_NOT_ARRAY.to_string()
            ));
        };
        
        // 获取长度
        let length = array.len();
        
        // 存储结果
        context.set_variable("result".to_string(), Value::Number(serde_json::Number::from(length)))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.length")
        ))
    }
}

// 获取数组切片
fn execute_array_slice(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                array::SLICE_MISSING_ARGS.to_string()
            ));
        }
        
        // 获取数组
        let array_value = if let Some(array_ref_str) = args_array[0].as_str() {
            if array_ref_str.starts_with("@") {
                if let Some(val) = context.get_value(array_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        array::var_not_found(array_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::SLICE_FIRST_ARG_NOT_ARRAY_REF.to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是数组类型
        let array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::SLICE_FIRST_ARG_NOT_ARRAY.to_string()
            ));
        };
        
        // 获取开始索引
        let start_value = context.resolve_value(&args_array[1]);
        let start = if let Ok(idx) = start_value.parse::<usize>() {
            idx
        } else {
            return Err(InterpreterError::RuntimeError(
                array::SLICE_SECOND_ARG_NOT_INDEX.to_string()
            ));
        };
        
        // 获取结束索引（如果提供）
        let end = if args_array.len() > 2 {
            let end_value = context.resolve_value(&args_array[2]);
            if let Ok(idx) = end_value.parse::<usize>() {
                idx
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::SLICE_THIRD_ARG_NOT_INDEX.to_string()
                ));
            }
        } else {
            array.len()
        };
        
        // 创建切片
        let slice = if start <= array.len() {
            let actual_end = end.min(array.len());
            if start <= actual_end {
                array[start..actual_end].to_vec()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        // 存储结果
        context.set_variable("result".to_string(), Value::Array(slice))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.slice")
        ))
    }
}

// =========== 对象操作函数 ===========

// 创建新对象
fn execute_object_create(args: &Value, context: &mut Context) -> Result<()> {
    let result = if let Some(obj) = args.as_object() {
        // 如果提供了初始属性，则使用它们创建对象
        // 处理每个属性，解析变量引用
        let mut result_obj = serde_json::Map::new();
        for (key, value) in obj {
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
            result_obj.insert(key.clone(), resolved_value);
        }
        Value::Object(result_obj)
    } else {
        // 如果参数不是对象，则创建一个空对象
        Value::Object(serde_json::Map::new())
    };
    
    context.set_variable("result".to_string(), result)?;
    Ok(())
}

// 获取对象属性
fn execute_object_get(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.get' 需要两个参数：对象和属性名".to_string()
            ));
        }
        
        // 获取对象
        let obj_value = if let Some(obj_ref_str) = args_array[0].as_str() {
            if obj_ref_str.starts_with("@") {
                if let Some(val) = context.get_value(obj_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        format!("杂鱼~变量 '{}' 不存在", obj_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.get' 的第一个参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是对象类型
        let obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.get' 的第一个参数必须是一个对象".to_string()
            ));
        };
        
        // 获取属性名
        let key = context.resolve_value(&args_array[1]);
        
        // 获取属性值
        let value = if let Some(val) = obj.get(&key) {
            val.clone()
        } else {
            Value::Null
        };
        
        // 存储结果
        context.set_variable("result".to_string(), value)?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.get' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// 设置对象属性
fn execute_object_set(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 3 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.set' 需要三个参数：对象、属性名和值".to_string()
            ));
        }
        
        // 获取对象变量引用
        let obj_ref = &args_array[0];
        let obj_var_name = if let Some(var_name) = obj_ref.as_str() {
            if var_name.starts_with("@var.") {
                &var_name[5..] // 去掉 "@var." 前缀
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.set' 的第一个参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.set' 的第一个参数必须是一个字符串变量引用".to_string()
            ));
        };
        
        // 获取对象变量
        let obj_value = if let Some(val) = context.get_value(&format!("@var.{}", obj_var_name)) {
            val.clone()
        } else {
            return Err(InterpreterError::RuntimeError(
                format!("杂鱼~变量 '{}' 不存在", obj_var_name)
            ));
        };
        
        // 确保变量是一个对象
        let mut obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                format!("杂鱼~变量 '{}' 不是一个对象", obj_var_name)
            ));
        };
        
        // 获取属性名
        let key = context.resolve_value(&args_array[1]);
        
        // 获取新值
        let new_value = if let Some(text) = args_array[2].as_str() {
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
            args_array[2].clone()
        };
        
        // 设置属性
        obj.insert(key, new_value);
        
        // 更新对象变量
        context.set_variable(obj_var_name.to_string(), Value::Object(obj.clone()))?;
        
        // 将修改后的对象也存储在result变量中
        context.set_variable("result".to_string(), Value::Object(obj))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.set' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// 检查对象是否有属性
fn execute_object_has(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.has' 需要两个参数：对象和属性名".to_string()
            ));
        }
        
        // 获取对象
        let obj_value = if let Some(obj_ref_str) = args_array[0].as_str() {
            if obj_ref_str.starts_with("@") {
                if let Some(val) = context.get_value(obj_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        format!("杂鱼~变量 '{}' 不存在", obj_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.has' 的第一个参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是对象类型
        let obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.has' 的第一个参数必须是一个对象".to_string()
            ));
        };
        
        // 获取属性名
        let key = context.resolve_value(&args_array[1]);
        
        // 检查属性是否存在
        let has_property = obj.contains_key(&key);
        
        // 存储结果
        context.set_variable("result".to_string(), Value::Bool(has_property))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.has' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// 获取对象所有键
fn execute_object_keys(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.is_empty() {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.keys' 需要一个参数：对象".to_string()
            ));
        }
        
        // 获取对象
        let obj_value = if let Some(obj_ref_str) = args_array[0].as_str() {
            if obj_ref_str.starts_with("@") {
                if let Some(val) = context.get_value(obj_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        format!("杂鱼~变量 '{}' 不存在", obj_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.keys' 的参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是对象类型
        let obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.keys' 的参数必须是一个对象".to_string()
            ));
        };
        
        // 获取所有键
        let keys: Vec<Value> = obj.keys()
            .map(|k| Value::String(k.clone()))
            .collect();
        
        // 存储结果
        context.set_variable("result".to_string(), Value::Array(keys))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.keys' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// 获取对象所有值
fn execute_object_values(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.is_empty() {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.values' 需要一个参数：对象".to_string()
            ));
        }
        
        // 获取对象
        let obj_value = if let Some(obj_ref_str) = args_array[0].as_str() {
            if obj_ref_str.starts_with("@") {
                if let Some(val) = context.get_value(obj_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        format!("杂鱼~变量 '{}' 不存在", obj_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.values' 的参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是对象类型
        let obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.values' 的参数必须是一个对象".to_string()
            ));
        };
        
        // 获取所有值
        let values: Vec<Value> = obj.values().cloned().collect();
        
        // 存储结果
        context.set_variable("result".to_string(), Value::Array(values))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.values' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// 删除对象属性
fn execute_object_delete(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.delete' 需要两个参数：对象和属性名".to_string()
            ));
        }
        
        // 获取对象变量引用
        let obj_ref = &args_array[0];
        let obj_var_name = if let Some(var_name) = obj_ref.as_str() {
            if var_name.starts_with("@var.") {
                &var_name[5..] // 去掉 "@var." 前缀
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.delete' 的第一个参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.delete' 的第一个参数必须是一个字符串变量引用".to_string()
            ));
        };
        
        // 获取对象变量
        let obj_value = if let Some(val) = context.get_value(&format!("@var.{}", obj_var_name)) {
            val.clone()
        } else {
            return Err(InterpreterError::RuntimeError(
                format!("杂鱼~变量 '{}' 不存在", obj_var_name)
            ));
        };
        
        // 确保变量是一个对象
        let mut obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                format!("杂鱼~变量 '{}' 不是一个对象", obj_var_name)
            ));
        };
        
        // 获取属性名
        let key = context.resolve_value(&args_array[1]);
        
        // 删除属性，并获取是否存在该属性
        let had_property = obj.remove(&key).is_some();
        
        // 更新对象变量
        context.set_variable(obj_var_name.to_string(), Value::Object(obj.clone()))?;
        
        // 将结果存储在result变量中
        context.set_variable("result".to_string(), Value::Bool(had_property))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.delete' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// =========== 正则表达式函数 ===========

// 正则表达式匹配
fn execute_regex_match(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'regex.match' 需要两个参数：正则表达式和要匹配的字符串".to_string()
            ));
        }
        
        // 获取正则表达式模式
        let pattern = context.resolve_value(&args_array[0]);
        
        // 获取要匹配的字符串
        let text = context.resolve_value(&args_array[1]);
        
        // 编译正则表达式
        let regex = match Regex::new(&pattern) {
            Ok(re) => re,
            Err(err) => {
                return Err(InterpreterError::RuntimeError(
                    format!("杂鱼~正则表达式编译错误: {}", err)
                ));
            }
        };
        
        // 执行匹配
        let captures = regex.captures(&text);
        
        // 处理匹配结果
        let result = if let Some(caps) = captures {
            // 创建匹配结果数组
            let mut matches = Vec::new();
            
            // 添加完整匹配
            if let Some(m) = caps.get(0) {
                matches.push(Value::String(m.as_str().to_string()));
            }
            
            // 添加捕获组
            for i in 1..caps.len() {
                if let Some(m) = caps.get(i) {
                    matches.push(Value::String(m.as_str().to_string()));
                } else {
                    matches.push(Value::Null);
                }
            }
            
            Value::Array(matches)
        } else {
            // 无匹配
            Value::Null
        };
        
        // 存储结果
        context.set_variable("result".to_string(), result)?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'regex.match' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// 正则表达式测试
fn execute_regex_test(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'regex.test' 需要两个参数：正则表达式和要测试的字符串".to_string()
            ));
        }
        
        // 获取正则表达式模式
        let pattern = context.resolve_value(&args_array[0]);
        
        // 获取要测试的字符串
        let text = context.resolve_value(&args_array[1]);
        
        // 编译正则表达式
        let regex = match Regex::new(&pattern) {
            Ok(re) => re,
            Err(err) => {
                return Err(InterpreterError::RuntimeError(
                    format!("杂鱼~正则表达式编译错误: {}", err)
                ));
            }
        };
        
        // 执行测试
        let is_match = regex.is_match(&text);
        
        // 存储结果
        context.set_variable("result".to_string(), Value::Bool(is_match))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'regex.test' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// 正则表达式替换
fn execute_regex_replace(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 3 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'regex.replace' 需要三个参数：正则表达式、要替换的字符串和替换值".to_string()
            ));
        }
        
        // 获取正则表达式模式
        let pattern = context.resolve_value(&args_array[0]);
        
        // 获取要替换的字符串
        let text = context.resolve_value(&args_array[1]);
        
        // 获取替换值
        let replacement = context.resolve_value(&args_array[2]);
        
        // 编译正则表达式
        let regex = match Regex::new(&pattern) {
            Ok(re) => re,
            Err(err) => {
                return Err(InterpreterError::RuntimeError(
                    format!("杂鱼~正则表达式编译错误: {}", err)
                ));
            }
        };
        
        // 执行替换，支持最多9个捕获组的引用($1-$9)
        let result = regex.replace_all(&text, replacement.as_str()).to_string();
        
        // 存储结果
        context.set_variable("result".to_string(), Value::String(result))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'regex.replace' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// 正则表达式分割
fn execute_regex_split(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'regex.split' 需要两个参数：正则表达式和要分割的字符串".to_string()
            ));
        }
        
        // 获取正则表达式模式
        let pattern = context.resolve_value(&args_array[0]);
        
        // 获取要分割的字符串
        let text = context.resolve_value(&args_array[1]);
        
        // 编译正则表达式
        let regex = match Regex::new(&pattern) {
            Ok(re) => re,
            Err(err) => {
                return Err(InterpreterError::RuntimeError(
                    format!("杂鱼~正则表达式编译错误: {}", err)
                ));
            }
        };
        
        // 执行分割
        let parts: Vec<Value> = regex.split(&text)
            .map(|s| Value::String(s.to_string()))
            .collect();
        
        // 存储结果
        context.set_variable("result".to_string(), Value::Array(parts))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'regex.split' 语句的参数必须是一个数组".to_string()
        ))
    }
} 
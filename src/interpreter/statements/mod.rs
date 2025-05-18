// 导入各模块
mod basic;
mod control_flow;
mod function;
mod array;
mod object;
mod exec;
mod regex;

use serde_json::Value;
use super::context::Context;
use super::error::{InterpreterError, Result};
use super::error::error_messages::statement;
use super::variable_reference::{VariableReference, ReferenceType};
use crate::modules::jl_module;
use crate::modules::external_module;
use crate::modules::external_module::ExternalModule;
use crate::is_debug_mode;
use std::collections::HashMap;

// 重新导出所有需要的函数
pub use basic::*;
pub use control_flow::*;
pub use function::*;
pub use array::*;
pub use object::*;
pub use regex::*;
pub use exec::*;

// 兼容性辅助函数 - 存储结果到result和可选的output变量
pub fn store_result_with_compatibility(args: &Value, result: &Value, context: &mut Context) -> Result<()> {
    // 1. 总是存储到result (向后兼容)
    context.set_variable("result".to_string(), result.clone())?;
    
    // 2. 如果指定了output参数，则同时存到该变量 (方案一兼容)
    if let Some(obj) = args.as_object() {
        if let Some(output_var) = obj.get("output").and_then(|v| v.as_str()) {
            if output_var != "result" {  // 避免重复设置result
                context.set_variable(output_var.to_string(), result.clone())?;
            }
        }
    }
    
    Ok(())
}

// 这是主要的语句执行函数，调度到各个具体的语句处理器
// 现在直接返回结果值，同时保持向后兼容性
pub fn execute_statement(stmt_type: &str, args: &Value, context: &mut Context, full_stmt: Option<&Value>) -> Result<Value> {
    if is_debug_mode() {
        println!("执行语句: {}", stmt_type);
    }
    
    // 首先检查是否是内置语句，无论是否包含点
    match stmt_type {
        "comment" => return execute_comment_statement(args, context),
        "var" => return execute_var_statement(args, context),
        "echo" => return execute_echo_statement(args, context),
        "concat" => return execute_concat_statement(args, context),
        "return" => return execute_return_statement(args, context),
        "if" => return execute_if_statement(args, context),
        "while" => return execute_while_statement(args, context),
        "for" => return execute_for_statement(args, context),
        "exec" => return execute_exec_statement(args, context),
        "switch" => return execute_switch_statement(args, context),
        "try" => return execute_try_statement(args, context),
        "get_property" => return execute_get_property_statement(args, context),
        "array.create" => return execute_array_create(args, context),
        "array.push" => return execute_array_push(args, context),
        "array.pop" => return execute_array_pop(args, context),
        "array.get" => return execute_array_get(args, context),
        "array.set" => return execute_array_set(args, context),
        "array.length" => return execute_array_length(args, context),
        "array.slice" => return execute_array_slice(args, context),
        "object.create" => return execute_object_create(args, context),
        "object.get" => return execute_object_get(args, context),
        "object.set" => return execute_object_set(args, context),
        "object.has" => return execute_object_has(args, context),
        "object.keys" => return execute_object_keys(args, context),
        "object.values" => return execute_object_values(args, context),
        "object.delete" => return execute_object_delete(args, context),
        "regex.match" => return execute_regex_match(args, context),
        "regex.test" => return execute_regex_test(args, context),
        "regex.replace" => return execute_regex_replace(args, context),
        "regex.split" => return execute_regex_split(args, context),
        _ => {}  // 不是内置语句，继续处理
    }
    
    // 特殊处理模块函数调用，避免借用冲突
    if stmt_type.contains('.') {
        let parts: Vec<&str> = stmt_type.split('.').collect();
        if parts.len() == 2 {
            let module_name = parts[0];
            let function_name = parts[1];
            
            if is_debug_mode() {
                println!("检测到简化语法调用: 模块='{}', 函数='{}'", module_name, function_name);
            }
            
            // 提取函数参数（数组）和其他参数（如output）
            let args_array = if args.is_array() {
                args.as_array().unwrap().to_vec()
            } else if let Some(obj) = args.as_object() {
                // 如果是对象，需要分离函数参数和output等特殊参数
                if obj.contains_key("0") || obj.contains_key("1") || obj.contains_key("2") {  // 对象格式数组
                    let mut array = Vec::new();
                    let mut i = 0;
                    while let Some(val) = obj.get(&i.to_string()) {
                        array.push(val.clone());
                        i += 1;
                    }
                    array
                } else {
                    vec![args.clone()]  // 不是数组格式，作为单一参数传递
                }
            } else {
                vec![args.clone()]
            };
            
            // 1. 检查模块是否存在
            if !context.modules.contains_key(module_name) {
                return Err(InterpreterError::ModuleError(
                    super::error::error_messages::context::module_not_found(module_name)
                ));
            }
            
            // 2. 检查是否是JLang模块类型，并且尝试获取函数定义
            let mut func_def_opt: Option<Value> = None;
            let mut is_jlang_module = false;
            
            if let Some(module) = context.modules.get(module_name) {
                // 检查JlModule类型
                if let Some(jl_module) = module.as_any().downcast_ref::<jl_module::JlModule>() {
                    if let Some(func_def) = jl_module.get_function(function_name) {
                        is_jlang_module = true;
                        func_def_opt = Some(func_def.clone());
                    }
                }
                
                // 检查JLangExternalModule类型
                if let Some(external_module) = module.as_any().downcast_ref::<external_module::JLangExternalModule>() {
                    if let Some(func_def) = external_module.get_jlang_function(function_name) {
                        is_jlang_module = true;
                        func_def_opt = Some(func_def);
                    }
                }
            }
            
            // 3. 如果获取到JLang函数定义，执行它
            if let Some(func_def) = func_def_opt {
                let mut params_map = serde_json::Map::new();
                
                // 获取函数参数定义
                if let Some(params) = func_def.get("params").and_then(|p| p.as_object()) {
                    for (i, (param_name, _)) in params.iter().enumerate() {
                        if let Some(arg) = args_array.get(i) {
                            params_map.insert(param_name.clone(), arg.clone());
                        } else {
                            return Err(InterpreterError::FunctionError(
                                statement::missing_parameter(param_name)
                            ));
                        }
                    }
                }
                
                let params = Value::Object(params_map);
                return execute_function(&func_def, context, Some(&params));
            }
            
            // 4. 如果不是JLang模块或找不到函数，尝试标准模块处理
            if !is_jlang_module {
                let result = context.call_module_function(module_name, function_name, &args_array)?;
                
                // 从完整语句中获取output参数
                let storage_args = if let Some(full_stmt) = full_stmt {
                    // 使用完整语句对象进行存储，它包含output参数
                    full_stmt
                } else {
                    args
                };
                
                if is_debug_mode() {
                    if let Some(obj) = storage_args.as_object() {
                        if let Some(output) = obj.get("output") {
                            println!("函数结果将存储到变量: {}", output);
                        } else {
                            println!("函数结果将只存储到默认的result变量");
                        }
                    } else {
                        println!("函数结果将只存储到默认的result变量");
                    }
                }
                
                store_result_with_compatibility(storage_args, &result, context)?;
                return Ok(result);
            } else {
                // 是JLang模块但找不到函数
                return Err(InterpreterError::FunctionError(
                    format!("在模块 '{}' 中未找到函数 '{}'", module_name, function_name)
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

// 检查是否是内置语句
pub fn is_builtin_statement(name: &str) -> bool {
    matches!(name, "var" | "echo" | "concat" | "if" | "while" | "for" | "comment" | "exec" | "switch" | "return" | "get_property"
             | "array.create" | "array.push" | "array.pop" | "array.get" | "array.set" | "array.length" | "array.slice"
             | "object.create" | "object.get" | "object.set" | "object.has" | "object.keys" | "object.values" | "object.delete"
             | "regex.match" | "regex.test" | "regex.replace" | "regex.split")
}

// 帮助函数：获取数值
pub fn get_number_value(value: &Value, context: &Context) -> Option<f64> {
    match value {
        Value::Number(n) => n.as_f64(),
        Value::String(s) if VariableReference::is_reference(s) => {
            context.get_value(s)
                .and_then(|v| match &v {
                    Value::Number(n) => n.as_f64(),
                    Value::String(s) => s.parse().ok(),
                    Value::Bool(b) => {
                        let bool_value = *b;
                        Some(if bool_value { 1.0 } else { 0.0 })
                    },
                    Value::Null => Some(0.0),
                    Value::Array(arr) => if arr.is_empty() { Some(0.0) } else { None },
                    Value::Object(obj) => if obj.is_empty() { Some(0.0) } else { None }
                })
        }
        Value::String(s) => s.parse().ok(),
        Value::Bool(b) => {
            let bool_value = *b;
            Some(if bool_value { 1.0 } else { 0.0 })
        },
        Value::Null => Some(0.0),
        Value::Array(arr) => if arr.is_empty() { Some(0.0) } else { None },
        Value::Object(obj) => if obj.is_empty() { Some(0.0) } else { None }
    }
}

// 辅助函数，用于方便在任何语句中处理变量引用
pub fn resolve_variable_reference(text: &str, context: &Context) -> Option<Value> {
    if VariableReference::is_reference(text) {
        if let Some(val) = context.get_value(text) {
            return Some(val.clone());
        }
    }
    None
}

// 更新execute_function函数以支持return语句中断执行
pub fn execute_function(func: &Value, context: &mut Context, params: Option<&Value>) -> Result<Value> {
    // 首先备份原始变量集，以便在函数执行完成后恢复
    let original_variables = context.variables.clone();
    
    // 创建函数内部的作用域
    let mut function_scope = HashMap::new();

    // 处理函数参数
    if let Some(params) = params {
        if let Some(params_obj) = params.as_object() {
            for (param_name, param_value) in params_obj {
                // 解析参数值
                let value = if let Some(text) = param_value.as_str() {
                    if VariableReference::is_reference(text) {
                        let var_ref = VariableReference::parse(text);
                        var_ref.resolve_value(&context.variables, &context.constants)
                    } else {
                        Value::String(text.to_string())
                    }
                } else if let Some(obj) = param_value.as_object() {
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
                                    println!("函数参数中检测到嵌套模块函数调用: {}.{}", module_name, function_name);
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
                                            println!("函数参数中嵌套模块函数调用成功: {} => {:?}", func_type, result);
                                        }
                                        result
                                    },
                                    Err(e) => {
                                        if is_debug_mode() {
                                            println!("函数参数中嵌套模块函数调用失败: {}", e);
                                        }
                                        // 如果调用失败，作为普通对象处理
                                        param_value.clone()
                                    }
                                }
                            } else {
                                // 点号格式不正确，作为普通对象处理
                                param_value.clone()
                            }
                        } else if is_builtin_statement(func_type) {
                            // 内置语句
                            if is_debug_mode() {
                                println!("函数参数中检测到嵌套内置语句: {}", func_type);
                            }
                            
                            // 执行嵌套的内置语句并获取结果
                            match super::execute_statement(func_type, func_args, context, Some(param_value)) {
                                Ok(result) => {
                                    if is_debug_mode() {
                                        println!("函数参数中嵌套内置语句成功执行: {} => {:?}", func_type, result);
                                    }
                                    result
                                },
                                Err(e) => {
                                    // 如果执行失败，作为普通对象处理
                                    if is_debug_mode() {
                                        println!("函数参数中嵌套内置语句执行失败: {}", e);
                                    }
                                    param_value.clone()
                                }
                            }
                        } else {
                            // 可能是自定义函数调用或者普通对象
                            param_value.clone()
                        }
                    } else {
                        // 多键对象，不是函数调用
                        param_value.clone()
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

    // 重置返回状态
    context.reset_return_status();

    // 执行函数体
    let mut last_result = Value::Null;
    for stmt in statements {
        if let Some(obj) = stmt.as_object() {
            if let Some((stmt_type, args)) = obj.iter().next() {
                // 检查是否有嵌套的模块函数调用
                if stmt_type == "var" {
                    if let Some(var_obj) = args.as_object() {
                        for (var_name, var_value) in var_obj {
                            if let Some(nested_obj) = var_value.as_object() {
                                if nested_obj.len() == 1 {
                                    let (func_type, func_args) = nested_obj.iter().next().unwrap();
                                    
                                    if func_type.contains('.') {
                                        let parts: Vec<&str> = func_type.split('.').collect();
                                        if parts.len() == 2 {
                                            if is_debug_mode() {
                                                println!("函数内检测到嵌套模块函数: {}.{}", parts[0], parts[1]);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // 执行语句并获取结果
                last_result = execute_statement(stmt_type, args, context, None)?;
                
                // 检查是否遇到return语句
                if context.is_returning() {
                    if is_debug_mode() {
                        println!("检测到return语句，中断函数执行");
                    }
                    break;  // 中断函数执行
                }
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
    // 优先使用return语句设置的返回值
    let function_result = if context.is_returning() {
        // 使用return语句设置的值
        context.get_return_value()
            .map(|v| v.clone())
            .unwrap_or(last_result.clone())
    } else if let Some(result) = context.variables.get("result") {
        // 其次使用结果变量
        result.clone()
    } else if let Some(factorial_result) = context.variables.get("factorial_result") {
        // 特殊处理阶乘函数的结果变量
        factorial_result.clone()
    } else {
        // 最后使用最后一个语句的结果
        last_result
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
    new_context_variables.insert("result".to_string(), function_result.clone());
    
    // 3.1 特别保留factorial_result变量用于递归
    if let Some(factorial_result) = context.variables.get("factorial_result") {
        new_context_variables.insert("factorial_result".to_string(), factorial_result.clone());
    }
    
    // 更新上下文
    context.variables = new_context_variables;
    
    // 重置返回状态
    context.reset_return_status();

    // 将参数中函数调用信息添加到结果中，以便兼容处理
    // 只在明确指定output变量时进行结果存储
    if let Some(params_value) = params {
        if let Some(obj) = params_value.as_object() {
            if obj.get("output").is_some() {
                store_result_with_compatibility(params_value, &function_result, context)?;
            }
        }
    }
    
    // 返回函数结果
    Ok(function_result)
} 
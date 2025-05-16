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
        "if" => return execute_if_statement(args, context),
        "while" => return execute_while_statement(args, context),
        "for" => return execute_for_statement(args, context),
        "exec" => return execute_exec_statement(args, context),
        "switch" => return execute_switch_statement(args, context),
        "try" => return execute_try_statement(args, context),
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
    matches!(name, "var" | "echo" | "concat" | "if" | "while" | "for" | "comment" | "exec" | "switch" 
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
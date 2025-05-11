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
use crate::is_debug_mode;

// 重新导出所有需要的函数
pub use basic::*;
pub use control_flow::*;
pub use function::*;
pub use array::*;
pub use object::*;
pub use regex::*;
pub use exec::*;

// 这是主要的语句执行函数，调度到各个具体的语句处理器
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
        "try" => execute_try_statement(args, context),
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
pub fn is_builtin_statement(name: &str) -> bool {
    matches!(name, "var" | "echo" | "concat" | "if" | "call" | "while" | "for" | "comment" | "exec" | "switch" 
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
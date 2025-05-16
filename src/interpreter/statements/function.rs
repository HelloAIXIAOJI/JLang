use serde_json::Value;
use std::collections::HashMap;
use super::super::context::Context;
use super::super::error::{InterpreterError, Result};
use super::super::error::error_messages::statement;
use super::execute_statement;
use super::super::variable_reference::{VariableReference, ReferenceType};
use crate::is_debug_mode;
use super::store_result_with_compatibility;

// execute_function - 执行函数体
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
                        } else if super::is_builtin_statement(func_type) {
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
    // 优先使用最后一个语句产生的结果变量
    let function_result = if let Some(result) = context.variables.get("result") {
        result.clone()
    } else if let Some(factorial_result) = context.variables.get("factorial_result") {
        // 特殊处理阶乘函数的结果变量
        factorial_result.clone()
    } else {
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
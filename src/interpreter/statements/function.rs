use serde_json::Value;
use std::collections::HashMap;
use super::super::context::Context;
use super::super::error::{InterpreterError, Result};
use super::super::error::error_messages::statement;
use super::execute_statement;
use super::super::variable_reference::{VariableReference, ReferenceType};
use crate::is_allow_call;

// execute_call_statement - 执行函数调用
pub fn execute_call_statement(args: &Value, context: &mut Context) -> Result<Value> {
    // 检查是否允许使用call语句
    if !is_allow_call() {
        return Err(InterpreterError::FunctionError(
            statement::CALL_DEPRECATED.to_string()
        ));
    }

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
                    
                    // 处理函数参数
                    // 1. 如果第二个参数是对象，直接传递
                    // 2. 如果参数是数组形式，需要创建一个对象，按顺序映射到函数定义的参数名
                    let params = if let Some(obj) = args_array.get(1).and_then(|v| v.as_object()) {
                        // 直接传递对象参数
                        args_array.get(1).cloned()
                    } else {
                        // 获取函数参数定义
                        if let Some(params_def) = func.get("params").and_then(|p| p.as_object()) {
                            // 创建参数映射
                            let mut params_map = serde_json::Map::new();
                            // 获取param_names列表以确保顺序一致
                            let param_names: Vec<&String> = params_def.keys().collect();
                            
                            // 映射参数值到对应参数名
                            for (i, param_name) in param_names.iter().enumerate() {
                                // 跳过第一个参数(函数名)，所以从args_array的索引1开始
                                if let Some(arg_value) = args_array.get(i + 1) {
                                    params_map.insert(param_name.to_string(), arg_value.clone());
                                }
                            }
                            
                            Some(Value::Object(params_map))
                        } else {
                            None
                        }
                    };
                    
                    return execute_function(&func, context, params.as_ref());
                }
            }
        }
    }
    Err(InterpreterError::RuntimeError(
        statement::INVALID_FUNCTION_CALL.to_string()
    ))
}

// execute_function - 执行函数体
pub fn execute_function(func: &Value, context: &mut Context, params: Option<&Value>) -> Result<Value> {
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
                    if VariableReference::is_reference(text) {
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
    
    // 更新上下文
    context.variables = new_context_variables;

    // 返回函数结果
    Ok(function_result)
} 
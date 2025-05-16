use serde_json::Value;
use super::super::context::Context;
use super::super::error::{InterpreterError, Result};
use super::super::error::error_messages::statement::{self, control_flow, switch, try_catch};
use super::basic::evaluate_condition;
use super::get_number_value;
use super::execute_statement;
use super::super::variable_reference::{VariableReference, ReferenceType};
use super::store_result_with_compatibility;

// execute_if_statement - 执行if条件语句
pub fn execute_if_statement(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(obj) = args.as_object() {
        if let (Some(condition), Some(then_block)) = (obj.get("condition"), obj.get("then")) {
            let condition_result = evaluate_condition(condition, context);
            let block = if condition_result {
                then_block
            } else {
                obj.get("else").unwrap_or(then_block)
            };

            let mut last_result = Value::Null;
            if let Some(statements) = block.as_array() {
                for stmt in statements {
                    if let Some(obj) = stmt.as_object() {
                        if let Some((stmt_type, args)) = obj.iter().next() {
                            last_result = execute_statement(stmt_type, args, context, None)?;
                        }
                    }
                }
            }
            
            // 存储结果并返回
            store_result_with_compatibility(args, &last_result, context)?;
            return Ok(last_result);
        }
    }
    // 如果条件语句无效，返回null
    let result = Value::Null;
    store_result_with_compatibility(args, &result, context)?;
    Ok(result)
}

// execute_while_statement - 执行while循环语句
pub fn execute_while_statement(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(obj) = args.as_object() {
        if let (Some(condition), Some(body)) = (obj.get("condition"), obj.get("body")) {
            let mut last_result = Value::Null;
            let mut iteration_count = 0;
            
            while evaluate_condition(condition, context) {
                if let Some(statements) = body.as_array() {
                    for stmt in statements {
                        if let Some(obj) = stmt.as_object() {
                            if let Some((stmt_type, args)) = obj.iter().next() {
                                last_result = execute_statement(stmt_type, args, context, None)?;
                            }
                        }
                    }
                }
                iteration_count += 1;
            }
            
            // 构造结果对象
            let mut result_obj = serde_json::Map::new();
            result_obj.insert("iterations".to_string(), Value::Number(serde_json::Number::from(iteration_count)));
            result_obj.insert("last_result".to_string(), last_result);
            let result = Value::Object(result_obj);
            
            // 存储结果并返回
            store_result_with_compatibility(args, &result, context)?;
            Ok(result)
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

// execute_for_statement - 执行for循环语句，支持多种循环方式
pub fn execute_for_statement(args: &Value, context: &mut Context) -> Result<Value> {
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
                    if VariableReference::is_reference(array_ref) {
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
                
                let mut last_result = Value::Null;
                let mut iteration_count = 0;
                
                // 遍历数组的每个元素
                for item in array {
                    // 设置循环变量
                    context.set_variable(var_name.to_string(), item.clone())?;
                    
                    // 执行循环体
                    if let Some(statements) = body.as_array() {
                        for stmt in statements {
                            if let Some(obj) = stmt.as_object() {
                                if let Some((stmt_type, args)) = obj.iter().next() {
                                    last_result = execute_statement(stmt_type, args, context, None)?;
                                }
                            }
                        }
                    } else {
                        return Err(InterpreterError::RuntimeError(
                            "杂鱼~循环体必须是语句数组".to_string()
                        ));
                    }
                    
                    iteration_count += 1;
                }
                
                // 构造结果对象
                let mut result_obj = serde_json::Map::new();
                result_obj.insert("iterations".to_string(), Value::Number(serde_json::Number::from(iteration_count)));
                result_obj.insert("last_result".to_string(), last_result);
                let result = Value::Object(result_obj);
                
                // 存储结果并返回
                store_result_with_compatibility(args, &result, context)?;
                return Ok(result);
            } else {
                return Err(InterpreterError::RuntimeError(
                    control_flow::FOR_MISSING_FIELDS.to_string()
                ));
            }
        }
        // 支持两种for循环语法
        else if let Some(range) = obj.get("range") {
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
                    
                    let mut current = start;
                    let mut last_result = Value::Null;
                    let mut iteration_count = 0;
                    
                    // 修改循环条件，包含等于情况
                    while (step > 0.0 && current <= end) || (step < 0.0 && current >= end) {
                        context.set_variable(var_name.to_string(), Value::Number(serde_json::Number::from_f64(current).unwrap()))?;
                        
                        if let Some(statements) = body.as_array() {
                            for stmt in statements {
                                if let Some(obj) = stmt.as_object() {
                                    if let Some((stmt_type, args)) = obj.iter().next() {
                                        last_result = execute_statement(stmt_type, args, context, None)?;
                                    }
                                }
                            }
                        }
                        
                        current += step;
                        iteration_count += 1;
                    }
                    
                    // 构造结果对象
                    let mut result_obj = serde_json::Map::new();
                    result_obj.insert("iterations".to_string(), Value::Number(serde_json::Number::from(iteration_count)));
                    result_obj.insert("last_result".to_string(), last_result);
                    let result = Value::Object(result_obj);
                    
                    // 存储结果并返回
                    store_result_with_compatibility(args, &result, context)?;
                    return Ok(result);
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
                
                let mut current = start;
                let mut last_result = Value::Null;
                let mut iteration_count = 0;
                
                // 修改循环条件，包含等于情况
                while (step > 0.0 && current <= end) || (step < 0.0 && current >= end) {
                    context.set_variable(var_name.to_string(), Value::Number(serde_json::Number::from_f64(current).unwrap()))?;
                    
                    if let Some(statements) = body.as_array() {
                        for stmt in statements {
                            if let Some(obj) = stmt.as_object() {
                                if let Some((stmt_type, args)) = obj.iter().next() {
                                    last_result = execute_statement(stmt_type, args, context, None)?;
                                }
                            }
                        }
                    }
                    
                    current += step;
                    iteration_count += 1;
                }
                
                // 构造结果对象
                let mut result_obj = serde_json::Map::new();
                result_obj.insert("iterations".to_string(), Value::Number(serde_json::Number::from(iteration_count)));
                result_obj.insert("last_result".to_string(), last_result);
                let result = Value::Object(result_obj);
                
                // 存储结果并返回
                store_result_with_compatibility(args, &result, context)?;
                Ok(result)
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

// execute_switch_statement - 执行switch语句
pub fn execute_switch_statement(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(obj) = args.as_object() {
        if let (Some(value_expr), Some(cases)) = (obj.get("value"), obj.get("cases")) {
            // 获取要匹配的值
            let value = context.resolve_value(value_expr);
            let value_num_result = value.parse::<f64>();
            
            let mut executed = false;
            let mut last_result = Value::Null;
            
            if let Some(cases_array) = cases.as_array() {
                // 遍历所有case
                for case in cases_array {
                    if let Some(case_obj) = case.as_object() {
                        if let (Some(case_value), Some(statements)) = (case_obj.get("case"), case_obj.get("do")) {
                            let case_val = context.resolve_value(case_value);
                            let case_num = case_val.parse::<f64>();
                            
                            // 改为使用value_num_result的引用，避免所有权移动
                            let is_match = match (&value_num_result, &case_num) {
                                (Ok(v), Ok(c)) => (v - c).abs() < std::f64::EPSILON,
                                _ => value == case_val
                            };
                            
                            if is_match {
                                executed = true;
                                // 执行匹配的case
                                if let Some(statements_array) = statements.as_array() {
                                    for stmt in statements_array {
                                        if let Some(obj) = stmt.as_object() {
                                            if let Some((stmt_type, args)) = obj.iter().next() {
                                                last_result = execute_statement(stmt_type, args, context, None)?;
                                            }
                                        }
                                    }
                                }
                                    break;
                            }
                        }
                    }
                }
                
                // 如果没有匹配的case，尝试执行default
                if !executed {
                    if let Some(default_block) = obj.get("default") {
                        if let Some(statements) = default_block.as_array() {
                        for stmt in statements {
                            if let Some(obj) = stmt.as_object() {
                                if let Some((stmt_type, args)) = obj.iter().next() {
                                        last_result = execute_statement(stmt_type, args, context, None)?;
                                    }
                                }
                            }
                        }
                    }
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    switch::CASES_NOT_ARRAY.to_string()
                ));
            }
            
            // 存储结果并返回
            store_result_with_compatibility(args, &last_result, context)?;
            Ok(last_result)
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

// execute_try_statement - 执行try-catch语句
pub fn execute_try_statement(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(obj) = args.as_object() {
        if let (Some(try_block), Some(catch_block)) = (obj.get("try"), obj.get("catch")) {
            let error_var = obj.get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("error");
        
        // 执行try块
            let mut result = Value::Null;
            let mut had_error = false;
        
            if let Some(statements) = try_block.as_array() {
                for stmt in statements {
            if let Some(obj) = stmt.as_object() {
                if let Some((stmt_type, args)) = obj.iter().next() {
                    match execute_statement(stmt_type, args, context, None) {
                                Ok(res) => result = res,
                        Err(e) => {
                                    // 捕获错误，存储错误信息
                                    let error_msg = match &e {
                                        InterpreterError::RuntimeError(msg) => msg.clone(),
                                        InterpreterError::FunctionError(msg) => msg.clone(),
                                        InterpreterError::ModuleError(msg) => msg.clone(),
                                        InterpreterError::InvalidProgramStructure(msg) => msg.clone(),
                                        InterpreterError::VariableError(msg) => msg.clone(),
                                    };
                                    
                                    // 设置错误变量
                                    context.set_variable(error_var.to_string(), Value::String(error_msg))?;
                                    had_error = true;
                            break;
                        }
                    }
                }
            }
        }
            }
            
            // 如果有错误，执行catch块
            if had_error {
                if let Some(statements) = catch_block.as_array() {
                    for stmt in statements {
                if let Some(obj) = stmt.as_object() {
                    if let Some((stmt_type, args)) = obj.iter().next() {
                                result = execute_statement(stmt_type, args, context, None)?;
                            }
                        }
                    }
                }
            }
            
            // 构造结果对象
            let mut result_obj = serde_json::Map::new();
            result_obj.insert("had_error".to_string(), Value::Bool(had_error));
            result_obj.insert("result".to_string(), result.clone());
            let final_result = Value::Object(result_obj);
            
            // 存储结果并返回
            store_result_with_compatibility(args, &final_result, context)?;
            Ok(final_result)
        } else {
            Err(InterpreterError::RuntimeError(
                try_catch::MISSING_FIELDS.to_string()
            ))
        }
    } else {
        Err(InterpreterError::RuntimeError(
            try_catch::ARGS_NOT_OBJ.to_string()
        ))
    }
} 
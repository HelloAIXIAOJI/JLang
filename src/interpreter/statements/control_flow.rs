use serde_json::Value;
use super::super::context::Context;
use super::super::error::{InterpreterError, Result};
use super::super::error::error_messages::statement::{self, control_flow, switch, try_catch};
use super::basic::evaluate_condition;
use super::get_number_value;
use super::execute_statement;
use super::super::variable_reference::{VariableReference, ReferenceType};

// execute_if_statement - 执行if条件语句
pub fn execute_if_statement(args: &Value, context: &mut Context) -> Result<()> {
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

// execute_while_statement - 执行while循环语句
pub fn execute_while_statement(args: &Value, context: &mut Context) -> Result<()> {
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

// execute_for_statement - 执行for循环语句，支持多种循环方式
pub fn execute_for_statement(args: &Value, context: &mut Context) -> Result<()> {
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

// execute_switch_statement - 执行switch分支语句
pub fn execute_switch_statement(args: &Value, context: &mut Context) -> Result<()> {
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

// execute_try_statement - 执行try-catch错误处理语句
pub fn execute_try_statement(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(obj) = args.as_object() {
        // 检查必要字段
        let try_body = obj.get("try").ok_or_else(|| {
            InterpreterError::RuntimeError(try_catch::MISSING_FIELDS.to_string())
        })?;
        
        let catch_body = obj.get("catch").ok_or_else(|| {
            InterpreterError::RuntimeError(try_catch::MISSING_FIELDS.to_string())
        })?;
        
        // 验证try块是数组
        let try_statements = try_body.as_array().ok_or_else(|| {
            InterpreterError::RuntimeError(try_catch::TRY_BODY_NOT_ARRAY.to_string())
        })?;
        
        // 验证catch块是数组
        let catch_statements = catch_body.as_array().ok_or_else(|| {
            InterpreterError::RuntimeError(try_catch::CATCH_BODY_NOT_ARRAY.to_string())
        })?;
        
        // 获取错误变量名（可选）
        let error_var = if let Some(var) = obj.get("error_var") {
            if let Some(var_name) = var.as_str() {
                Some(var_name.to_string())
            } else {
                return Err(InterpreterError::RuntimeError(
                    try_catch::ERROR_VAR_NOT_STRING.to_string()
                ));
            }
        } else {
            None
        };
        
        // 执行try块
        let mut error_caught = false;
        let mut error_message = String::new();
        
        // 尝试执行try块中的语句
        for stmt in try_statements {
            if let Some(obj) = stmt.as_object() {
                if let Some((stmt_type, args)) = obj.iter().next() {
                    match execute_statement(stmt_type, args, context) {
                        Ok(_) => {},
                        Err(e) => {
                            // 捕获错误
                            error_caught = true;
                            error_message = format!("{}", e);
                            break;
                        }
                    }
                }
            }
        }
        
        // 如果捕获到错误，执行catch块
        if error_caught {
            // 如果指定了错误变量名，将错误信息存储到该变量
            if let Some(var_name) = error_var {
                context.set_variable(var_name, Value::String(error_message))?;
            }
            
            // 执行catch块
            for stmt in catch_statements {
                if let Some(obj) = stmt.as_object() {
                    if let Some((stmt_type, args)) = obj.iter().next() {
                        execute_statement(stmt_type, args, context)?;
                    }
                }
            }
        }
        
        // 检查是否有finally块
        if let Some(finally_body) = obj.get("finally") {
            if let Some(finally_statements) = finally_body.as_array() {
                // 无论try或catch块的执行结果如何，都执行finally块
                for stmt in finally_statements {
                    if let Some(obj) = stmt.as_object() {
                        if let Some((stmt_type, args)) = obj.iter().next() {
                            execute_statement(stmt_type, args, context)?;
                        }
                    }
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    try_catch::FINALLY_BODY_NOT_ARRAY.to_string()
                ));
            }
        }
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            try_catch::ARGS_NOT_OBJ.to_string()
        ))
    }
} 
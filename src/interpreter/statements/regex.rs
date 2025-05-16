use serde_json::Value;
use regex::Regex;
use super::super::context::Context;
use super::super::error::{InterpreterError, Result};
use super::store_result_with_compatibility;

// execute_regex_match - 正则表达式匹配
pub fn execute_regex_match(args: &Value, context: &mut Context) -> Result<Value> {
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
        store_result_with_compatibility(args, &result, context)?;
        Ok(result)
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'regex.match' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// execute_regex_test - 正则表达式测试
pub fn execute_regex_test(args: &Value, context: &mut Context) -> Result<Value> {
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
        let result = Value::Bool(is_match);
        
        // 存储结果
        store_result_with_compatibility(args, &result, context)?;
        Ok(result)
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'regex.test' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// execute_regex_replace - 正则表达式替换
pub fn execute_regex_replace(args: &Value, context: &mut Context) -> Result<Value> {
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
        let replaced = regex.replace_all(&text, replacement.as_str()).to_string();
        let result = Value::String(replaced);
        
        // 存储结果
        store_result_with_compatibility(args, &result, context)?;
        Ok(result)
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'regex.replace' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// execute_regex_split - 正则表达式分割
pub fn execute_regex_split(args: &Value, context: &mut Context) -> Result<Value> {
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
        
        let result = Value::Array(parts);
        
        // 存储结果
        store_result_with_compatibility(args, &result, context)?;
        Ok(result)
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'regex.split' 语句的参数必须是一个数组".to_string()
        ))
    }
} 
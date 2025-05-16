use serde_json::Value;
use std::process::Command;
use super::super::context::Context;
use super::super::error::{InterpreterError, Result};
use super::super::error::error_messages::statement::exec;
use super::store_result_with_compatibility;

// execute_exec_statement - 执行系统命令
pub fn execute_exec_statement(args: &Value, context: &mut Context) -> Result<Value> {
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
                
                let result = Value::Object(result_obj);
                
                // 保存结果
                context.set_variable(output_var.to_string(), result.clone())?;
                
                // 兼容性处理 - 如果output_var不是"result"，则同时存储在"result"变量中
                store_result_with_compatibility(args, &result, context)?;
                
                Ok(result)
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
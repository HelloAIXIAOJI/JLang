use std::fs;
use serde_json::Value;
use crate::interpreter::context::Context;
use crate::interpreter::error::{InterpreterError, Result};
use super::Module;

pub struct JlModule {
    name: String,
    functions: Vec<(String, Value)>,
}

impl JlModule {
    pub fn new(name: &str, file_path: &str) -> Result<Self> {
        let content = fs::read_to_string(file_path)
            .map_err(|e| InterpreterError::ModuleError(format!("无法读取文件 '{}': {}", file_path, e)))?;
        
        let program: Value = serde_json::from_str(&content)
            .map_err(|e| InterpreterError::ModuleError(format!("无效的 JSON 格式: {}", e)))?;

        let mut functions = Vec::new();
        if let Some(program_obj) = program.get("program") {
            if let Some(obj) = program_obj.as_object() {
                for (func_name, func_def) in obj {
                    functions.push((func_name.clone(), func_def.clone()));
                }
            }
        }

        Ok(JlModule {
            name: name.to_string(),
            functions,
        })
    }
}

impl Module for JlModule {
    fn get_name(&self) -> &'static str {
        Box::leak(self.name.clone().into_boxed_str())
    }

    fn get_functions(&self) -> Vec<(&'static str, fn(&[Value], &mut Context) -> Value)> {
        Vec::new() // 我们不需要实现这个，因为我们使用自定义的函数调用机制
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl JlModule {
    pub fn get_function(&self, name: &str) -> Option<&Value> {
        self.functions.iter()
            .find(|(func_name, _)| func_name == name)
            .map(|(_, func_def)| func_def)
    }
} 
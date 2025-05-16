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
                    if crate::is_debug_mode() {
                        println!("在模块 '{}' 中找到函数: {}", name, func_name);
                    }
                    functions.push((func_name.clone(), func_def.clone()));
                }
            } else if crate::is_debug_mode() {
                println!("警告: 模块 '{}' 中的 'program' 不是对象", name);
            }
        } else if crate::is_debug_mode() {
            println!("警告: 模块 '{}' 中没有 'program' 字段", name);
        }

        if crate::is_debug_mode() {
            println!("模块 '{}' 中共加载了 {} 个函数", name, functions.len());
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
        if crate::is_debug_mode() {
            println!("尝试在模块 '{}' 中查找函数: '{}'", self.name, name);
            println!("可用函数: {}", self.functions.iter()
                .map(|(fname, _)| fname.clone())
                .collect::<Vec<_>>()
                .join(", "));
        }
        
        let result = self.functions.iter()
            .find(|(func_name, _)| func_name == name)
            .map(|(_, func_def)| func_def);
            
        if crate::is_debug_mode() {
            if result.is_some() {
                println!("在模块 '{}' 中找到函数: '{}'", self.name, name);
            } else {
                println!("在模块 '{}' 中未找到函数: '{}'", self.name, name);
            }
        }
        
        result
    }
} 
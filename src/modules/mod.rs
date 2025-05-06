pub mod io;
pub mod math;
pub mod jl_module;

use std::path::Path;
use serde_json::Value;
use crate::interpreter::context::Context;
use crate::interpreter::error::{InterpreterError, Result};

pub trait Module: std::any::Any {
    fn get_name(&self) -> &'static str;
    fn get_functions(&self) -> Vec<(&'static str, fn(&[Value], &mut Context) -> Value)>;
    fn as_any(&self) -> &dyn std::any::Any;
}

pub fn get_module(name: &str) -> Option<Box<dyn Module>> {
    match name {
        "io" => Some(Box::new(io::IoModule::new())),
        "math" => Some(Box::new(math::MathModule::new())),
        _ => {
            // 尝试加载 .jl 文件
            // 1. 首先检查当前目录
            let file_path = format!("{}.jl", name);
            if Path::new(&file_path).exists() {
                match jl_module::JlModule::new(name, &file_path) {
                    Ok(module) => return Some(Box::new(module)),
                    Err(_) => {}
                }
            }
            
            // 2. 检查 examples 目录
            let examples_path = format!("examples/{}.jl", name);
            if Path::new(&examples_path).exists() {
                match jl_module::JlModule::new(name, &examples_path) {
                    Ok(module) => return Some(Box::new(module)),
                    Err(_) => {}
                }
            }
            
            None
        }
    }
} 
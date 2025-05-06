use std::fs;
use serde_json::Value;
use crate::interpreter::context::Context;
use super::Module;

pub struct IoModule;

impl IoModule {
    pub fn new() -> Self {
        IoModule
    }

    fn read_file(args: &[Value], context: &mut Context) -> Value {
        if let Some(path) = args.get(0) {
            let resolved_path = context.resolve_value(path);
            match fs::read_to_string(resolved_path) {
                Ok(content) => Value::String(content),
                Err(e) => Value::String(format!("Error: {}", e))
            }
        } else {
            Value::String("Error: No file path provided".to_string())
        }
    }

    fn write_file(args: &[Value], context: &mut Context) -> Value {
        if let (Some(path), Some(content)) = (
            args.get(0),
            args.get(1)
        ) {
            let resolved_path = context.resolve_value(path);
            let resolved_content = context.resolve_value(content);
            match fs::write(resolved_path, resolved_content) {
                Ok(_) => Value::String("File written successfully".to_string()),
                Err(e) => Value::String(format!("Error: {}", e))
            }
        } else {
            Value::String("Error: Invalid arguments".to_string())
        }
    }

    fn input(args: &[Value], _context: &mut Context) -> Value {
        let prompt = args.get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("");
        
        print!("{}", prompt);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                // 尝试将输入解析为数字
                if let Ok(num) = trimmed.parse::<f64>() {
                    Value::Number(serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from_f64(0.0).unwrap()))
                } else {
                    Value::String(trimmed.to_string())
                }
            },
            Err(e) => Value::String(format!("Error: {}", e))
        }
    }
}

impl Module for IoModule {
    fn get_name(&self) -> &'static str {
        "io"
    }

    fn get_functions(&self) -> Vec<(&'static str, fn(&[Value], &mut Context) -> Value)> {
        vec![
            ("read_file", Self::read_file),
            ("write_file", Self::write_file),
            ("input", Self::input)
        ]
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
} 
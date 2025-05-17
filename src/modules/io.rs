use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;
use serde_json::{Value, json};
use crate::interpreter::context::Context;
use super::Module;

pub struct IoModule;

impl IoModule {
    pub fn new() -> Self {
        IoModule
    }

    fn echo(args: &[Value], _context: &mut Context) -> Value {
        let mut result = String::new();
        for arg in args {
            result.push_str(&arg.to_string());
        }
        print!("{}", result);
        Value::String(result)
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
    
    // 新增: 向文件追加内容
    fn append_file(args: &[Value], context: &mut Context) -> Value {
        if let (Some(path), Some(content)) = (
            args.get(0),
            args.get(1)
        ) {
            let resolved_path = context.resolve_value(path);
            let resolved_content = context.resolve_value(content);
            
            // 使用OpenOptions来追加内容
            match OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(resolved_path) {
                Ok(mut file) => {
                    match file.write_all(resolved_content.to_string().as_bytes()) {
                        Ok(_) => Value::String("Content appended successfully".to_string()),
                        Err(e) => Value::String(format!("Error appending to file: {}", e))
                    }
                },
                Err(e) => Value::String(format!("Error opening file for append: {}", e))
            }
        } else {
            Value::String("Error: Invalid arguments for append_file".to_string())
        }
    }
    
    // 新增: 检查文件是否存在
    fn file_exists(args: &[Value], context: &mut Context) -> Value {
        if let Some(path) = args.get(0) {
            let resolved_path = context.resolve_value(path);
            let path_str = resolved_path.to_string();
            let exists = Path::new(&path_str).exists();
            Value::Bool(exists)
        } else {
            Value::String("Error: No file path provided for file_exists".to_string())
        }
    }
    
    // 新增: 删除文件
    fn delete_file(args: &[Value], context: &mut Context) -> Value {
        if let Some(path) = args.get(0) {
            let resolved_path = context.resolve_value(path);
            let path_str = resolved_path.to_string();
            
            if !Path::new(&path_str).exists() {
                return Value::String(format!("Error: File '{}' does not exist", path_str));
            }
            
            match fs::remove_file(&path_str) {
                Ok(_) => Value::String(format!("File '{}' deleted successfully", path_str)),
                Err(e) => Value::String(format!("Error deleting file: {}", e))
            }
        } else {
            Value::String("Error: No file path provided for delete_file".to_string())
        }
    }
    
    // 新增: 列出目录内容
    fn list_dir(args: &[Value], context: &mut Context) -> Value {
        if let Some(path) = args.get(0) {
            let resolved_path = context.resolve_value(path);
            let path_str = resolved_path.to_string();
            let dir_path = if path_str.is_empty() { "." } else { &path_str };
            
            match fs::read_dir(dir_path) {
                Ok(entries) => {
                    let mut files = Vec::new();
                    let mut dirs = Vec::new();
                    
                    for entry in entries {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            let name = path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("")
                                .to_string();
                                
                            if path.is_dir() {
                                dirs.push(Value::String(name));
                            } else {
                                files.push(Value::String(name));
                            }
                        }
                    }
                    
                    let mut result = serde_json::Map::new();
                    result.insert("files".to_string(), Value::Array(files));
                    result.insert("directories".to_string(), Value::Array(dirs));
                    
                    Value::Object(result)
                },
                Err(e) => Value::String(format!("Error reading directory: {}", e))
            }
        } else {
            Value::String("Error: No directory path provided for list_dir".to_string())
        }
    }
    
    // 新增: 专门用于获取数字输入，带验证
    fn input_number(args: &[Value], _context: &mut Context) -> Value {
        let prompt = args.get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("请输入一个数字: ");
        
        let min = args.get(1)
            .and_then(|v| v.as_f64())
            .unwrap_or(f64::NEG_INFINITY);
            
        let max = args.get(2)
            .and_then(|v| v.as_f64())
            .unwrap_or(f64::INFINITY);
        
        loop {
            print!("{}", prompt);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let trimmed = input.trim();
                    if let Ok(num) = trimmed.parse::<f64>() {
                        if num >= min && num <= max {
                            return Value::Number(serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from_f64(0.0).unwrap()));
                        } else {
                            println!("请输入在 {} 到 {} 范围内的数字", min, max);
                        }
                    } else {
                        println!("无效的数字，请重新输入");
                    }
                },
                Err(e) => {
                    println!("读取输入出错: {}", e);
                }
            }
        }
    }
    
    // 新增: 带默认值的输入
    fn input_with_default(args: &[Value], _context: &mut Context) -> Value {
        let prompt = args.get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("");
            
        let default_value = args.get(1)
            .cloned()
            .unwrap_or(Value::String("".to_string()));
            
        let default_display = match &default_value {
            Value::String(s) => s.clone(),
            _ => default_value.to_string()
        };
        
        print!("{} [{}]: ", prompt, default_display);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    // 返回默认值
                    default_value
                } else {
                    // 尝试将输入解析为数字
                    if let Ok(num) = trimmed.parse::<f64>() {
                        Value::Number(serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from_f64(0.0).unwrap()))
                    } else {
                        Value::String(trimmed.to_string())
                    }
                }
            },
            Err(e) => Value::String(format!("Error: {}", e))
        }
    }
    
    // 新增: 获取用户确认(y/n)
    fn confirm(args: &[Value], _context: &mut Context) -> Value {
        let prompt = args.get(0)
            .and_then(|v| v.as_str())
            .unwrap_or("确认? (y/n): ");
            
        let default_yes = args.get(1)
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
            
        loop {
            print!("{}", prompt);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            
            let mut input = String::new();
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let trimmed = input.trim().to_lowercase();
                    if trimmed.is_empty() {
                        return Value::Bool(default_yes);
                    } else if trimmed == "y" || trimmed == "yes" || trimmed == "是" {
                        return Value::Bool(true);
                    } else if trimmed == "n" || trimmed == "no" || trimmed == "否" {
                        return Value::Bool(false);
                    } else {
                        println!("请输入 'y' 或 'n'");
                    }
                },
                Err(e) => {
                    println!("读取输入出错: {}", e);
                }
            }
        }
    }
    
    // 新增: 读取JSON文件并解析
    fn read_json(args: &[Value], context: &mut Context) -> Value {
        if let Some(path) = args.get(0) {
            let resolved_path = context.resolve_value(path);
            match fs::read_to_string(resolved_path) {
                Ok(content) => {
                    match serde_json::from_str::<Value>(&content) {
                        Ok(json_value) => json_value,
                        Err(e) => Value::String(format!("Error parsing JSON: {}", e))
                    }
                },
                Err(e) => Value::String(format!("Error reading file: {}", e))
            }
        } else {
            Value::String("Error: No file path provided for read_json".to_string())
        }
    }
    
    // 新增: 将对象写入为JSON文件
    fn write_json(args: &[Value], context: &mut Context) -> Value {
        if let (Some(path), Some(data)) = (
            args.get(0),
            args.get(1)
        ) {
            let resolved_path = context.resolve_value(path);
            let resolved_data = context.resolve_value(data);
            
            // 是否格式化输出
            let pretty = args.get(2)
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
                
            let json_string = if pretty {
                match serde_json::to_string_pretty(&resolved_data) {
                    Ok(s) => s,
                    Err(e) => return Value::String(format!("Error formatting JSON: {}", e))
                }
            } else {
                match serde_json::to_string(&resolved_data) {
                    Ok(s) => s,
                    Err(e) => return Value::String(format!("Error serializing JSON: {}", e))
                }
            };
            
            match fs::write(resolved_path, json_string) {
                Ok(_) => Value::String("JSON written successfully".to_string()),
                Err(e) => Value::String(format!("Error writing JSON file: {}", e))
            }
        } else {
            Value::String("Error: Invalid arguments for write_json".to_string())
        }
    }

    fn json_get(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 2 {
            return Value::String("Error: json_get需要至少两个参数: JSON对象和属性路径".to_string());
        }
        
        // 获取JSON对象
        let json_var = match &args[0] {
            Value::String(s) if s.starts_with("@var.") => {
                let var_name = &s[5..]; // 跳过"@var."前缀
                
                // 直接从变量表中获取值
                match context.variables.get(var_name) {
                    Some(val) => val.clone(),
                    None => return Value::String(format!("Error: 变量'{}' 不存在", var_name))
                }
            },
            _ => return Value::String("Error: 第一个参数必须是@var类型的变量引用".to_string())
        };
        
        // 确保是JSON对象或数组
        if !json_var.is_object() && !json_var.is_array() {
            return Value::String("Error: 输入不是有效的JSON对象或数组".to_string());
        }
        
        // 获取属性路径
        let property = match context.resolve_value(&args[1]) {
            ref s => s.clone(),
        };
        
        // 只在调试模式下输出
        if crate::is_debug_mode() {
            println!("DEBUG: 属性路径: {}", property);
        }
        
        // 分割属性路径，例如 "user.profile.name" -> ["user", "profile", "name"]
        let path_parts: Vec<&str> = property.split('.').collect();
        
        // 递归获取嵌套属性
        let mut current = json_var.clone();
        for part in path_parts {
            if let Some(obj) = current.as_object() {
                if let Some(value) = obj.get(part) {
                    current = value.clone();
                } else {
                    return Value::Null;
                }
            } else if let Some(arr) = current.as_array() {
                if let Ok(index) = part.parse::<usize>() {
                    if index < arr.len() {
                        current = arr[index].clone();
                    } else {
                        return Value::Null;
                    }
                } else {
                    return Value::String(format!("Error: 无法将'{}]'解析为数组索引", part));
                }
            } else {
                return Value::Null;
            }
        }
        
        current
    }
    
    fn json_set(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 3 {
            return Value::String("Error: json_set需要三个参数: JSON对象, 属性路径和新值".to_string());
        }
        
        // 获取JSON对象变量名
        let var_name = match &args[0] {
            Value::String(s) if s.starts_with("@var.") => {
                &s[5..] // 跳过"@var."前缀
            },
            _ => return Value::String("Error: 第一个参数必须是@var类型的变量引用".to_string())
        };
        
        // 直接从变量表中获取值
        let mut json_obj = match context.variables.get(var_name) {
            Some(val) => val.clone(),
            None => return Value::String(format!("Error: 变量'{}' 不存在", var_name))
        };
        
        // 确保是JSON对象或数组
        if !json_obj.is_object() && !json_obj.is_array() {
            return Value::String("Error: 输入不是有效的JSON对象或数组".to_string());
        }
        
        // 获取属性路径
        let property = match context.resolve_value(&args[1]) {
            ref s => s.clone(),
        };
        
        // 解析新值
        let new_value = match args[2] {
            Value::String(ref s) if s.starts_with("@") => {
                // 这是一个变量引用，我们需要从上下文中获取它
                if let Some(var_ref) = context.get_value(s) {
                    var_ref
                } else {
                    // 如果没有找到变量，尝试使用resolve_value
                    serde_json::from_str(&context.resolve_value(&args[2])).unwrap_or(args[2].clone())
                }
            },
            _ => {
                // 尝试把原始值转为JSON结构
                let val_str = context.resolve_value(&args[2]);
                serde_json::from_str(&val_str).unwrap_or(args[2].clone())
            }
        };
        
        // 分割属性路径
        let path_parts: Vec<&str> = property.split('.').collect();
        
        if path_parts.is_empty() {
            return Value::String("Error: 属性路径不能为空".to_string());
        }
        
        // 递归设置嵌套属性
        let set_result = Self::set_nested_property(&mut json_obj, &path_parts, &new_value);
        if let Err(e) = set_result {
            return Value::String(format!("Error: {}", e));
        }
        
        // 更新变量
        match context.set_variable(var_name.to_string(), json_obj.clone()) {
            Ok(_) => json_obj,
            Err(e) => Value::String(format!("Error: {}", e))
        }
    }
    
    // 辅助函数：设置嵌套属性
    fn set_nested_property(obj: &mut Value, path_parts: &[&str], value: &Value) -> Result<(), String> {
        if path_parts.is_empty() {
            return Ok(());
        }
        
        let mut current = obj;
        let path_len = path_parts.len();
        
        for (i, &part) in path_parts.iter().enumerate() {
            if i == path_len - 1 {
                // 最后一个部分，设置值
                if let Some(obj_map) = current.as_object_mut() {
                    // 只在调试模式下输出
                    if crate::is_debug_mode() {
                        println!("DEBUG: 设置对象属性 '{}' 的值", part);
                    }
                    obj_map.insert(part.to_string(), value.clone());
                    return Ok(());
                } else if let Some(arr) = current.as_array_mut() {
                    if let Ok(index) = part.parse::<usize>() {
                        // 只在调试模式下输出
                        if crate::is_debug_mode() {
                            println!("DEBUG: 设置数组索引 {} 的值", index);
                        }
                        while arr.len() <= index {
                            arr.push(Value::Null);
                        }
                        arr[index] = value.clone();
                        return Ok(());
                    } else {
                        return Err(format!("无法将'{}]'解析为数组索引", part));
                    }
                } else {
                    return Err("无法设置非对象或数组的属性".to_string());
                }
            }
            
            // 中间路径处理 - 需要区分对象和数组情况
            let is_obj = current.is_object();
            let is_arr = current.is_array();
            
            if is_obj {
                let obj_map = current.as_object_mut().unwrap();
                if !obj_map.contains_key(part) {
                    // 只在调试模式下输出
                    if crate::is_debug_mode() {
                        println!("DEBUG: 创建新的嵌套对象属性 '{}'", part);
                    }
                    obj_map.insert(part.to_string(), json!({}));
                }
                let next_obj = obj_map.get_mut(part).unwrap();
                current = next_obj;
            } else if is_arr {
                let arr = current.as_array_mut().unwrap();
                if let Ok(index) = part.parse::<usize>() {
                    // 只在调试模式下输出
                    if crate::is_debug_mode() {
                        println!("DEBUG: 访问或创建数组索引 {}", index);
                    }
                    while arr.len() <= index {
                        arr.push(json!({}));
                    }
                    let next_obj = &mut arr[index];
                    current = next_obj;
                } else {
                    return Err(format!("无法将'{}]'解析为数组索引", part));
                }
            } else {
                return Err("无法访问非对象或数组的属性".to_string());
            }
        }
        
        Ok(())
    }
}

impl Module for IoModule {
    fn get_name(&self) -> &'static str {
        "io"
    }
    
    fn get_functions(&self) -> Vec<(&'static str, Box<dyn Fn(&[Value], &mut Context) -> Value + Send + Sync + 'static>)> {
        vec![
            ("echo", Box::new(Self::echo)),
            ("input", Box::new(Self::input)),
            ("input_with_default", Box::new(Self::input_with_default)),
            ("input_number", Box::new(Self::input_number)),
            ("confirm", Box::new(Self::confirm)),
            ("read_file", Box::new(Self::read_file)),
            ("write_file", Box::new(Self::write_file)),
            ("append_file", Box::new(Self::append_file)),
            ("file_exists", Box::new(Self::file_exists)),
            ("delete_file", Box::new(Self::delete_file)),
            ("list_dir", Box::new(Self::list_dir)),
            ("read_json", Box::new(Self::read_json)),
            ("write_json", Box::new(Self::write_json)),
            ("json_get", Box::new(Self::json_get)),
            ("json_set", Box::new(Self::json_set)),
        ]
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
} 
use std::fs;
use std::collections::HashMap;
use serde_json::Value;
use mlua::{Lua, prelude::LuaFunction, prelude::LuaTable, Error as LuaError};
use crate::interpreter::context::Context;
use crate::interpreter::error::{InterpreterError, Result};
use super::Module;
use super::external_module::{ExternalModule, ModuleLoader, ExternalModuleType, ModuleMetadata, FunctionMetadata, ExternalModuleOptions};

/// Lua模块加载器 - 用于加载.lua文件模块
pub struct LuaModuleLoader;

impl ModuleLoader for LuaModuleLoader {
    fn can_load(&self, path: &str) -> bool {
        let file_exists = std::path::Path::new(path).exists();
        let has_lua_ext = path.ends_with(".lua");
        
        if crate::is_debug_mode() {
            println!("检查Lua模块文件: {} (存在: {}, 扩展名正确: {})", 
                     path, file_exists, has_lua_ext);
        }
        
        file_exists && has_lua_ext
    }
    
    fn load(&self, name: &str, path: &str, options: Option<ExternalModuleOptions>) -> Result<Box<dyn ExternalModule>> {
        if crate::is_debug_mode() {
            println!("加载Lua模块: {} 从文件: {}", name, path);
        }
        
        // 检查文件是否存在
        if !std::path::Path::new(path).exists() {
            return Err(InterpreterError::ModuleError(
                format!("Lua模块文件不存在: {}", path)
            ));
        }
        
        // 读取Lua文件内容
        let content = fs::read_to_string(path)
            .map_err(|e| InterpreterError::ModuleError(format!("无法读取Lua文件 '{}': {}", path, e)))?;
        
        // 提取函数名称列表和module_meta，不实际执行代码
        let (functions, module_meta) = extract_module_info(&content)?;
        
        // 创建元数据
        let metadata = create_metadata(name, &functions, module_meta.as_ref())?;
        
        if crate::is_debug_mode() {
            println!("成功加载Lua模块: {}", name);
            println!("模块 {} 中的函数:", name);
            for (fname, _) in metadata.functions.iter() {
                println!("  - {}", fname);
            }
            
            if let Some(meta) = &module_meta {
                println!("模块定义了自定义元数据:");
                println!("{}", serde_json::to_string_pretty(meta).unwrap_or_else(|_| "无法格式化元数据".to_string()));
            }
        }
        
        Ok(Box::new(LuaModule {
            name: name.to_string(),
            path: path.to_string(),
            content,
            metadata,
            module_meta,
            options: options.unwrap_or_default(),
        }))
    }
    
    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec!["lua"]
    }
    
    fn get_loader_name(&self) -> &'static str {
        "Lua模块加载器"
    }
}

/// 创建模块元数据
fn create_metadata(name: &str, functions: &[String], module_meta: Option<&Value>) -> Result<ModuleMetadata> {
    // 默认元数据
    let mut metadata = ModuleMetadata {
        name: name.to_string(),
        version: "1.0.0".to_string(),
        description: "Lua模块".to_string(),
        author: "未知".to_string(),
        functions: HashMap::new(),
    };
    
    // 如果有模块自定义元数据，使用它来覆盖默认值
    if let Some(meta) = module_meta {
        if let Some(obj) = meta.as_object() {
            if let Some(version) = obj.get("version").and_then(|v| v.as_str()) {
                metadata.version = version.to_string();
            }
            if let Some(description) = obj.get("description").and_then(|v| v.as_str()) {
                metadata.description = description.to_string();
            }
            if let Some(author) = obj.get("author").and_then(|v| v.as_str()) {
                metadata.author = author.to_string();
            }
        }
    }
    
    // 为每个函数创建元数据
    for func_name in functions {
        let function_meta = FunctionMetadata {
            name: func_name.clone(),
            description: format!("Lua函数 {}", func_name),
            parameters: Vec::new(), // 在Lua中难以静态获取参数信息
            return_type: "Any".to_string(),
            example: "".to_string(),
        };
        
        metadata.functions.insert(func_name.clone(), function_meta);
    }
    
    Ok(metadata)
}

/// 从Lua文件内容中提取函数名称和module_meta
fn extract_module_info(content: &str) -> Result<(Vec<String>, Option<Value>)> {
    // 创建一个临时Lua环境来解析模块
    let lua = Lua::new();
    
    // 加载Lua代码
    let chunk_result = match lua.load(content).eval::<LuaTable>() {
        Ok(table) => table,
        Err(e) => {
            return Err(InterpreterError::ModuleError(
                format!("Lua模块加载错误: {}", e)
            ));
        }
    };
    
    // 调试：检查Lua返回的表结构
    debug_print_lua_table(&chunk_result, "根模块表");
    
    // 提取函数名称
    let mut function_names = Vec::new();
    let mut module_meta = None;
    
    for pair in chunk_result.clone().pairs::<String, mlua::Value>() {
        if let Ok((key, value)) = pair {
            match key.as_str() {
                "module_meta" => {
                    // 提取模块元数据
                    if let Ok(json_value) = lua_to_json(value.clone()) {
                        module_meta = Some(json_value);
                    }
                },
                _ => {
                    if let mlua::Value::Function(_) = value {
                        function_names.push(key);
                    }
                }
            }
        }
    }
    
    Ok((function_names, module_meta))
}

// 调试函数：递归打印Lua表结构
fn debug_print_lua_table(table: &LuaTable, prefix: &str) {
    if !crate::is_debug_mode() {
        return;
    }
    
    println!("调试: {} 表结构:", prefix);
    for pair in table.clone().pairs::<mlua::Value, mlua::Value>() {
        if let Ok((key, value)) = pair {
            let key_str = match &key {
                mlua::Value::String(s) => format!("\"{}\"", s.to_string_lossy()),
                _ => format!("{:?}", key)
            };
            
            match &value {
                mlua::Value::Table(t) => {
                    println!("  {}[{}] = <表>", prefix, key_str);
                    debug_print_lua_table(t, &format!("  {}.{}", prefix, key_str));
                },
                mlua::Value::Function(_) => {
                    println!("  {}[{}] = <函数>", prefix, key_str);
                },
                _ => {
                    println!("  {}[{}] = {:?}", prefix, key_str, value);
                }
            }
        }
    }
}

/// Lua模块实现
pub struct LuaModule {
    name: String,
    path: String,
    content: String,       // 存储Lua代码内容，每次调用时创建新环境
    metadata: ModuleMetadata,
    module_meta: Option<Value>, // 存储模块自定义元数据
    options: ExternalModuleOptions,
}

impl Module for LuaModule {
    fn get_name(&self) -> &'static str {
        Box::leak(self.name.clone().into_boxed_str())
    }
    
    fn get_functions(&self) -> Vec<(&'static str, fn(&[Value], &mut Context) -> Value)> {
        Vec::new() // 使用自定义调用机制
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ExternalModule for LuaModule {
    fn get_module_type(&self) -> ExternalModuleType {
        ExternalModuleType::External("Lua".to_string())
    }
    
    fn get_metadata(&self) -> &ModuleMetadata {
        &self.metadata
    }
    
    fn get_options(&self) -> &ExternalModuleOptions {
        &self.options
    }
    
    fn set_options(&mut self, options: ExternalModuleOptions) -> Result<()> {
        self.options = options;
        Ok(())
    }
    
    fn reload(&mut self) -> Result<()> {
        // 重新加载Lua模块内容
        self.content = fs::read_to_string(&self.path)
            .map_err(|e| InterpreterError::ModuleError(format!("无法读取Lua文件 '{}': {}", self.path, e)))?;
        
        // 重新提取函数名和模块元数据
        let (functions, module_meta) = extract_module_info(&self.content)?;
        
        // 更新元数据
        self.metadata = create_metadata(&self.name, &functions, module_meta.as_ref())?;
        
        // 更新模块元数据
        self.module_meta = module_meta;
        
        Ok(())
    }
    
    fn call_function(&self, name: &str, args: &[Value], context: &mut Context) -> Result<Value> {
        // 为每次调用创建新的Lua环境
        let lua = Lua::new();
        
        // 设置JiLang环境
        self.setup_jilang_environment(&lua, context)?;
        
        // 加载模块代码
        let module_table = lua.load(&self.content).eval::<mlua::Value>()
            .map_err(|e| InterpreterError::ModuleError(
                format!("Lua模块加载错误: {}", e)
            ))?;
        
        // 调试：详细分析模块结构
        if crate::is_debug_mode() {
            println!("调试: 调用函数 '{}' 的详细分析", name);
            println!("模块返回值类型: {:?}", module_table.type_name());
            
            match module_table {
                mlua::Value::Table(ref table) => {
                    debug_print_lua_table(table, "模块");
                    println!("尝试获取函数路径: 模块.{}", name);
                },
                _ => {
                    println!("错误: 模块不是表结构，而是 {:?}", module_table.type_name());
                }
            }
        }
        
        // 获取函数 - 尝试从table中获取
        if let mlua::Value::Table(ref table) = module_table {
            // 首先直接尝试获取函数（可能是第一级导出）
            let lua_fn_result = table.get::<_, mlua::Value>(name);
            
            // 如果直接获取失败，检查是否函数在模块表中的各个字段
            let lua_fn = match lua_fn_result {
                Ok(mlua::Value::Function(f)) => {
                    if crate::is_debug_mode() {
                        println!("直接在模块表中找到函数: '{}'", name);
                    }
                    f
                },
                _ => {
                    // 在直接获取失败的情况下，尝试更详细的搜索
                    if crate::is_debug_mode() {
                        println!("直接获取函数失败，尝试遍历模块表寻找...");
                    }
                    
                    // 列出所有键并寻找匹配的函数
                    let mut found_fn = None;
                    for pair in table.clone().pairs::<String, mlua::Value>() {
                        if let Ok((key, val)) = pair {
                            if crate::is_debug_mode() {
                                println!("检查键: '{}' (类型: {:?})", key, val.type_name());
                            }
                            
                            if key == name {
                                if let mlua::Value::Function(f) = val {
                                    found_fn = Some(f);
                                    if crate::is_debug_mode() {
                                        println!("在模块表中找到函数: '{}'", name);
                                    }
                                    break;
                                } else if crate::is_debug_mode() {
                                    println!("键名匹配但不是函数: '{}'", name);
                                }
                            }
                        }
                    }
                    
                    // 如果还是找不到，返回错误
                    match found_fn {
                        Some(f) => f,
                        None => {
                            if crate::is_debug_mode() {
                                println!("未能在模块表中找到函数: '{}'", name);
                                println!("模块中的所有键:");
                                for pair in table.clone().pairs::<String, mlua::Value>() {
                                    if let Ok((key, val)) = pair {
                                        println!("  - {}: {:?}", key, val.type_name());
                                    }
                                }
                            }
                            
                            return Err(InterpreterError::FunctionError(
                                format!("Lua模块 '{}' 中未找到函数 '{}'", self.name, name)
                            ));
                        }
                    }
                }
            };
            
            // 转换参数为Lua值
            let mut lua_args = Vec::new();
            for (i, arg) in args.iter().enumerate() {
                if crate::is_debug_mode() {
                    println!("转换第{}个参数: {:?}", i, arg);
                }
                
                let lua_value = match json_to_lua(&lua, arg) {
                    Ok(val) => {
                        if crate::is_debug_mode() {
                            println!("  转换结果: {} (类型: {:?})", 
                                      match &val {
                                          mlua::Value::Nil => "nil".to_string(),
                                          mlua::Value::Boolean(b) => format!("{}", b),
                                          mlua::Value::Integer(i) => format!("{}", i),
                                          mlua::Value::Number(n) => format!("{}", n),
                                          mlua::Value::String(s) => s.to_string_lossy().to_string(),
                                          _ => format!("{:?}", val.type_name())
                                      }, 
                                      val.type_name());
                        }
                        val
                    },
                    Err(e) => {
                        if crate::is_debug_mode() {
                            println!("  转换失败: {}", e);
                        }
                        return Err(InterpreterError::RuntimeError(
                            format!("转换参数为Lua值失败: {}", e)
                        ));
                    }
                };
                lua_args.push(lua_value);
            }
            
            // 调用Lua函数
            if crate::is_debug_mode() {
                println!("调用Lua函数 '{}' 传入 {} 个参数", name, lua_args.len());
            }
            
            let lua_result = lua_fn.call::<_, mlua::Value>(lua_args)
                .map_err(|e| InterpreterError::RuntimeError(
                    format!("Lua函数 '{}' 调用失败: {}", name, e)
                ))?;
            
            // 转换Lua结果为JSON值
            let result = lua_to_json(lua_result)
                .map_err(|e| InterpreterError::RuntimeError(
                    format!("转换Lua结果为JSON值失败: {}", e)
                ))?;
            
            if crate::is_debug_mode() {
                println!("Lua函数 '{}' 调用成功，返回值: {:?}", name, result);
            }
            
            Ok(result)
        } else {
            Err(InterpreterError::ModuleError(
                format!("Lua模块 '{}' 未返回一个表", self.name)
            ))
        }
    }
    
    fn get_function_metadata(&self, name: &str) -> Option<&FunctionMetadata> {
        self.metadata.functions.get(name)
    }
    
    fn has_function(&self, name: &str) -> bool {
        self.metadata.functions.contains_key(name)
    }
    
    fn get_all_function_metadata(&self) -> Vec<&FunctionMetadata> {
        self.metadata.functions.values().collect()
    }
    
    fn initialize(&mut self) -> Result<()> {
        // Lua模块初始化时不需要特殊操作
        Ok(())
    }
    
    fn destroy(&mut self) -> Result<()> {
        // Lua模块销毁时不需要特殊操作
        Ok(())
    }
    
    fn get_jlang_function(&self, _name: &str) -> Option<Value> {
        None // Lua模块不支持该功能
    }
    
    fn get_module_meta_value(&self) -> Option<&Value> {
        self.module_meta.as_ref()
    }
}

impl LuaModule {
    // 设置JiLang环境到Lua状态机
    fn setup_jilang_environment(&self, lua: &Lua, context: &mut Context) -> Result<()> {
        // 创建jilang全局表
        let globals = lua.globals();
        let jilang_table = lua.create_table()
            .map_err(|e| InterpreterError::RuntimeError(format!("创建Lua表失败: {}", e)))?;
        
        // 添加变量访问函数
        self.add_get_var_function(lua, &jilang_table, context)?;
        
        // 添加设置变量函数
        self.add_set_var_function(lua, &jilang_table, context)?;
        
        // 添加打印函数
        self.add_print_function(lua, &jilang_table)?;
        
        // 添加调用JiLang函数
        self.add_call_function(lua, &jilang_table, context)?;
        
        // 设置jilang表到全局环境
        globals.set("jilang", jilang_table)
            .map_err(|e| InterpreterError::RuntimeError(format!("设置jilang表失败: {}", e)))?;
        
        Ok(())
    }
    
    fn add_get_var_function(&self, lua: &Lua, table: &LuaTable, context: &Context) -> Result<()> {
        // 捕获context的引用到Lua闭包
        let context_ref = context as *const Context;
        
        let get_var = lua.create_function(move |lua_ctx, var_name: String| {
            // 安全地从指针恢复context引用
            let context = unsafe { &*context_ref };
            
            // 获取变量值
            if let Some(value) = context.get_value(&var_name) {
                // 将JiLang变量值转换为Lua值
                json_to_lua(lua_ctx, &value)
            } else {
                // 如果变量不存在，返回nil
                Ok(mlua::Value::Nil)
            }
        }).map_err(|e| InterpreterError::RuntimeError(format!("创建get_var函数失败: {}", e)))?;
        
        table.set("get_var", get_var)
            .map_err(|e| InterpreterError::RuntimeError(format!("设置get_var函数失败: {}", e)))?;
        
        Ok(())
    }
    
    fn add_set_var_function(&self, lua: &Lua, table: &LuaTable, context: &mut Context) -> Result<()> {
        // 捕获context的可变引用到Lua闭包
        let context_ptr = context as *mut Context;
        
        let set_var = lua.create_function(move |_, (var_name, value): (String, mlua::Value)| {
            // 安全地从指针恢复context可变引用
            let context = unsafe { &mut *context_ptr };
            
            // 将Lua值转换为JiLang值
            match lua_to_json(value) {
                Ok(json_value) => {
                    // 设置变量
                    match context.set_variable(var_name.clone(), json_value) {
                        Ok(_) => Ok(true),
                        Err(e) => Err(LuaError::RuntimeError(format!("设置变量 {} 失败: {}", var_name, e)))
                    }
                },
                Err(e) => Err(e)
            }
        }).map_err(|e| InterpreterError::RuntimeError(format!("创建set_var函数失败: {}", e)))?;
        
        table.set("set_var", set_var)
            .map_err(|e| InterpreterError::RuntimeError(format!("设置set_var函数失败: {}", e)))?;
        
        Ok(())
    }
    
    fn add_print_function(&self, lua: &Lua, table: &LuaTable) -> Result<()> {
        let print_fn = lua.create_function(|_, text: String| {
            println!("{}", text);
            Ok(())
        }).map_err(|e| InterpreterError::RuntimeError(format!("创建print函数失败: {}", e)))?;
        
        table.set("print", print_fn)
            .map_err(|e| InterpreterError::RuntimeError(format!("设置print函数失败: {}", e)))?;
        
        Ok(())
    }
    
    fn add_call_function(&self, lua: &Lua, table: &LuaTable, context: &mut Context) -> Result<()> {
        // 捕获context的可变引用到Lua闭包
        let context_ptr = context as *mut Context;
        
        let call_fn = lua.create_function(move |lua_ctx, (func_name, args): (String, mlua::Value)| {
            // 安全地从指针恢复context可变引用
            let context = unsafe { &mut *context_ptr };
            
            // 将参数转换为JiLang数组
            let jilang_args = match lua_to_json(args) {
                Ok(Value::Array(arr)) => arr,
                Ok(value) => vec![value],
                Err(e) => return Err(e)
            };
            
            // 执行JiLang语句
            match crate::interpreter::statements::execute_statement(&func_name, &Value::Array(jilang_args), context, None) {
                Ok(result) => json_to_lua(lua_ctx, &result),
                Err(e) => Err(LuaError::RuntimeError(format!("执行语句 {} 失败: {}", func_name, e)))
            }
        }).map_err(|e| InterpreterError::RuntimeError(format!("创建call函数失败: {}", e)))?;
        
        table.set("call", call_fn)
            .map_err(|e| InterpreterError::RuntimeError(format!("设置call函数失败: {}", e)))?;
        
        Ok(())
    }
    
    // 获取模块自定义元数据
    pub fn get_module_meta_value(&self) -> Option<&Value> {
        self.module_meta.as_ref()
    }
}

/// 将JSON值转换为Lua值
fn json_to_lua<'lua>(lua: &'lua Lua, value: &Value) -> mlua::Result<mlua::Value<'lua>> {
    if crate::is_debug_mode() {
        // 修复：使用match获取值类型而不是调用不存在的type_name方法
        let type_str = match value {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
        };
        println!("将JSON值转换为Lua: 值={:?}, 类型={}", value, type_str);
    }
    
    match value {
        Value::Null => Ok(mlua::Value::Nil),
        Value::Bool(b) => Ok(mlua::Value::Boolean(*b)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                if crate::is_debug_mode() {
                    println!("  JSON整数转Lua整数: {} -> Lua::Integer", i);
                }
                Ok(mlua::Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                if crate::is_debug_mode() {
                    println!("  JSON浮点数转Lua浮点数: {} -> Lua::Number", f);
                }
                Ok(mlua::Value::Number(f))
            } else {
                // 如果无法提取数值，尝试转换为字符串然后解析
                let string_val = n.to_string();
                if crate::is_debug_mode() {
                    println!("  JSON数字转换失败，尝试字符串方式: {}", string_val);
                }
                
                if let Ok(i) = string_val.parse::<i64>() {
                    Ok(mlua::Value::Integer(i))
                } else if let Ok(f) = string_val.parse::<f64>() {
                    Ok(mlua::Value::Number(f))
                } else {
                    Err(LuaError::FromLuaConversionError {
                        from: "number",
                        to: "lua value",
                        message: Some(format!("无效的JSON数字: {}", n)),
                    })
                }
            }
        },
        Value::String(s) => {
            if crate::is_debug_mode() {
                println!("  JSON字符串: \"{}\"", s);
            }
            
            // 对于字符串，尝试解析为数字
            if let Ok(i) = s.parse::<i64>() {
                if crate::is_debug_mode() {
                    println!("  字符串解析为整数: {}", i);
                }
                Ok(mlua::Value::Integer(i))
            } else if let Ok(f) = s.parse::<f64>() {
                if crate::is_debug_mode() {
                    println!("  字符串解析为浮点数: {}", f);
                }
                Ok(mlua::Value::Number(f))
            } else {
                if crate::is_debug_mode() {
                    println!("  作为普通字符串保留: \"{}\"", s);
                }
                Ok(mlua::Value::String(lua.create_string(s)?))
            }
        },
        Value::Array(arr) => {
            if crate::is_debug_mode() {
                println!("  JSON数组转Lua表，数组长度: {}", arr.len());
            }
            
            let lua_table = lua.create_table()?;
            for (i, item) in arr.iter().enumerate() {
                if crate::is_debug_mode() {
                    println!("  处理数组元素[{}]: {:?}", i, item);
                }
                lua_table.set(i + 1, json_to_lua(lua, item)?)?;
            }
            Ok(mlua::Value::Table(lua_table))
        },
        Value::Object(obj) => {
            if crate::is_debug_mode() {
                println!("  JSON对象转Lua表，键值对数量: {}", obj.len());
            }
            
            let lua_table = lua.create_table()?;
            for (key, value) in obj {
                if crate::is_debug_mode() {
                    println!("  处理对象键值对: {} -> {:?}", key, value);
                }
                lua_table.set(key.clone(), json_to_lua(lua, value)?)?;
            }
            Ok(mlua::Value::Table(lua_table))
        }
    }
}

/// 将Lua值转换为JSON值
fn lua_to_json(value: mlua::Value) -> mlua::Result<Value> {
    if crate::is_debug_mode() {
        println!("将Lua值转换为JSON: 类型={:?}", value.type_name());
    }
    
    match value {
        mlua::Value::Nil => Ok(Value::Null),
        mlua::Value::Boolean(b) => Ok(Value::Bool(b)),
        mlua::Value::Integer(i) => {
            if crate::is_debug_mode() {
                println!("  转换Lua整数: {} -> JSON数字", i);
            }
            Ok(Value::Number(serde_json::Number::from(i)))
        },
        mlua::Value::Number(n) => {
            if crate::is_debug_mode() {
                println!("  转换Lua浮点数: {} -> JSON数字", n);
            }
            // 直接使用精确的浮点数值
            if let Some(num) = serde_json::Number::from_f64(n) {
                Ok(Value::Number(num))
            } else {
                // 如果失败，尝试使用字符串表示再转换
                let s = n.to_string();
                if crate::is_debug_mode() {
                    println!("  浮点数转换失败，尝试通过字符串: {}", s);
                }
                
                if let Ok(parsed) = s.parse::<f64>() {
                    if let Some(num) = serde_json::Number::from_f64(parsed) {
                        Ok(Value::Number(num))
                    } else {
                        // 最后使用整数
                        let i = n as i64;
                        if crate::is_debug_mode() {
                            println!("  尝试通过整数转换: {}", i);
                        }
                        Ok(Value::Number(serde_json::Number::from(i)))
                    }
                } else {
                    Ok(Value::String(s))
                }
            }
        },
        mlua::Value::String(s) => Ok(Value::String(s.to_str()?.to_string())),
        mlua::Value::Table(t) => {
            if crate::is_debug_mode() {
                println!("  转换Lua表为JSON");
            }
            
            // 检查是否为数组
            let len = t.len()?;
            if len > 0 {
                // 首先检查是否可以作为数组处理
                let mut is_array = true;
                for i in 1..=len {
                    if let mlua::Value::Nil = t.get::<_, mlua::Value>(i)? {
                        is_array = false;
                        break;
                    }
                }
                
                if is_array {
                    if crate::is_debug_mode() {
                        println!("  Lua表作为数组处理，长度: {}", len);
                    }
                    
                    let mut array = Vec::with_capacity(len as usize);
                    for i in 1..=len {
                        let item = t.get::<_, mlua::Value>(i)?;
                        array.push(lua_to_json(item)?);
                    }
                    return Ok(Value::Array(array));
                }
            }
            
            // 作为对象处理
            if crate::is_debug_mode() {
                println!("  Lua表作为对象处理");
            }
            
            let mut obj = serde_json::Map::new();
            for pair in t.pairs::<mlua::Value, mlua::Value>() {
                let (key, value) = pair?;
                
                let key_str = match key {
                    mlua::Value::String(s) => s.to_str()?.to_string(),
                    mlua::Value::Integer(i) => i.to_string(),
                    mlua::Value::Number(n) => n.to_string(),
                    _ => {
                        if crate::is_debug_mode() {
                            println!("  跳过不支持的键类型: {:?}", key.type_name());
                        }
                        continue;
                    }
                };
                
                if crate::is_debug_mode() {
                    println!("  处理对象键: {}", key_str);
                }
                
                let json_value = lua_to_json(value)?;
                obj.insert(key_str, json_value);
            }
            
            Ok(Value::Object(obj))
        },
        mlua::Value::Function(_) => {
            if crate::is_debug_mode() {
                println!("  函数类型转换为字符串");
            }
            Ok(Value::String("<Lua函数>".to_string()))
        },
        _ => {
            if crate::is_debug_mode() {
                println!("  不支持的Lua类型: {:?}", value.type_name());
            }
            Ok(Value::String(format!("<不支持的Lua值类型: {}>", value.type_name())))
        },
    }
} 
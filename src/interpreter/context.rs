use std::collections::HashMap;
use serde_json::Value;
use crate::modules;
use crate::modules::external_module::{ExternalModule, ExternalModuleOptions};
use super::error::{InterpreterError, Result};
use super::error::error_messages::context as error_msg;
use super::variable_reference::{VariableReference, ReferenceType};
use crate::is_print_full_values;  // 导入新函数
use std::collections::BTreeMap;

// 上下文选项结构体
#[derive(Debug, Clone, Default)]
pub struct ContextOptions {
    pub debug_mode: bool,
    pub include_stack_trace: bool,
}

pub struct Context {
    pub variables: HashMap<String, Value>,
    pub constants: HashMap<String, Value>,
    pub program: Value,
    pub modules: HashMap<String, Box<dyn modules::Module>>,
    pub module_meta: HashMap<String, Value>,
    pub current_path: Option<String>,
    pub options: ContextOptions,
    return_value: Option<Value>,
    is_returning: bool,
}

impl Context {
    pub fn new(program: Value, modules: Vec<Box<dyn modules::Module>>) -> Result<Self> {
        let mut context = Context {
            variables: HashMap::new(),
            constants: HashMap::new(),
            program: program.clone(),
            modules: HashMap::new(),
            module_meta: HashMap::new(),
            current_path: None,
            options: ContextOptions::default(),
            return_value: None,
            is_returning: false,
        };

        // 验证程序结构
        if !program.is_object() {
            return Err(InterpreterError::InvalidProgramStructure(
                error_msg::PROGRAM_NOT_OBJECT.to_string()
            ));
        }

        // 加载常量
        if let Some(constants) = program.get("const") {
            if let Some(obj) = constants.as_object() {
                for (key, value) in obj {
                    context.constants.insert(key.clone(), value.clone());
                }
            } else {
                return Err(InterpreterError::InvalidProgramStructure(
                    error_msg::CONST_NOT_OBJECT.to_string()
                ));
            }
        }

        // 加载模块
        for module in modules {
            let name = module.get_name().to_string();
            
            // 加载模块元数据（如果有）
            if let Some(lua_module) = module.as_any().downcast_ref::<crate::modules::lua_module::LuaModule>() {
                if let Some(meta_value) = lua_module.get_module_meta_value() {
                    context.module_meta.insert(name.clone(), meta_value.clone());
                    if crate::is_debug_mode() {
                        println!("已加载Lua模块 '{}' 的元数据", name);
                    }
                }
            } else if let Some(jl_module) = module.as_any().downcast_ref::<crate::modules::external_module::JLangExternalModule>() {
                if let Some(meta_value) = jl_module.get_module_meta_value() {
                    context.module_meta.insert(name.clone(), meta_value.clone());
                    if crate::is_debug_mode() {
                        println!("已加载JL模块 '{}' 的元数据", name);
                    }
                }
            }
            
            context.modules.insert(name, module);
        }

        // 创建特殊的module_meta变量
        if !context.module_meta.is_empty() {
            let meta_obj = Value::Object(
                context.module_meta.iter()
                    .map(|(k, v)| (k.clone(), v.clone()))
                    .collect()
            );
            context.variables.insert("module_meta".to_string(), meta_obj);
            
            if crate::is_debug_mode() {
                println!("已创建module_meta全局变量，包含 {} 个模块的元数据", context.module_meta.len());
            }
        }

        // 检查函数名冲突
        if let Some(program_obj) = program.get("program") {
            if let Some(obj) = program_obj.as_object() {
                for (func_name, _) in obj {
                    // 检查是否是内置语句
                    if crate::interpreter::statements::is_builtin_statement(func_name) {
                        return Err(InterpreterError::FunctionError(
                            error_msg::function_name_conflict_builtin(func_name)
                        ));
                    }

                    // 检查是否是模块函数
                    if func_name.contains('.') {
                        let parts: Vec<&str> = func_name.split('.').collect();
                        if parts.len() == 2 {
                            let module_name = parts[0];
                            let function_name = parts[1];
                            if let Some(module) = context.modules.get(module_name) {
                                for (fname, _) in module.get_functions() {
                                    if fname == function_name {
                                        return Err(InterpreterError::FunctionError(
                                            error_msg::function_name_conflict_module(func_name)
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(context)
    }

    pub fn get_value(&self, text: &str) -> Option<Value> {
        if VariableReference::is_reference(text) {
            let var_ref = VariableReference::parse(text);
            Some(var_ref.resolve_value(&self.variables, &self.constants))
        } else {
            None
        }
    }

    pub fn set_variable(&mut self, name: String, value: Value) -> Result<()> {
        if self.constants.contains_key(&name) {
            return Err(InterpreterError::VariableError(
                error_msg::constant_modification(&name)
            ));
        }
        self.variables.insert(name, value);
        Ok(())
    }

    pub fn resolve_value(&self, value: &Value) -> String {
        match value {
            Value::String(text) => {
                if VariableReference::is_reference(text) {
                    let var_ref = VariableReference::parse(text);
                    match var_ref.resolve_value(&self.variables, &self.constants) {
                        Value::String(s) => s.to_string(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => "null".to_string(),
                        Value::Array(arr) => {
                            if is_print_full_values() {
                                let mut result = String::new();
                                result.push('[');
                                for (i, item) in arr.iter().enumerate() {
                                    if i > 0 {
                                        result.push_str(", ");
                                    }
                                    result.push_str(&self.resolve_value(item));
                                }
                                result.push(']');
                                result
                            } else {
                                "<array>".to_string()
                            }
                        },
                        Value::Object(obj) => {
                            if is_print_full_values() {
                                let mut result = String::new();
                                result.push('{');
                                for (i, (key, val)) in obj.iter().enumerate() {
                                    if i > 0 {
                                        result.push_str(", ");
                                    }
                                    result.push_str(&format!("\"{}\": {}", key, self.resolve_value(val)));
                                }
                                result.push('}');
                                result
                            } else {
                                "<object>".to_string()
                            }
                        },
                    }
                } else {
                    text.to_string()
                }
            },
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(arr) => {
                if is_print_full_values() {
                    let mut result = String::new();
                    result.push('[');
                    for (i, item) in arr.iter().enumerate() {
                        if i > 0 {
                            result.push_str(", ");
                        }
                        result.push_str(&self.resolve_value(item));
                    }
                    result.push(']');
                    result
                } else {
                    "<array>".to_string()
                }
            },
            Value::Object(obj) => {
                if is_print_full_values() {
                    let mut result = String::new();
                    result.push('{');
                    for (i, (key, val)) in obj.iter().enumerate() {
                        if i > 0 {
                            result.push_str(", ");
                        }
                        result.push_str(&format!("\"{}\": {}", key, self.resolve_value(val)));
                    }
                    result.push('}');
                    result
                } else {
                    "<object>".to_string()
                }
            },
        }
    }

    pub fn resolve_value_with_error(&self, value: &Value) -> Result<String> {
        match value {
            Value::String(text) => {
                if VariableReference::is_reference(text) {
                    // 打印调试信息，确认变量引用格式
                    if crate::is_debug_mode() {
                        println!("解析变量引用: {}", text);
                    }
                    
                    let var_ref = VariableReference::parse(text);
                    
                    // 打印解析后的引用类型和变量名
                    if crate::is_debug_mode() {
                        println!("变量引用类型: {:?}, 变量名: {}", var_ref.ref_type, var_ref.name);
                    }
                    
                    // 使用新的解析方法获取变量值
                    let resolved = var_ref.resolve_value_with_error(&self.variables, &self.constants)?;
                    
                    // 打印解析后的值
                    if crate::is_debug_mode() {
                        println!("解析结果值: {:?}", resolved);
                    }
                    
                    Ok(match resolved {
                        Value::String(s) => s.to_string(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => "null".to_string(),
                        Value::Array(arr) => {
                            if is_print_full_values() {
                                let mut result = String::new();
                                result.push('[');
                                for (i, item) in arr.iter().enumerate() {
                                    if i > 0 {
                                        result.push_str(", ");
                                    }
                                    result.push_str(&self.resolve_value(item));
                                }
                                result.push(']');
                                result
                            } else {
                                "<array>".to_string()
                            }
                        },
                        Value::Object(obj) => {
                            if is_print_full_values() {
                                let mut result = String::new();
                                result.push('{');
                                for (i, (key, val)) in obj.iter().enumerate() {
                                    if i > 0 {
                                        result.push_str(", ");
                                    }
                                    result.push_str(&format!("\"{}\": {}", key, self.resolve_value(val)));
                                }
                                result.push('}');
                                result
                            } else {
                                "<object>".to_string()
                            }
                        },
                    })
                } else {
                    Ok(text.to_string())
                }
            },
            Value::Number(n) => Ok(n.to_string()),
            Value::Bool(b) => Ok(b.to_string()),
            Value::Null => Ok("null".to_string()),
            Value::Array(arr) => {
                if is_print_full_values() {
                    let mut result = String::new();
                    result.push('[');
                    for (i, item) in arr.iter().enumerate() {
                        if i > 0 {
                            result.push_str(", ");
                        }
                        result.push_str(&self.resolve_value(item));
                    }
                    result.push(']');
                    Ok(result)
                } else {
                    Ok("<array>".to_string())
                }
            },
            Value::Object(obj) => {
                if is_print_full_values() {
                    let mut result = String::new();
                    result.push('{');
                    for (i, (key, val)) in obj.iter().enumerate() {
                        if i > 0 {
                            result.push_str(", ");
                        }
                        result.push_str(&format!("\"{}\": {}", key, self.resolve_value(val)));
                    }
                    result.push('}');
                    Ok(result)
                } else {
                    Ok("<object>".to_string())
                }
            },
        }
    }

    pub fn process_special_chars(&self, text: &str) -> String {
        text.replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
    }

    pub fn call_module_function(&mut self, module_name: &str, function_name: &str, args: &[Value]) -> Result<Value> {
        // 使用clone避免借用冲突
        let module_name = module_name.to_string();
        let function_name = function_name.to_string();
        let args = args.to_vec();
        
        // 检查模块是否存在
        if !self.modules.contains_key(&module_name) {
            return Err(InterpreterError::ModuleError(
                error_msg::module_not_found(&module_name)
            ));
        }
        
        // 首先尝试普通模块函数调用
        let module = self.modules.get(&module_name).unwrap();
        for (fname, func) in module.get_functions() {
            if fname == function_name {
                return Ok(func(&args, self));
            }
        }
        
        // 备份模块类型
        let is_lua_module = module.as_any().downcast_ref::<crate::modules::lua_module::LuaModule>().is_some();
        let is_jlang_module = module.as_any().downcast_ref::<crate::modules::external_module::JLangExternalModule>().is_some();
        
        // 使用unsafe处理借用冲突问题
        if is_lua_module {
            let module_ptr = self.modules.get_mut(&module_name).unwrap() as *mut Box<dyn modules::Module>;
            let module_ref = unsafe { &*module_ptr };
            
            if let Some(lua_module) = module_ref.as_any().downcast_ref::<crate::modules::lua_module::LuaModule>() {
                if lua_module.has_function(&function_name) {
                    if crate::is_debug_mode() {
                        println!("调用Lua模块 '{}' 中的函数: '{}'", module_name, function_name);
                    }
                    
                    let result = lua_module.call_function(&function_name, &args, self);
                    return result;
                }
            }
        } else if is_jlang_module {
            let module_ptr = self.modules.get_mut(&module_name).unwrap() as *mut Box<dyn modules::Module>;
            let module_ref = unsafe { &*module_ptr };
            
            if let Some(jlang_module) = module_ref.as_any().downcast_ref::<crate::modules::external_module::JLangExternalModule>() {
                if jlang_module.has_function(&function_name) {
                    if crate::is_debug_mode() {
                        println!("调用JLang外部模块 '{}' 中的函数: '{}'", module_name, function_name);
                    }
                    
                    let result = jlang_module.call_function(&function_name, &args, self);
                    return result;
                }
            }
        }
        
        // 如果到这里还没有返回，说明没有找到函数
        Err(InterpreterError::ModuleError(
            error_msg::module_function_not_found(&module_name, &function_name)
        ))
    }

    pub fn set_return_value(&mut self, value: Option<Value>) {
        // 使用clone避免所有权问题
        self.is_returning = value.is_some();
        self.return_value = value;
    }

    pub fn get_return_value(&self) -> Option<&Value> {
        self.return_value.as_ref()
    }

    pub fn is_returning(&self) -> bool {
        self.is_returning
    }

    pub fn reset_return_status(&mut self) {
        self.return_value = None;
        self.is_returning = false;
    }
} 
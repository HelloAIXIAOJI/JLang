use std::collections::HashMap;
use serde_json::Value;
use crate::modules;
use super::error::{InterpreterError, Result};
use super::error::error_messages::context as error_msg;

pub struct Context {
    pub variables: HashMap<String, Value>,
    pub constants: HashMap<String, Value>,
    pub program: Value,
    pub modules: HashMap<String, Box<dyn modules::Module>>,
}

impl Context {
    pub fn new(program: Value, modules: Vec<Box<dyn modules::Module>>) -> Result<Self> {
        let mut context = Context {
            variables: HashMap::new(),
            constants: HashMap::new(),
            program: program.clone(),
            modules: HashMap::new(),
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
            context.modules.insert(name, module);
        }

        // 检查函数名冲突
        if let Some(program_obj) = program.get("program") {
            if let Some(obj) = program_obj.as_object() {
                for (func_name, _) in obj {
                    // 检查是否是内置语句
                    if is_builtin_statement(func_name) {
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

    pub fn get_value(&self, text: &str) -> Option<&Value> {
        if text.starts_with("@var.") {
            let var_path = &text[5..];
            
            // 处理数组索引访问，如 array[0]
            if let Some(bracket_pos) = var_path.find('[') {
                if let Some(end_bracket) = var_path.find(']') {
                    if bracket_pos < end_bracket {
                        let base_var = &var_path[0..bracket_pos];
                        let index_str = &var_path[bracket_pos+1..end_bracket];
                        
                        // 尝试将索引解析为数字
                        if let Ok(index) = index_str.parse::<usize>() {
                            if let Some(array_value) = self.variables.get(base_var) {
                                if let Some(arr) = array_value.as_array() {
                                    return arr.get(index);
                                }
                            }
                        }
                        return None;
                    }
                }
            }
            
            // 处理多层嵌套路径，如 nested_data.level1.level2...
            if var_path.contains('.') {
                let parts: Vec<&str> = var_path.split('.').collect();
                let base_var = parts[0];
                
                if let Some(base_value) = self.variables.get(base_var) {
                    let mut current_value = base_value;
                    
                    // 遍历路径中的每一段
                    for &part in &parts[1..] {
                        // 检查是否有数组索引
                        if let Some(bracket_pos) = part.find('[') {
                            if let Some(end_bracket) = part.find(']') {
                                if bracket_pos < end_bracket {
                                    let obj_key = &part[0..bracket_pos];
                                    let index_str = &part[bracket_pos+1..end_bracket];
                                    
                                    // 先获取对象属性
                                    if let Some(obj) = current_value.as_object() {
                                        if let Some(array_value) = obj.get(obj_key) {
                                            // 再获取数组索引
                                            if let Ok(index) = index_str.parse::<usize>() {
                                                if let Some(arr) = array_value.as_array() {
                                                    if let Some(item) = arr.get(index) {
                                                        current_value = item;
                                                        continue;
                                                    }
                                                }
                                            }
                                            return None;
                                        }
                                    }
                                    return None;
                                }
                            }
                        }
                        
                        // 普通对象属性访问
                        if let Some(obj) = current_value.as_object() {
                            if let Some(next_value) = obj.get(part) {
                                current_value = next_value;
                            } else {
                                // 路径中的某一段不存在
                                return None;
                            }
                        } else {
                            // 当前值不是对象，无法继续访问
                            return None;
                        }
                    }
                    
                    return Some(current_value);
                } else {
                    return None;  // 基础变量不存在
                }
            } else {
                // 原有的简单变量访问
                return self.variables.get(var_path);
            }
        } else if text.starts_with("@params.") {
            let var_name = &text[8..];
            self.variables.get(var_name)
        } else if text.starts_with("@const.") {
            let const_name = &text[7..];
            self.constants.get(const_name)
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
                if text.starts_with("@") {
                    if let Some(resolved) = self.get_value(text) {
                        match resolved {
                            Value::String(s) => self.process_special_chars(s),
                            Value::Number(n) => n.to_string(),
                            _ => resolved.to_string()
                        }
                    } else {
                        self.process_special_chars(text)
                    }
                } else {
                    self.process_special_chars(text)
                }
            }
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            _ => value.to_string()
        }
    }

    pub fn process_special_chars(&self, text: &str) -> String {
        text.replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\r", "\r")
    }

    pub fn call_module_function(&mut self, module_name: &str, function_name: &str, args: &[Value]) -> Result<Value> {
        if let Some(module) = self.modules.get(module_name) {
            for (fname, func) in module.get_functions() {
                if fname == function_name {
                    return Ok(func(args, self));
                }
            }
            Err(InterpreterError::ModuleError(
                error_msg::module_function_not_found(module_name, function_name)
            ))
        } else {
            Err(InterpreterError::ModuleError(
                error_msg::module_not_found(module_name)
            ))
        }
    }
}

// 检查是否是内置语句
fn is_builtin_statement(name: &str) -> bool {
    matches!(name, "var" | "echo" | "concat" | "if" | "call" | "while" | "for" | "comment" | "exec" | "switch" 
             | "array.create" | "array.push" | "array.pop" | "array.get" | "array.set" | "array.length" | "array.slice")
} 
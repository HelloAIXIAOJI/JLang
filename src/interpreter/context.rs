use std::collections::HashMap;
use serde_json::Value;
use crate::modules;
use super::error::{InterpreterError, Result};
use super::error::error_messages::context as error_msg;
use super::variable_reference::{VariableReference, ReferenceType};

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
                        Value::Array(_) => "<array>".to_string(),
                        Value::Object(_) => "<object>".to_string(),
                    }
                } else {
                    text.to_string()
                }
            },
            Value::Number(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Null => "null".to_string(),
            Value::Array(_) => "<array>".to_string(),
            Value::Object(_) => "<object>".to_string(),
        }
    }

    pub fn resolve_value_with_error(&self, value: &Value) -> Result<String> {
        match value {
            Value::String(text) => {
                if VariableReference::is_reference(text) {
                    let var_ref = VariableReference::parse(text);
                    let resolved = var_ref.resolve_value_with_error(&self.variables, &self.constants)?;
                    Ok(match resolved {
                        Value::String(s) => s.to_string(),
                        Value::Number(n) => n.to_string(),
                        Value::Bool(b) => b.to_string(),
                        Value::Null => "null".to_string(),
                        Value::Array(_) => "<array>".to_string(),
                        Value::Object(_) => "<object>".to_string(),
                    })
                } else {
                    Ok(text.to_string())
                }
            },
            Value::Number(n) => Ok(n.to_string()),
            Value::Bool(b) => Ok(b.to_string()),
            Value::Null => Ok("null".to_string()),
            Value::Array(_) => Ok("<array>".to_string()),
            Value::Object(_) => Ok("<object>".to_string()),
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
             | "array.create" | "array.push" | "array.pop" | "array.get" | "array.set" | "array.length" | "array.slice"
             | "object.create" | "object.get" | "object.set" | "object.has" | "object.keys" | "object.values" | "object.delete"
             | "regex.match" | "regex.test" | "regex.replace" | "regex.split")
} 
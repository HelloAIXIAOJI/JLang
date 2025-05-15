use serde_json::Value;
use super::super::context::Context;
use super::super::error::{InterpreterError, Result};
use super::super::error::error_messages::statement;
use super::super::variable_reference::{VariableReference, ReferenceType};

// execute_object_create - 创建新对象
pub fn execute_object_create(args: &Value, context: &mut Context) -> Result<()> {
    let result = if let Some(obj) = args.as_object() {
        // 如果提供了初始属性，则使用它们创建对象
        // 处理每个属性，解析变量引用
        let mut result_obj = serde_json::Map::new();
        for (key, value) in obj {
            let resolved_value = if let Some(text) = value.as_str() {
                if VariableReference::is_reference(text) {
                    if let Some(val) = context.get_value(text) {
                        val.clone()
                    } else {
                        Value::String(text.to_string())
                    }
                } else {
                    Value::String(text.to_string())
                }
            } else {
                value.clone()
            };
            result_obj.insert(key.clone(), resolved_value);
        }
        Value::Object(result_obj)
    } else {
        // 如果参数不是对象，则创建一个空对象
        Value::Object(serde_json::Map::new())
    };
    
    context.set_variable("result".to_string(), result)?;
    Ok(())
}

// execute_object_get - 获取对象属性
pub fn execute_object_get(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.get' 需要两个参数：对象和属性名".to_string()
            ));
        }
        
        // 获取对象
        let obj_value = if let Some(obj_ref_str) = args_array[0].as_str() {
            if VariableReference::is_reference(obj_ref_str) {
                if let Some(val) = context.get_value(obj_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        format!("杂鱼~变量 '{}' 不存在", obj_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.get' 的第一个参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是对象类型
        let obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.get' 的第一个参数必须是一个对象".to_string()
            ));
        };
        
        // 获取属性名
        let key = context.resolve_value(&args_array[1]);
        
        // 获取属性值
        let value = if let Some(val) = obj.get(&key) {
            val.clone()
        } else {
            Value::Null
        };
        
        // 存储结果
        context.set_variable("result".to_string(), value)?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.get' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// execute_object_set - 设置对象属性
pub fn execute_object_set(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 3 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.set' 需要三个参数：对象、属性名和值".to_string()
            ));
        }
        
        // 获取对象变量引用
        let obj_ref = &args_array[0];
        let obj_var_name = if let Some(var_name) = obj_ref.as_str() {
            let var_ref = VariableReference::parse(var_name);
            if var_ref.ref_type == ReferenceType::Variable {
                var_ref.name
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.set' 的第一个参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.set' 的第一个参数必须是一个字符串变量引用".to_string()
            ));
        };
        
        // 获取对象变量 - 使用variable_reference模块
        let var_ref = VariableReference {
            ref_type: ReferenceType::Variable,
            name: obj_var_name.clone(),
        };
        
        let obj_value = if let Some(val) = var_ref.get_value(&context.variables, &context.constants) {
            val.clone()
        } else {
            return Err(InterpreterError::RuntimeError(
                format!("杂鱼~变量 '{}' 不存在", obj_var_name)
            ));
        };
        
        // 确保变量是一个对象
        let mut obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                format!("杂鱼~变量 '{}' 不是一个对象", obj_var_name)
            ));
        };
        
        // 获取属性名
        let key = context.resolve_value(&args_array[1]);
        
        // 获取新值
        let new_value = if let Some(text) = args_array[2].as_str() {
            if VariableReference::is_reference(text) {
                if let Some(val) = context.get_value(text) {
                    val.clone()
                } else {
                    Value::String(text.to_string())
                }
            } else {
                Value::String(text.to_string())
            }
        } else {
            args_array[2].clone()
        };
        
        // 设置属性
        obj.insert(key, new_value);
        
        // 更新对象变量
        context.set_variable(obj_var_name.to_string(), Value::Object(obj.clone()))?;
        
        // 将修改后的对象也存储在result变量中
        context.set_variable("result".to_string(), Value::Object(obj))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.set' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// execute_object_has - 检查对象是否有属性
pub fn execute_object_has(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.has' 需要两个参数：对象和属性名".to_string()
            ));
        }
        
        // 获取对象
        let obj_value = if let Some(obj_ref_str) = args_array[0].as_str() {
            if VariableReference::is_reference(obj_ref_str) {
                if let Some(val) = context.get_value(obj_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        format!("杂鱼~变量 '{}' 不存在", obj_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.has' 的第一个参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是对象类型
        let obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.has' 的第一个参数必须是一个对象".to_string()
            ));
        };
        
        // 获取属性名
        let key = context.resolve_value(&args_array[1]);
        
        // 检查属性是否存在
        let has_property = obj.contains_key(&key);
        
        // 存储结果
        context.set_variable("result".to_string(), Value::Bool(has_property))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.has' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// execute_object_keys - 获取对象所有键
pub fn execute_object_keys(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.is_empty() {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.keys' 需要一个参数：对象".to_string()
            ));
        }
        
        // 获取对象
        let obj_value = if let Some(obj_ref_str) = args_array[0].as_str() {
            if VariableReference::is_reference(obj_ref_str) {
                if let Some(val) = context.get_value(obj_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        format!("杂鱼~变量 '{}' 不存在", obj_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.keys' 的参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是对象类型
        let obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.keys' 的参数必须是一个对象".to_string()
            ));
        };
        
        // 获取所有键
        let keys: Vec<Value> = obj.keys()
            .map(|k| Value::String(k.clone()))
            .collect();
        
        // 存储结果
        context.set_variable("result".to_string(), Value::Array(keys))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.keys' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// execute_object_values - 获取对象所有值
pub fn execute_object_values(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.is_empty() {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.values' 需要一个参数：对象".to_string()
            ));
        }
        
        // 获取对象
        let obj_value = if let Some(obj_ref_str) = args_array[0].as_str() {
            if VariableReference::is_reference(obj_ref_str) {
                if let Some(val) = context.get_value(obj_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        format!("杂鱼~变量 '{}' 不存在", obj_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.values' 的参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是对象类型
        let obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.values' 的参数必须是一个对象".to_string()
            ));
        };
        
        // 获取所有值
        let values: Vec<Value> = obj.values().cloned().collect();
        
        // 存储结果
        context.set_variable("result".to_string(), Value::Array(values))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.values' 语句的参数必须是一个数组".to_string()
        ))
    }
}

// execute_object_delete - 删除对象属性
pub fn execute_object_delete(args: &Value, context: &mut Context) -> Result<()> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.delete' 需要两个参数：对象和属性名".to_string()
            ));
        }
        
        // 获取对象变量引用
        let obj_ref = &args_array[0];
        let obj_var_name = if let Some(var_name) = obj_ref.as_str() {
            let var_ref = VariableReference::parse(var_name);
            if var_ref.ref_type == ReferenceType::Variable {
                var_ref.name
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~'object.delete' 的第一个参数必须是一个对象变量引用".to_string()
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                "杂鱼~'object.delete' 的第一个参数必须是一个字符串变量引用".to_string()
            ));
        };
        
        // 获取对象变量 - 使用variable_reference模块
        let var_ref = VariableReference {
            ref_type: ReferenceType::Variable,
            name: obj_var_name.clone(),
        };
        
        let obj_value = if let Some(val) = var_ref.get_value(&context.variables, &context.constants) {
            val.clone()
        } else {
            return Err(InterpreterError::RuntimeError(
                format!("杂鱼~变量 '{}' 不存在", obj_var_name)
            ));
        };
        
        // 确保变量是一个对象
        let mut obj = if let Value::Object(obj) = obj_value {
            obj
        } else {
            return Err(InterpreterError::RuntimeError(
                format!("杂鱼~变量 '{}' 不是一个对象", obj_var_name)
            ));
        };
        
        // 获取属性名
        let key = context.resolve_value(&args_array[1]);
        
        // 删除属性，并获取是否存在该属性
        let had_property = obj.remove(&key).is_some();
        
        // 更新对象变量
        context.set_variable(obj_var_name.to_string(), Value::Object(obj.clone()))?;
        
        // 将结果存储在result变量中
        context.set_variable("result".to_string(), Value::Bool(had_property))?;
        
        Ok(())
    } else {
        Err(InterpreterError::RuntimeError(
            "杂鱼~'object.delete' 语句的参数必须是一个数组".to_string()
        ))
    }
} 
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use super::error::{InterpreterError, Result};

/// 变量引用类型，用于区分不同类型的标识符
#[derive(Debug, PartialEq)]
pub enum ReferenceType {
    Variable,    // @var.
    Parameter,   // @param.
    Constant,    // @const.
    Environment, // @env.
    None         // 不是引用
}

/// 变量引用结构，包含引用类型和引用的名称
#[derive(Debug)]
pub struct VariableReference {
    pub ref_type: ReferenceType,
    pub name: String,
}

impl VariableReference {
    /// 从文本字符串解析变量引用
    pub fn parse(text: &str) -> Self {
        if text.starts_with("@var.") {
            VariableReference {
                ref_type: ReferenceType::Variable,
                name: text[5..].to_string(),
            }
        } else if text.starts_with("$") {
            VariableReference {
                ref_type: ReferenceType::Variable,
                name: text[1..].to_string(),
            }
        } else if text.starts_with("￥") {
            VariableReference {
                ref_type: ReferenceType::Variable,
                name: text.chars().skip(1).collect::<String>(),
            }
        } else if text.starts_with("@param.") {
            VariableReference {
                ref_type: ReferenceType::Parameter,
                name: text[7..].to_string(),
            }
        } else if text.starts_with("@params.") {  // 保持向后兼容
            VariableReference {
                ref_type: ReferenceType::Parameter,
                name: text[8..].to_string(),
            }
        } else if text.starts_with("@const.") {
            VariableReference {
                ref_type: ReferenceType::Constant,
                name: text[7..].to_string(),
            }
        } else if text.starts_with("@env.") {
            VariableReference {
                ref_type: ReferenceType::Environment,
                name: text[5..].to_string(),
            }
        } else {
            VariableReference {
                ref_type: ReferenceType::None,
                name: text.to_string(),
            }
        }
    }
    
    /// 检查文本是否是变量引用
    pub fn is_reference(text: &str) -> bool {
        text.starts_with("@var.") || 
        text.starts_with("$") ||
        text.starts_with("￥") ||
        text.starts_with("@param.") ||
        text.starts_with("@params.") || 
        text.starts_with("@const.") ||
        text.starts_with("@env.")
    }
    
    /// 根据引用类型和名称，从对应的存储中获取值
    pub fn get_value<'a>(&self, 
                       variables: &'a HashMap<String, Value>, 
                       constants: &'a HashMap<String, Value>) -> Option<&'a Value> {
        match self.ref_type {
            ReferenceType::Variable => {
                // 处理复杂的变量路径（嵌套属性或数组索引）
                if self.name.contains('.') || self.name.contains('[') {
                    self.get_nested_value(variables)
                } else {
                    variables.get(&self.name)
                }
            },
            ReferenceType::Parameter => variables.get(&self.name),
            ReferenceType::Constant => constants.get(&self.name),
            ReferenceType::Environment => None, // 环境变量需要特殊处理
            ReferenceType::None => None,
        }
    }
    
    /// 获取完整的值，包括处理环境变量
    pub fn resolve_value(&self, 
                       variables: &HashMap<String, Value>, 
                       constants: &HashMap<String, Value>) -> Value {
        match self.ref_type {
            ReferenceType::Environment => {
                // 获取环境变量值
                match env::var(&self.name) {
                    Ok(value) => {
                        Value::String(value)
                    },
                    Err(_) => {
                        Value::Null
                    }
                }
            },
            ReferenceType::Variable => {
                // 处理嵌套属性访问
                if self.name.contains('.') || self.name.contains('[') {
                    if let Some(val) = self.get_nested_value(variables) {
                        val.clone()
                    } else {
                        Value::Null
                    }
                } else {
                    // 简单变量访问
                    if let Some(val) = variables.get(&self.name) {
                        val.clone()
                    } else {
                        Value::Null
                    }
                }
            },
            _ => {
                // 对于其他类型，使用已有的get_value方法
                if let Some(val) = self.get_value(variables, constants) {
                    val.clone()
                } else {
                    Value::Null
                }
            }
        }
    }
    
    /// 获取完整的值，失败时返回错误而非Null
    pub fn resolve_value_with_error(&self, 
                       variables: &HashMap<String, Value>, 
                       constants: &HashMap<String, Value>) -> Result<Value> {
        match self.ref_type {
            ReferenceType::Environment => {
                // 获取环境变量值
                match env::var(&self.name) {
                    Ok(value) => {
                        Ok(Value::String(value))
                    },
                    Err(_) => {
                        Err(InterpreterError::VariableError(
                            format!("环境变量 '{}' 不存在", self.name)
                        ))
                    }
                }
            },
            ReferenceType::Variable => {
                // 处理嵌套属性访问
                if self.name.contains('.') || self.name.contains('[') {
                    if let Some(val) = self.get_nested_value(variables) {
                        Ok(val.clone())
                    } else {
                        Err(InterpreterError::VariableError(
                            format!("无法访问嵌套属性 '{}'", self.name)
                        ))
                    }
                } else {
                    // 简单变量访问
                    if let Some(val) = variables.get(&self.name) {
                        Ok(val.clone())
                    } else {
                        Err(InterpreterError::VariableError(
                            format!("变量 '{}' 未定义", self.name)
                        ))
                    }
                }
            },
            ReferenceType::Constant => {
                // 常量必须存在
                if let Some(val) = constants.get(&self.name) {
                    Ok(val.clone())
                } else {
                    Err(InterpreterError::VariableError(
                        format!("常量 '{}' 未定义", self.name)
                    ))
                }
            },
            ReferenceType::Parameter => {
                // 参数必须存在
                if let Some(val) = variables.get(&self.name) {
                    Ok(val.clone())
                } else {
                    Err(InterpreterError::VariableError(
                        format!("参数 '{}' 未定义", self.name)
                    ))
                }
            },
            ReferenceType::None => {
                // 不是引用，直接返回文本值
                Ok(Value::String(self.name.clone()))
            }
        }
    }
    
    /// 处理复杂的嵌套属性路径或数组索引访问
    fn get_nested_value<'a>(&self, variables: &'a HashMap<String, Value>) -> Option<&'a Value> {
        // 解析表达式，提取基础变量名
        let first_bracket = self.name.find('[');
        let first_dot = self.name.find('.');
        
        // 确定基础变量名在哪里结束
        let base_end = match (first_bracket, first_dot) {
            (Some(b), Some(d)) => b.min(d),
            (Some(b), None) => b,
            (None, Some(d)) => d,
            (None, None) => self.name.len()
        };
        
        let base_var = &self.name[0..base_end];
        
        // 获取基础变量值
        let base_value = variables.get(base_var)?;
        let mut current_value = base_value;
        
        // 如果没有后续路径，直接返回
        if base_end == self.name.len() {
            return Some(current_value);
        }
        
        // 解析并处理剩余路径
        let path = &self.name[base_end..];
        let mut chars = path.chars().peekable();
        let mut index = base_end;
        
        while index < self.name.len() {
            // 处理点号
            if self.name[index..].starts_with('.') {
                index += 1; // 跳过点号
                
                // 读取属性名
                let mut prop_end = index;
                while prop_end < self.name.len() && 
                      self.name.chars().nth(prop_end) != Some('.') && 
                      self.name.chars().nth(prop_end) != Some('[') {
                    prop_end += 1;
                }
                
                if prop_end == index {
                    return None; // 空属性名
                }
                
                let prop_name = &self.name[index..prop_end];
                
                // 访问对象属性
                if let Some(obj) = current_value.as_object() {
                    current_value = obj.get(prop_name)?;
                } else {
                    return None; // 不是对象，无法访问属性
                }
                
                index = prop_end;
            }
            // 处理方括号（数组索引）
            else if self.name[index..].starts_with('[') {
                index += 1; // 跳过左方括号
                
                // 读取索引
                let mut idx_end = index;
                while idx_end < self.name.len() && self.name.chars().nth(idx_end) != Some(']') {
                    idx_end += 1;
                }
                
                if idx_end >= self.name.len() {
                    return None; // 没有找到右方括号
                }
                
                let idx_str = &self.name[index..idx_end];
                
                // 解析索引
                if let Ok(arr_idx) = idx_str.parse::<usize>() {
                    // 访问数组元素
                    if let Some(arr) = current_value.as_array() {
                        current_value = arr.get(arr_idx)?;
                    } else {
                        return None; // 不是数组，无法索引
                    }
                } else {
                    return None; // 索引不是有效数字
                }
                
                index = idx_end + 1; // 跳过右方括号
            }
            else {
                return None; // 不符合预期的路径格式
            }
        }
        
        Some(current_value)
    }
}

/// 解析变量值，处理字符串中的特殊字符和变量引用
pub fn resolve_variable_value(value: &Value, 
                           variables: &HashMap<String, Value>,
                           constants: &HashMap<String, Value>) -> Value {
    match value {
        Value::String(text) => {
            if VariableReference::is_reference(text) {
                let var_ref = VariableReference::parse(text);
                return var_ref.resolve_value(variables, constants);
            } else {
                Value::String(text.clone())
            }
        },
        _ => value.clone(),
    }
} 
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
        // 安全检查：确保字符串非空
        if text.is_empty() {
            return VariableReference {
                ref_type: ReferenceType::None,
                name: String::new(),
            };
        }
        
        // 使用Unicode字符迭代器
        let mut chars = text.chars();
        
        if text.starts_with("@var.") {
            // 跳过前缀
            for _ in 0..5 {
                chars.next();
            }
            VariableReference {
                ref_type: ReferenceType::Variable,
                name: chars.collect(),
            }
        } else if text.starts_with("$") {
            // 跳过前缀
            chars.next();
            VariableReference {
                ref_type: ReferenceType::Variable,
                name: chars.collect(),
            }
        } else if text.starts_with("￥") {
            // 跳过前缀（使用字符而不是字节）
            chars.next();
            VariableReference {
                ref_type: ReferenceType::Variable,
                name: chars.collect(),
            }
        } else if text.starts_with("@param.") {
            // 跳过前缀
            for _ in 0..7 {
                chars.next();
            }
            VariableReference {
                ref_type: ReferenceType::Parameter,
                name: chars.collect(),
            }
        } else if text.starts_with("@params.") {  // 保持向后兼容
            // 跳过前缀
            for _ in 0..8 {
                chars.next();
            }
            VariableReference {
                ref_type: ReferenceType::Parameter,
                name: chars.collect(),
            }
        } else if text.starts_with("@const.") {
            // 跳过前缀
            for _ in 0..7 {
                chars.next();
            }
            VariableReference {
                ref_type: ReferenceType::Constant,
                name: chars.collect(),
            }
        } else if text.starts_with("@env.") {
            // 跳过前缀
            for _ in 0..5 {
                chars.next();
            }
            VariableReference {
                ref_type: ReferenceType::Environment,
                name: chars.collect(),
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
        // 如果开启调试模式，输出调试信息
        if crate::is_debug_mode() {
            println!("解析嵌套属性: {}", self.name);
        }

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
        
        if crate::is_debug_mode() {
            println!("基础变量名: {}", base_var);
        }
        
        // 获取基础变量值
        let base_value = variables.get(base_var)?;
        let mut current_value = base_value;
        
        // 如果没有后续路径，直接返回
        if base_end == self.name.len() {
            return Some(current_value);
        }
        
        // 解析并处理剩余路径
        let path = &self.name[base_end..];
        
        if crate::is_debug_mode() {
            println!("属性路径: {}", path);
        }

        let mut index = base_end;
        
        while index < self.name.len() {
            // 处理点号访问属性
            if index < self.name.len() && self.name[index..].starts_with('.') {
                index += 1; // 跳过点号
                
                // 读取属性名
                let mut prop_end = index;
                while prop_end < self.name.len() && 
                      !self.name[prop_end..].starts_with('.') && 
                      !self.name[prop_end..].starts_with('[') {
                    prop_end += 1;
                }
                
                if prop_end == index {
                    if crate::is_debug_mode() {
                        println!("错误: 空属性名");
                    }
                    return None; // 空属性名
                }
                
                let prop_name = &self.name[index..prop_end];
                
                if crate::is_debug_mode() {
                    println!("访问属性: {}", prop_name);
                }
                
                // 特殊处理数组的length属性
                if prop_name == "length" && current_value.is_array() {
                    if let Some(arr) = current_value.as_array() {
                        // 创建一个静态的值来代表数组长度
                        // 注意：这是一个技巧，实际应该创建一个新的Value，但这里为简单起见
                        static mut ARRAY_LENGTH: Option<Value> = None;
                        
                        // 安全地访问并更新静态变量
                        unsafe {
                            ARRAY_LENGTH = Some(Value::Number(serde_json::Number::from(arr.len())));
                            if let Some(ref length_value) = ARRAY_LENGTH {
                                return Some(length_value);
                            }
                        }
                    }
                }
                
                // 普通对象属性访问
                if let Some(obj) = current_value.as_object() {
                    if let Some(prop_value) = obj.get(prop_name) {
                        current_value = prop_value;
                    } else {
                        if crate::is_debug_mode() {
                            println!("错误: 对象中不存在属性 '{}'", prop_name);
                        }
                        return None; // 属性不存在
                    }
                } else {
                    if crate::is_debug_mode() {
                        println!("错误: 不是对象，无法访问属性");
                    }
                    return None; // 不是对象，无法访问属性
                }
                
                index = prop_end;
            }
            // 处理方括号（数组索引）
            else if index < self.name.len() && self.name[index..].starts_with('[') {
                index += 1; // 跳过左方括号
                
                // 读取索引
                let mut idx_end = index;
                while idx_end < self.name.len() && !self.name[idx_end..].starts_with(']') {
                    idx_end += 1;
                }
                
                if idx_end >= self.name.len() {
                    if crate::is_debug_mode() {
                        println!("错误: 没有找到右方括号");
                    }
                    return None; // 没有找到右方括号
                }
                
                let idx_str = &self.name[index..idx_end];
                
                if crate::is_debug_mode() {
                    println!("数组索引: {}", idx_str);
                }
                
                // 解析索引
                if let Ok(arr_idx) = idx_str.parse::<usize>() {
                    // 访问数组元素
                    if let Some(arr) = current_value.as_array() {
                        if let Some(arr_value) = arr.get(arr_idx) {
                            current_value = arr_value;
                        } else {
                            if crate::is_debug_mode() {
                                println!("错误: 数组索引越界 {}", arr_idx);
                            }
                            return None; // 索引越界
                        }
                    } else {
                        if crate::is_debug_mode() {
                            println!("错误: 不是数组，无法索引");
                        }
                        return None; // 不是数组，无法索引
                    }
                } else {
                    // 索引不是数字，可能是变量引用或其他
                    if crate::is_debug_mode() {
                        println!("警告: 索引不是有效数字: {}", idx_str);
                    }
                    return None; // 索引不是有效数字
                }
                
                index = idx_end + 1; // 跳过右方括号
            }
            else {
                if crate::is_debug_mode() {
                    println!("错误: 不符合预期的路径格式");
                }
                return None; // 不符合预期的路径格式
            }
        }
        
        if crate::is_debug_mode() {
            println!("成功获取嵌套值: {:?}", current_value);
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

/// 解析包含嵌套变量引用的表达式，例如 @var.array[@var.index] 或 @var.user.@var.key
pub fn resolve_nested_variable_reference(text: &str, variables: &HashMap<String, Value>, constants: &HashMap<String, Value>) -> Result<String> {
    // 如果文本中不包含变量引用，直接返回
    if !text.contains('@') && !text.contains('$') && !text.contains('￥') {
        return Ok(text.to_string());
    }
    
    if crate::is_debug_mode() {
        println!("解析嵌套变量引用表达式: {}", text);
    }
    
    // 处理带变量索引的数组访问: @var.array[@var.index]
    if let Some(open_bracket) = text.find('[') {
        if let Some(close_bracket) = text.find(']') {
            if open_bracket < close_bracket {
                let base_ref = &text[0..open_bracket];
                let index_ref = &text[open_bracket+1..close_bracket];
                
                if crate::is_debug_mode() {
                    println!("找到嵌套引用 - 基础部分: {}, 索引部分: {}", base_ref, index_ref);
                }
                
                // 解析基础数组引用
                let var_ref_base = VariableReference::parse(base_ref);
                if var_ref_base.ref_type == ReferenceType::None {
                    if crate::is_debug_mode() {
                        println!("基础部分不是有效的变量引用");
                    }
                    return Ok(text.to_string());
                }
                
                // 获取数组对象或对象
                let base_value = var_ref_base.resolve_value_with_error(variables, constants)?;
                
                // 检查对象后是否还有其他访问 - 例如 @var.person.skills[@var.index].name
                let remaining_text = if close_bracket + 1 < text.len() {
                    &text[close_bracket+1..]
                } else {
                    ""
                };
                
                // 解析索引值 - 可能是变量引用 或 数字
                let index_value = if index_ref.contains('@') || index_ref.contains('$') || index_ref.contains('￥') {
                    // 索引是变量引用
                    let var_ref_index = VariableReference::parse(index_ref);
                    let index_result = var_ref_index.resolve_value_with_error(variables, constants)?;
                    
                    // 转换为数字
                    match index_result {
                        Value::Number(n) => {
                            if let Some(idx) = n.as_u64() {
                                idx as usize
                            } else {
                                return Err(InterpreterError::VariableError(
                                    format!("无法将索引值 '{}' 转换为有效数组索引", index_ref)
                                ));
                            }
                        },
                        Value::String(s) => {
                            if let Ok(idx) = s.parse::<usize>() {
                                idx
                            } else {
                                // 如果不是数字，可能是字符串索引（用于对象）
                                return Ok(s);
                            }
                        },
                        _ => {
                            return Err(InterpreterError::VariableError(
                                format!("索引值类型错误，不能用作数组索引")
                            ));
                        }
                    }
                } else {
                    // 索引是数字
                    if let Ok(idx) = index_ref.parse::<usize>() {
                        idx
                    } else {
                        return Err(InterpreterError::VariableError(
                            format!("无法将索引值 '{}' 转换为有效数组索引", index_ref)
                        ));
                    }
                };
                
                if crate::is_debug_mode() {
                    println!("解析出的索引值: {}", index_value);
                }
                
                // 访问数组元素或对象属性
                if let Some(arr) = base_value.as_array() {
                    if index_value < arr.len() {
                        // 获取数组元素
                        let element = &arr[index_value];
                        
                        // 检查是否有后续访问
                        if !remaining_text.is_empty() {
                            if remaining_text.starts_with('.') {
                                // 尝试访问数组元素的属性
                                let prop_path = remaining_text[1..].to_string(); // 去掉点号
                                
                                if crate::is_debug_mode() {
                                    println!("尝试访问数组元素的属性: {}", prop_path);
                                }
                                
                                if element.is_object() {
                                    if let Some(obj) = element.as_object() {
                                        // 分割属性路径
                                        let parts: Vec<&str> = prop_path.split('.').collect();
                                        
                                        // 递归访问属性路径
                                        let mut current = element;
                                        for part in parts {
                                            // 检查是否是嵌套变量引用
                                            if part.starts_with('@') {
                                                let var_ref = VariableReference::parse(part);
                                                let prop_name = var_ref.resolve_value_with_error(variables, constants)?;
                                                
                                                if let Value::String(prop_str) = prop_name {
                                                    if let Some(obj) = current.as_object() {
                                                        if let Some(prop) = obj.get(&prop_str) {
                                                            current = prop;
                                                        } else {
                                                            return Err(InterpreterError::VariableError(
                                                                format!("对象中不存在属性 '{}'", prop_str)
                                                            ));
                                                        }
                                                    } else {
                                                        return Err(InterpreterError::VariableError(
                                                            format!("不是对象，无法访问属性 '{}'", prop_str)
                                                        ));
                                                    }
                                                } else {
                                                    return Err(InterpreterError::VariableError(
                                                        format!("属性名必须是字符串，但得到了 {:?}", prop_name)
                                                    ));
                                                }
                                            } else {
                                                if let Some(obj) = current.as_object() {
                                                    if let Some(prop) = obj.get(part) {
                                                        current = prop;
                                                    } else {
                                                        return Err(InterpreterError::VariableError(
                                                            format!("对象中不存在属性 '{}'", part)
                                                        ));
                                                    }
                                                } else {
                                                    return Err(InterpreterError::VariableError(
                                                        format!("不是对象，无法访问属性 '{}'", part)
                                                    ));
                                                }
                                            }
                                        }
                                        
                                        if crate::is_debug_mode() {
                                            println!("成功获取属性值: {:?}", current);
                                        }
                                        
                                        return Ok(current.to_string());
                                    }
                                }
                                
                                return Err(InterpreterError::VariableError(
                                    format!("数组元素不是对象，无法访问属性 '{}'", prop_path)
                                ));
                            } else if remaining_text.starts_with('[') {
                                // 递归解析剩余部分
                                let new_ref = format!("@var._temp{}", remaining_text);
                                
                                // 创建临时变量
                                let mut temp_variables = variables.clone();
                                temp_variables.insert("_temp".to_string(), element.clone());
                                
                                return resolve_nested_variable_reference(&new_ref, &temp_variables, constants);
                            }
                        }
                        
                        if crate::is_debug_mode() {
                            println!("成功获取数组元素: {:?}", element);
                        }
                        
                        return Ok(element.to_string());
                    } else {
                        return Err(InterpreterError::VariableError(
                            format!("数组索引 {} 超出范围 (0-{})", index_value, arr.len()-1)
                        ));
                    }
                } else if let Some(obj) = base_value.as_object() {
                    // 尝试用字符串作为键访问对象
                    let key = if let Ok(s) = index_value.to_string().parse::<String>() {
                        s
                    } else {
                        index_value.to_string()
                    };
                    
                    if let Some(value) = obj.get(&key) {
                        if crate::is_debug_mode() {
                            println!("使用键 {} 访问对象属性", key);
                        }
                        
                        // 处理剩余访问路径
                        if !remaining_text.is_empty() {
                            // 类似上面数组元素属性访问
                            let new_ref = format!("@var._temp{}", remaining_text);
                            
                            // 创建临时变量
                            let mut temp_variables = variables.clone();
                            temp_variables.insert("_temp".to_string(), value.clone());
                            
                            return resolve_nested_variable_reference(&new_ref, &temp_variables, constants);
                        }
                        
                        return Ok(value.to_string());
                    } else {
                        return Err(InterpreterError::VariableError(
                            format!("对象中不存在键 '{}'", key)
                        ));
                    }
                } else {
                    return Err(InterpreterError::VariableError(
                        format!("变量 '{}' 不是数组或对象，无法使用索引访问", base_ref)
                    ));
                }
            }
        }
    }
    
    // 处理对象属性中的变量引用: @var.user.@var.key
    if let Some(dot_pos) = text.find('.') {
        let parts: Vec<&str> = text.split('.').collect();
        if parts.len() > 2 {
            // 检查是否含有变量引用的属性名
            let mut has_var_ref = false;
            for (i, part) in parts.iter().enumerate() {
                if i > 1 && (part.starts_with('@') || part.starts_with('$') || part.starts_with('￥')) {
                    has_var_ref = true;
                    break;
                }
            }
            
            if has_var_ref {
                if crate::is_debug_mode() {
                    println!("发现属性访问中的变量引用");
                }
                
                // 解析基础对象
                let base_parts = parts[0..2].join(".");
                let var_ref = VariableReference::parse(&base_parts);
                if var_ref.ref_type == ReferenceType::None {
                    if crate::is_debug_mode() {
                        println!("基础部分不是有效的变量引用");
                    }
                    return Ok(text.to_string());
                }
                
                // 获取基础对象
                let mut current_value = var_ref.resolve_value_with_error(variables, constants)?;
                
                // 逐层访问属性
                for i in 2..parts.len() {
                    let part = parts[i];
                    
                    // 处理变量引用属性名
                    if part.starts_with('@') || part.starts_with('$') || part.starts_with('￥') {
                        let prop_ref = VariableReference::parse(part);
                        let prop_name = prop_ref.resolve_value_with_error(variables, constants)?;
                        
                        if let Value::String(prop_str) = prop_name {
                            if let Some(obj) = current_value.as_object() {
                                if let Some(prop_value) = obj.get(&prop_str) {
                                    current_value = prop_value.clone();
                                } else {
                                    return Err(InterpreterError::VariableError(
                                        format!("对象中不存在属性 '{}'", prop_str)
                                    ));
                                }
                            } else {
                                return Err(InterpreterError::VariableError(
                                    format!("不是对象，无法访问属性")
                                ));
                            }
                        } else {
                            return Err(InterpreterError::VariableError(
                                format!("属性名引用必须解析为字符串，得到了 {:?}", prop_name)
                            ));
                        }
                    } else {
                        // 常规属性访问
                        if let Some(obj) = current_value.as_object() {
                            if let Some(prop_value) = obj.get(part) {
                                current_value = prop_value.clone();
                            } else {
                                return Err(InterpreterError::VariableError(
                                    format!("对象中不存在属性 '{}'", part)
                                ));
                            }
                        } else {
                            return Err(InterpreterError::VariableError(
                                format!("不是对象，无法访问属性")
                            ));
                        }
                    }
                }
                
                if crate::is_debug_mode() {
                    println!("成功解析变量属性访问，结果: {:?}", current_value);
                }
                
                return Ok(current_value.to_string());
            }
        }
    }
    
    // 如果没有复杂嵌套结构，可能是普通变量引用
    if VariableReference::is_reference(text) {
        let var_ref = VariableReference::parse(text);
        let result = var_ref.resolve_value_with_error(variables, constants)?;
        return Ok(result.to_string());
    }
    
    // 不是嵌套引用格式，按字面文本返回
    Ok(text.to_string())
} 
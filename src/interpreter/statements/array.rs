use serde_json::Value;
use super::super::context::Context;
use super::super::error::{InterpreterError, Result};
use super::super::error::error_messages::statement::{self, array};
use super::super::variable_reference::{VariableReference, ReferenceType};
use super::store_result_with_compatibility;

// execute_array_create - 创建新数组
pub fn execute_array_create(args: &Value, context: &mut Context) -> Result<Value> {
    let result = if let Some(args_array) = args.as_array() {
        // 如果提供了初始元素，则使用它们创建数组
        // 处理每个元素，解析变量引用
        let resolved_elements: Vec<Value> = args_array.iter()
            .map(|elem| {
                if let Some(text) = elem.as_str() {
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
                    elem.clone()
                }
            })
            .collect();
        
        Value::Array(resolved_elements)
    } else if let Some(obj) = args.as_object() {
        if let Some(size) = obj.get("size") {
            // 如果提供了size参数，则创建指定大小的空数组
            let size_value = context.resolve_value(size);
            if let Ok(size) = size_value.parse::<usize>() {
                // 如果提供了初始值，则使用它填充数组
                let initial_value = if let Some(init) = obj.get("initial") {
                    if let Some(text) = init.as_str() {
                        if VariableReference::is_reference(text) {
                            if let Some(val) = context.get_value(text) {
                                val.clone()
                            } else {
                                Value::Null
                            }
                        } else {
                            Value::String(text.to_string())
                        }
                    } else {
                        init.clone()
                    }
                } else {
                    Value::Null
                };
                
                let elements = vec![initial_value; size];
                Value::Array(elements)
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::CREATE_SIZE_NOT_NUMBER.to_string()
                ));
            }
        } else {
            // 如果没有指定size，则创建一个空数组
            Value::Array(Vec::new())
        }
    } else {
        // 如果参数不是数组也不是对象，则创建一个空数组
        Value::Array(Vec::new())
    };
    
    // 存储结果
    store_result_with_compatibility(args, &result, context)?;
    Ok(result)
}

// execute_array_push - 向数组末尾添加元素
pub fn execute_array_push(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                array::PUSH_MISSING_ARGS.to_string()
            ));
        }
        
        // 第一个参数是数组
        let array_ref = &args_array[0];
        let array_var_name = if let Some(var_name) = array_ref.as_str() {
            let var_ref = VariableReference::parse(var_name);
            if var_ref.ref_type == ReferenceType::Variable {
                var_ref.name
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::PUSH_FIRST_ARG_NOT_ARRAY_REF.to_string()
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                array::PUSH_FIRST_ARG_NOT_STRING_REF.to_string()
            ));
        };
        
        // 获取数组变量 - 使用variable_reference模块
        let var_ref = VariableReference {
            ref_type: ReferenceType::Variable,
            name: array_var_name.clone(),
        };
        
        let array_value = if let Some(val) = var_ref.get_value(&context.variables, &context.constants) {
            val.clone()
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_found(&array_var_name)
            ));
        };
        
        // 确保变量是一个数组
        let mut array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_array(&array_var_name)
            ));
        };
        
        // 添加其余的参数到数组
        for item in &args_array[1..] {
            let resolved_item = if let Some(text) = item.as_str() {
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
                item.clone()
            };
            
            array.push(resolved_item);
        }
        
        // 更新数组变量
        context.set_variable(array_var_name, Value::Array(array.clone()))?;
        
        // 将修改后的数组作为结果
        let result = Value::Array(array);
        
        // 存储结果
        store_result_with_compatibility(args, &result, context)?;
        Ok(result)
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.push")
        ))
    }
}

// execute_array_pop - 从数组末尾移除元素
pub fn execute_array_pop(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(args_array) = args.as_array() {
        if args_array.is_empty() {
            return Err(InterpreterError::RuntimeError(
                array::POP_MISSING_ARGS.to_string()
            ));
        }
        
        // 获取数组变量引用
        let array_ref = &args_array[0];
        let array_var_name = if let Some(var_name) = array_ref.as_str() {
            let var_ref = VariableReference::parse(var_name);
            if var_ref.ref_type == ReferenceType::Variable {
                var_ref.name
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::POP_ARG_NOT_ARRAY_REF.to_string()
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                array::POP_ARG_NOT_STRING_REF.to_string()
            ));
        };
        
        // 获取数组变量 - 使用variable_reference模块
        let var_ref = VariableReference {
            ref_type: ReferenceType::Variable,
            name: array_var_name.clone(),
        };
        
        let array_value = if let Some(val) = var_ref.get_value(&context.variables, &context.constants) {
            val.clone()
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_found(&array_var_name)
            ));
        };
        
        // 确保变量是一个数组
        let mut array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_array(&array_var_name)
            ));
        };
        
        // 从数组末尾移除元素
        let popped = if !array.is_empty() {
            array.pop().unwrap()
        } else {
            Value::Null
        };
        
        // 更新数组变量
        context.set_variable(array_var_name.to_string(), Value::Array(array))?;
        
        // 将弹出的元素作为结果
        // 存储结果
        store_result_with_compatibility(args, &popped, context)?;
        Ok(popped)
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.pop")
        ))
    }
}

// execute_array_get - 获取数组指定索引的元素
pub fn execute_array_get(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                array::GET_MISSING_ARGS.to_string()
            ));
        }
        
        // 获取数组
        let array_value = if let Some(array_ref_str) = args_array[0].as_str() {
            if VariableReference::is_reference(array_ref_str) {
                if let Some(val) = context.get_value(array_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        array::var_not_found(array_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::GET_FIRST_ARG_NOT_ARRAY_REF.to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是数组类型
        let array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::GET_FIRST_ARG_NOT_ARRAY.to_string()
            ));
        };
        
        // 获取索引
        let index_value = context.resolve_value(&args_array[1]);
        let index = if let Ok(idx) = index_value.parse::<usize>() {
            idx
        } else {
            // 尝试直接从变量中获取值
            if let Some(index_str) = args_array[1].as_str() {
                if VariableReference::is_reference(index_str) {
                    if let Some(val) = context.get_value(index_str) {
                        if let Some(num) = val.as_u64() {
                            num as usize
                        } else if let Some(num) = val.as_i64() {
                            num as usize
                        } else if let Some(num) = val.as_f64() {
                            num as usize
                        } else {
                            return Err(InterpreterError::RuntimeError(
                                array::GET_SECOND_ARG_NOT_INDEX.to_string()
                            ));
                        }
                    } else {
                        return Err(InterpreterError::RuntimeError(
                            array::var_not_found(index_str)
                        ));
                    }
                } else {
                    return Err(InterpreterError::RuntimeError(
                        array::GET_SECOND_ARG_NOT_INDEX.to_string()
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::GET_SECOND_ARG_NOT_INDEX.to_string()
                ));
            }
        };
        
        // 获取元素
        let element = if index < array.len() {
            array[index].clone()
        } else {
            Value::Null
        };
        
        // 存储结果
        store_result_with_compatibility(args, &element, context)?;
        Ok(element)
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.get")
        ))
    }
}

// execute_array_set - 设置数组指定索引的元素
pub fn execute_array_set(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 3 {
            return Err(InterpreterError::RuntimeError(
                array::SET_MISSING_ARGS.to_string()
            ));
        }
        
        // 获取数组变量引用
        let array_ref = &args_array[0];
        let array_var_name = if let Some(var_name) = array_ref.as_str() {
            let var_ref = VariableReference::parse(var_name);
            if var_ref.ref_type == ReferenceType::Variable {
                var_ref.name
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::set_first_arg_not_array_ref(var_name)
                ));
            }
        } else {
            return Err(InterpreterError::RuntimeError(
                array::set_first_arg_not_string_ref()
            ));
        };
        
        // 获取数组变量 - 使用variable_reference模块
        let var_ref = VariableReference {
            ref_type: ReferenceType::Variable,
            name: array_var_name.clone(),
        };
        
        let array_value = if let Some(val) = var_ref.get_value(&context.variables, &context.constants) {
            val.clone()
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_found(&array_var_name)
            ));
        };
        
        // 确保变量是一个数组
        let mut array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::var_not_array(&array_var_name)
            ));
        };
        
        // 获取索引
        let index_value = context.resolve_value(&args_array[1]);
        let index = if let Ok(idx) = index_value.parse::<usize>() {
            idx
        } else {
            return Err(InterpreterError::RuntimeError(
                array::set_second_arg_must_be_number()
            ));
        };
        
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
        
        // 如果索引超出范围，扩展数组
        if index >= array.len() {
            array.resize(index + 1, Value::Null);
        }
        
        // 设置元素
        array[index] = new_value;
        
        // 更新数组变量
        context.set_variable(array_var_name.to_string(), Value::Array(array.clone()))?;
        
        // 将修改后的数组作为结果
        let result = Value::Array(array);
        
        // 存储结果
        store_result_with_compatibility(args, &result, context)?;
        Ok(result)
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.set")
        ))
    }
}

// execute_array_length - 获取数组长度
pub fn execute_array_length(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(args_array) = args.as_array() {
        if args_array.is_empty() {
            return Err(InterpreterError::RuntimeError(
                array::LENGTH_MISSING_ARGS.to_string()
            ));
        }
        
        // 获取数组
        let array_value = if let Some(array_ref_str) = args_array[0].as_str() {
            if VariableReference::is_reference(array_ref_str) {
                if let Some(val) = context.get_value(array_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        array::var_not_found(array_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::LENGTH_ARG_NOT_ARRAY_REF.to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是数组类型
        let array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::LENGTH_ARG_NOT_ARRAY.to_string()
            ));
        };
        
        // 获取长度
        let length = array.len();
        let result = Value::Number(serde_json::Number::from(length));
        
        // 存储结果
        store_result_with_compatibility(args, &result, context)?;
        Ok(result)
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.length")
        ))
    }
}

// execute_array_slice - 获取数组切片
pub fn execute_array_slice(args: &Value, context: &mut Context) -> Result<Value> {
    if let Some(args_array) = args.as_array() {
        if args_array.len() < 2 {
            return Err(InterpreterError::RuntimeError(
                array::SLICE_MISSING_ARGS.to_string()
            ));
        }
        
        // 获取数组
        let array_value = if let Some(array_ref_str) = args_array[0].as_str() {
            if VariableReference::is_reference(array_ref_str) {
                if let Some(val) = context.get_value(array_ref_str) {
                    val.clone()
                } else {
                    return Err(InterpreterError::RuntimeError(
                        array::var_not_found(array_ref_str)
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::SLICE_FIRST_ARG_NOT_ARRAY_REF.to_string()
                ));
            }
        } else {
            args_array[0].clone()
        };
        
        // 确保是数组类型
        let array = if let Value::Array(arr) = array_value {
            arr
        } else {
            return Err(InterpreterError::RuntimeError(
                array::SLICE_FIRST_ARG_NOT_ARRAY.to_string()
            ));
        };
        
        // 获取开始索引
        let start_value = context.resolve_value(&args_array[1]);
        let start = if let Ok(idx) = start_value.parse::<usize>() {
            idx
        } else {
            return Err(InterpreterError::RuntimeError(
                array::SLICE_SECOND_ARG_NOT_INDEX.to_string()
            ));
        };
        
        // 获取结束索引（如果提供）
        let end = if args_array.len() > 2 {
            let end_value = context.resolve_value(&args_array[2]);
            if let Ok(idx) = end_value.parse::<usize>() {
                idx
            } else {
                return Err(InterpreterError::RuntimeError(
                    array::SLICE_THIRD_ARG_NOT_INDEX.to_string()
                ));
            }
        } else {
            array.len()
        };
        
        // 创建切片
        let slice = if start <= array.len() {
            let actual_end = end.min(array.len());
            if start <= actual_end {
                array[start..actual_end].to_vec()
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        };
        
        let result = Value::Array(slice);
        
        // 存储结果
        store_result_with_compatibility(args, &result, context)?;
        Ok(result)
    } else {
        Err(InterpreterError::RuntimeError(
            statement::param_must_be_array("array.slice")
        ))
    }
} 
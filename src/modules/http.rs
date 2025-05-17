use serde_json::{Value, json};
use reqwest::{blocking, header};
use std::collections::HashMap;
use std::time::Duration;
use crate::interpreter::context::Context;
use super::Module;

pub struct HttpModule;

impl HttpModule {
    pub fn new() -> Self {
        HttpModule
    }

    // GET 请求
    fn get(args: &[Value], context: &mut Context) -> Value {
        if args.is_empty() {
            return json!({
                "error": "URL参数缺失"
            });
        }

        // 解析URL
        let url = match &args[0] {
            Value::String(s) if s.starts_with("@") => {
                // 这是一个变量引用
                if let Some(val) = context.get_value(s) {
                    match val {
                        Value::String(url) => url.clone(),
                        _ => val.to_string()
                    }
                } else {
                    return json!({
                        "error": format!("变量 '{}' 不存在", s)
                    });
                }
            },
            _ => args[0].to_string().trim_matches('"').to_string()
        };
        
        // 获取可选的请求头
        let headers_map = if args.len() > 1 {
            Self::extract_headers(&args[1], context)
        } else {
            HashMap::new()
        };
        
        // 获取可选的超时设置（单位：秒）
        let timeout = if args.len() > 2 {
            Self::extract_number(&args[2], context)
        } else {
            None
        };

        // 执行GET请求
        Self::execute_request("GET", &url, None, headers_map, timeout)
    }

    // POST 请求
    fn post(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 2 {
            return json!({
                "error": "缺少URL或请求体参数"
            });
        }

        // 解析URL
        let url = match &args[0] {
            Value::String(s) if s.starts_with("@") => {
                // 这是一个变量引用
                if let Some(val) = context.get_value(s) {
                    match val {
                        Value::String(url) => url.clone(),
                        _ => val.to_string()
                    }
                } else {
                    return json!({
                        "error": format!("变量 '{}' 不存在", s)
                    });
                }
            },
            _ => args[0].to_string().trim_matches('"').to_string()
        };
        
        // 获取请求体
        let body = Self::extract_value(&args[1], context);
        
        // 获取可选的请求头
        let headers_map = if args.len() > 2 {
            Self::extract_headers(&args[2], context)
        } else {
            HashMap::new()
        };
        
        // 获取可选的超时设置（单位：秒）
        let timeout = if args.len() > 3 {
            Self::extract_number(&args[3], context)
        } else {
            None
        };

        // 执行POST请求
        Self::execute_request("POST", &url, Some(body), headers_map, timeout)
    }

    // PUT 请求
    fn put(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 2 {
            return json!({
                "error": "缺少URL或请求体参数"
            });
        }

        // 解析URL
        let url = match &args[0] {
            Value::String(s) if s.starts_with("@") => {
                // 这是一个变量引用
                if let Some(val) = context.get_value(s) {
                    match val {
                        Value::String(url) => url.clone(),
                        _ => val.to_string()
                    }
                } else {
                    return json!({
                        "error": format!("变量 '{}' 不存在", s)
                    });
                }
            },
            _ => args[0].to_string().trim_matches('"').to_string()
        };
        
        // 获取请求体
        let body = Self::extract_value(&args[1], context);
        
        // 获取可选的请求头
        let headers_map = if args.len() > 2 {
            Self::extract_headers(&args[2], context)
        } else {
            HashMap::new()
        };
        
        // 获取可选的超时设置（单位：秒）
        let timeout = if args.len() > 3 {
            Self::extract_number(&args[3], context)
        } else {
            None
        };

        // 执行PUT请求
        Self::execute_request("PUT", &url, Some(body), headers_map, timeout)
    }

    // DELETE 请求
    fn delete(args: &[Value], context: &mut Context) -> Value {
        if args.is_empty() {
            return json!({
                "error": "URL参数缺失"
            });
        }

        // 解析URL
        let url = match &args[0] {
            Value::String(s) if s.starts_with("@") => {
                // 这是一个变量引用
                if let Some(val) = context.get_value(s) {
                    match val {
                        Value::String(url) => url.clone(),
                        _ => val.to_string()
                    }
                } else {
                    return json!({
                        "error": format!("变量 '{}' 不存在", s)
                    });
                }
            },
            _ => args[0].to_string().trim_matches('"').to_string()
        };
        
        // 获取可选的请求头
        let headers_map = if args.len() > 1 {
            Self::extract_headers(&args[1], context)
        } else {
            HashMap::new()
        };
        
        // 获取可选的超时设置（单位：秒）
        let timeout = if args.len() > 2 {
            Self::extract_number(&args[2], context)
        } else {
            None
        };

        // 执行DELETE请求
        Self::execute_request("DELETE", &url, None, headers_map, timeout)
    }
    
    // 从参数中提取值
    fn extract_value(value: &Value, context: &Context) -> Value {
        match value {
            Value::String(s) if s.starts_with("@") => {
                // 这是一个变量引用
                if let Some(val) = context.get_value(s) {
                    val.clone()
                } else {
                    Value::Null
                }
            },
            _ => value.clone()
        }
    }
    
    // 从参数中提取请求头
    fn extract_headers(headers_value: &Value, context: &Context) -> HashMap<String, String> {
        let mut headers_map = HashMap::new();
        
        // 尝试获取头部变量
        let headers_obj = match headers_value {
            Value::String(s) if s.starts_with("@") => {
                // 这是一个变量引用
                if let Some(val) = context.get_value(s) {
                    val
                } else {
                    return headers_map;
                }
            },
            _ => headers_value.clone()
        };
        
        // 检查是否为对象类型
        if let Value::Object(obj) = headers_obj {
            for (key, value) in obj {
                let value_str = match value {
                    Value::String(s) => s,
                    _ => value.to_string()
                };
                headers_map.insert(key, value_str);
            }
        }
        
        headers_map
    }
    
    // 从参数中提取数字
    fn extract_number(value: &Value, context: &Context) -> Option<f64> {
        match value {
            Value::String(s) if s.starts_with("@") => {
                // 这是一个变量引用
                if let Some(val) = context.get_value(s) {
                    match val {
                        Value::Number(n) => n.as_f64(),
                        Value::String(s) => s.parse::<f64>().ok(),
                        _ => None
                    }
                } else {
                    None
                }
            },
            Value::Number(n) => n.as_f64(),
            Value::String(s) => s.parse::<f64>().ok(),
            _ => None
        }
    }
    
    // 通用请求执行函数
    fn execute_request(method: &str, url: &str, body: Option<Value>, headers: HashMap<String, String>, timeout: Option<f64>) -> Value {
        // 创建客户端
        let client_builder = blocking::Client::builder();
        
        // 设置超时
        let client_builder = if let Some(t) = timeout {
            client_builder.timeout(Duration::from_secs_f64(t))
        } else {
            client_builder
        };
        
        // 构建客户端
        let client = match client_builder.build() {
            Ok(client) => client,
            Err(e) => return json!({
                "error": format!("创建HTTP客户端失败: {}", e)
            })
        };
        
        // 创建请求
        let mut request_builder = match method {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            _ => return json!({
                "error": format!("不支持的HTTP方法: {}", method)
            })
        };
        
        // 添加请求头
        for (key, value) in headers {
            request_builder = request_builder.header(key, value);
        }
        
        // 添加请求体
        if let Some(body_value) = body {
            // 根据类型设置适当的请求体
            if body_value.is_object() || body_value.is_array() {
                // JSON类型
                let json_str = match serde_json::to_string(&body_value) {
                    Ok(s) => s,
                    Err(e) => return json!({
                        "error": format!("序列化请求体失败: {}", e)
                    })
                };
                request_builder = request_builder.header(header::CONTENT_TYPE, "application/json").body(json_str);
            } else {
                // 字符串类型
                request_builder = request_builder.body(body_value.to_string());
            }
        }
        
        // 执行请求
        let response = match request_builder.send() {
            Ok(resp) => resp,
            Err(e) => return json!({
                "error": format!("HTTP请求失败: {}", e)
            })
        };
        
        // 解析响应
        let status = response.status().as_u16();
        let headers = response.headers().clone();
        
        // 提取响应头
        let mut header_map = json!({});
        for (name, value) in headers.iter() {
            if let Ok(value_str) = value.to_str() {
                header_map[name.as_str()] = json!(value_str);
            }
        }
        
        // 尝试解析JSON响应体
        match response.text() {
            Ok(text) => {
                match serde_json::from_str::<Value>(&text) {
                    Ok(json_data) => {
                        // 成功解析为JSON
                        json!({
                            "status": status,
                            "headers": header_map,
                            "body": json_data,
                            "raw": text
                        })
                    },
                    Err(_) => {
                        // 无法解析为JSON，返回原始文本
                        json!({
                            "status": status,
                            "headers": header_map,
                            "body": text,
                            "raw": text
                        })
                    }
                }
            },
            Err(e) => json!({
                "error": format!("读取响应失败: {}", e),
                "status": status,
                "headers": header_map
            })
        }
    }
    
    // URL解码
    fn url_decode(args: &[Value], context: &mut Context) -> Value {
        if args.is_empty() {
            return Value::String("".to_string());
        }
        
        // 获取需要解码的字符串
        let encoded = match &args[0] {
            Value::String(s) if s.starts_with("@") => {
                // 这是一个变量引用
                if let Some(val) = context.get_value(s) {
                    match val {
                        Value::String(str) => str.clone(),
                        _ => val.to_string()
                    }
                } else {
                    return json!({
                        "error": format!("变量 '{}' 不存在", s)
                    });
                }
            },
            Value::String(s) => s.clone(),
            _ => args[0].to_string()
        };
        
        let encoded = encoded.trim_matches('"');
        
        match urlencoding::decode(encoded) {
            Ok(decoded) => Value::String(decoded.into_owned()),
            Err(e) => json!({
                "error": format!("URL解码失败: {}", e)
            })
        }
    }
    
    // URL编码
    fn url_encode(args: &[Value], context: &mut Context) -> Value {
        if args.is_empty() {
            return Value::String("".to_string());
        }
        
        // 获取需要编码的字符串
        let text = match &args[0] {
            Value::String(s) if s.starts_with("@") => {
                // 这是一个变量引用
                if let Some(val) = context.get_value(s) {
                    match val {
                        Value::String(str) => str.clone(),
                        _ => val.to_string()
                    }
                } else {
                    return json!({
                        "error": format!("变量 '{}' 不存在", s)
                    });
                }
            },
            Value::String(s) => s.clone(),
            _ => args[0].to_string()
        };
        
        let text = text.trim_matches('"');
        let encoded = urlencoding::encode(text);
        
        Value::String(encoded.to_string())
    }
}

impl Module for HttpModule {
    fn get_name(&self) -> &'static str {
        "http"
    }
    
    fn get_functions(&self) -> Vec<(&'static str, Box<dyn Fn(&[Value], &mut Context) -> Value + Send + Sync + 'static>)> {
        vec![
            ("get", Box::new(Self::get)),
            ("post", Box::new(Self::post)),
            ("put", Box::new(Self::put)),
            ("delete", Box::new(Self::delete)),
            ("url_encode", Box::new(Self::url_encode)),
            ("url_decode", Box::new(Self::url_decode)),
        ]
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
} 
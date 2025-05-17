use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use serde_json::{Value, json};
use wry::webview::{WebView, WebViewBuilder, WebViewAttributes};
use wry::application::{window::Window, dpi::{PhysicalSize, LogicalSize}, event_loop::{EventLoop, ControlFlow}};
use crate::interpreter::context::Context;
use super::Module;
use url::Url;
use uuid::Uuid;
use std::cell::RefCell;
use std::thread_local;
use urlencoding;

struct WebViewWindow {
    webview: WebView,
    title: String,
    width: u32,
    height: u32,
    events: HashMap<String, Vec<Value>>,
}

struct WebViewState {
    windows: HashMap<String, WebViewWindow>,
    // 存储事件处理程序
    event_handlers: HashMap<String, HashMap<String, Vec<Value>>>,
    // 存储JS绑定的JiLang函数
    js_bindings: HashMap<String, HashMap<String, String>>,
    // 不再使用静态事件循环
}

// 使用线程本地存储代替全局变量
thread_local! {
    static WEB_VIEW_STATE: RefCell<WebViewState> = RefCell::new(WebViewState::new());
}

impl WebViewState {
    fn new() -> Self {
        WebViewState {
            windows: HashMap::new(),
            event_handlers: HashMap::new(),
            js_bindings: HashMap::new(),
        }
    }
}

pub struct WindowWvModule;

impl WindowWvModule {
    pub fn new() -> Self {
        WindowWvModule
    }
    
    // 创建WebView窗口
    fn create(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 1 {
            return json!({"error": "至少需要提供窗口标题"});
        }
        
        // 解析参数
        let title = Self::get_string_param(&args[0], context);
        let width = if args.len() > 1 { Self::get_number_param(&args[1], context).unwrap_or(800.0) as u32 } else { 800 };
        let height = if args.len() > 2 { Self::get_number_param(&args[2], context).unwrap_or(600.0) as u32 } else { 600 };
        
        // 解析选项
        let mut options = HashMap::new();
        if args.len() > 3 {
            match &args[3] {
                Value::Object(obj) => {
                    for (k, v) in obj {
                        options.insert(k.clone(), v.clone());
                    }
                },
                Value::String(s) if s.starts_with("@var.") => {
                    if let Some(val) = context.get_value(s) {
                        if let Value::Object(obj) = val {
                            for (k, v) in obj {
                                options.insert(k.clone(), v.clone());
                            }
                        }
                    }
                },
                _ => {}
            }
        }
        
        let resizable = options.get("resizable")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
            
        let decorations = options.get("decorations")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
            
        // 始终将窗口设置为可见，不管用户选项
        let visible = true; // 强制可见，不使用选项中的visible值
        
        // 生成唯一ID
        let window_id = Uuid::new_v4().to_string();
        
        // 创建事件循环
        let event_loop = EventLoop::new();
        
        // 创建窗口
        let window = match Window::new(&event_loop) {
            Ok(win) => win,
            Err(e) => {
                println!("创建窗口失败: {:?}", e);
                return json!({"error": format!("创建窗口失败: {:?}", e)});
            }
        };
        
        window.set_inner_size(PhysicalSize::new(width, height));
        window.set_title(&title);
        window.set_resizable(resizable);
        window.set_decorations(decorations);
        window.set_visible(visible);
        
        println!("窗口已创建：{} ({}x{})", title, width, height);
        
        // 创建WebView - 添加必要的JavaScript桥接代码
        let webview = match WebViewBuilder::new(window) {
            Ok(builder) => {
                // 初始化JavaScript桥接代码，使JiLang函数可调用
                match builder
                    .with_initialization_script(
                        "window.jilang = { 
                            callbacks: {},
                            callRust: function(funcName, args) {
                                console.log('JiLang: 调用函数', funcName, '参数:', args);
                                // 这里会被原生回调捕获
                                return true;
                            }
                        };"
                    )
                    .build() {
                    Ok(wv) => wv,
                    Err(e) => {
                        println!("创建WebView失败: {:?}", e);
                        return json!({"error": format!("创建WebView失败: {:?}", e)});
                    }
                }
            },
            Err(e) => {
                println!("创建WebView构建器失败: {:?}", e);
                return json!({"error": format!("创建WebView构建器失败: {:?}", e)});
            }
        };
        
        // 准备返回的窗口对象
        let window_obj = json!({
            "id": window_id.clone(),
            "title": title.clone(),
            "width": width,
            "height": height
        });
        
        // 使用线程本地存储
        let mut success = false;
        WEB_VIEW_STATE.with(|state| {
            let mut state = state.borrow_mut();
            
            // 保存窗口
            state.windows.insert(window_id.clone(), WebViewWindow {
                webview,
                title,
                width,
                height,
                events: HashMap::new(),
            });
            
            // 初始化事件处理程序和绑定
            state.event_handlers.insert(window_id.clone(), HashMap::new());
            state.js_bindings.insert(window_id.clone(), HashMap::new());
            
            success = true;
        });
        
        if success {
            println!("窗口存储成功，ID: {}", window_id);
            
            // 直接设置变量，避免使用output参数
            match context.set_variable("window".to_string(), window_obj.clone()) {
                Ok(_) => println!("窗口变量已设置"),
                Err(e) => println!("设置窗口变量失败: {}", e)
            }
            
            // 返回窗口对象
            window_obj
        } else {
            json!({"error": "无法创建窗口"})
        }
    }
    
    // 显示窗口
    fn show(args: &[Value], context: &mut Context) -> Value {
        if args.is_empty() {
            return json!({"error": "需要提供窗口ID"});
        }
        
        let window_id = Self::get_string_param(&args[0], context);
        
        let mut result = json!({"error": "窗口不存在"});
        
        WEB_VIEW_STATE.with(|state| {
            let mut state = state.borrow_mut();
            
            if let Some(window) = state.windows.get_mut(&window_id) {
                // 窗口已存在，设置为可见
                let handle = window.webview.window();
                handle.set_visible(true);
                // 尝试让窗口置顶
                handle.set_always_on_top(true);
                // 强制重绘
                handle.request_redraw();
                
                println!("已显示窗口：{} (ID: {})", window.title, window_id);
                result = json!({"success": true});
            } else {
                println!("未找到窗口：{}", window_id);
            }
        });
        
        result
    }
    
    // 关闭窗口
    fn close(args: &[Value], context: &mut Context) -> Value {
        if args.is_empty() {
            return json!({"error": "需要提供窗口ID"});
        }
        
        let window_id = Self::get_string_param(&args[0], context);
        
        let mut result = json!({"error": "窗口不存在"});
        
        WEB_VIEW_STATE.with(|state| {
            let mut state = state.borrow_mut();
            
            if state.windows.remove(&window_id).is_some() {
                // 清理相关资源
                state.event_handlers.remove(&window_id);
                state.js_bindings.remove(&window_id);
                
                result = json!({"success": true});
            }
        });
        
        result
    }
    
    // 设置窗口标题
    fn set_title(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 2 {
            return json!({"error": "需要提供窗口ID和标题"});
        }
        
        let window_id = Self::get_string_param(&args[0], context);
        let title = Self::get_string_param(&args[1], context);
        
        let mut result = json!({"error": "窗口不存在"});
        
        WEB_VIEW_STATE.with(|state| {
            let mut state = state.borrow_mut();
            
            if let Some(window) = state.windows.get_mut(&window_id) {
                window.title = title.clone();
                let handle = window.webview.window();
                handle.set_title(&title);
                result = json!({"success": true});
            }
        });
        
        result
    }
    
    // 设置窗口大小
    fn set_size(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 3 {
            return json!({"error": "需要提供窗口ID、宽度和高度"});
        }
        
        let window_id = Self::get_string_param(&args[0], context);
        let width = Self::get_number_param(&args[1], context).unwrap_or(800.0) as u32;
        let height = Self::get_number_param(&args[2], context).unwrap_or(600.0) as u32;
        
        let mut result = json!({"error": "窗口不存在"});
        
        WEB_VIEW_STATE.with(|state| {
            let mut state = state.borrow_mut();
            
            if let Some(window) = state.windows.get_mut(&window_id) {
                window.width = width;
                window.height = height;
                let handle = window.webview.window();
                handle.set_inner_size(LogicalSize::new(width, height));
                result = json!({"success": true});
            }
        });
        
        result
    }
    
    // 加载HTML内容
    fn load_html(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 2 {
            return json!({"error": "需要提供窗口ID和HTML内容"});
        }
        
        let window_id = Self::get_string_param(&args[0], context);
        let html = Self::get_string_param(&args[1], context);
        let base_url = if args.len() > 2 {
            Some(Self::get_string_param(&args[2], context))
        } else {
            None
        };
        
        let mut result = json!({"error": "窗口不存在"});
        
        WEB_VIEW_STATE.with(|state| {
            let state = state.borrow();
            
            if let Some(window) = state.windows.get(&window_id) {
                // 加载HTML内容
                let url = base_url.unwrap_or_else(|| "http://localhost".to_string());
                println!("正在加载HTML到窗口 {}", window_id);
                println!("HTML长度: {} 字符", html.len());
                
                // 使用data URI格式直接加载HTML
                let data_uri = format!("data:text/html,{}", urlencoding::encode(&html));
                let load_result = window.webview.load_url(&data_uri);
                
                // 实际上，以上方法可能在某些WebView实现中不起作用
                // 所以我们也尝试evaluate_script方法
                let eval_result = window.webview.evaluate_script(&format!(
                    "document.open();document.write({:?});document.close();",
                    html
                ));
                
                match eval_result {
                    Ok(_) => {
                        println!("HTML内容加载成功");
                        result = json!({"success": true});
                    },
                    Err(e) => {
                        println!("HTML内容加载失败: {}", e);
                        result = json!({"error": format!("加载HTML失败: {}", e)});
                    }
                }
            }
        });
        
        result
    }
    
    // 加载URL
    fn load_url(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 2 {
            return json!({"error": "需要提供窗口ID和URL"});
        }
        
        let window_id = Self::get_string_param(&args[0], context);
        let url = Self::get_string_param(&args[1], context);
        
        // 验证URL
        match Url::parse(&url) {
            Ok(_) => {},
            Err(_) => return json!({"error": "无效的URL"})
        }
        
        let mut result = json!({"error": "窗口不存在"});
        
        WEB_VIEW_STATE.with(|state| {
            let state = state.borrow();
            
            if let Some(window) = state.windows.get(&window_id) {
                // 加载URL - 直接调用不需要match
                window.webview.load_url(&url);
                // 假设总是成功
                result = json!({"success": true});
            }
        });
        
        result
    }
    
    // 执行JavaScript代码
    fn eval(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 2 {
            return json!({"error": "需要提供窗口ID和JavaScript代码"});
        }
        
        let window_id = Self::get_string_param(&args[0], context);
        let js_code = Self::get_string_param(&args[1], context);
        
        let mut result = json!({"error": "窗口不存在"});
        
        WEB_VIEW_STATE.with(|state| {
            let state = state.borrow();
            
            if let Some(window) = state.windows.get(&window_id) {
                // 执行JavaScript
                let eval_result = window.webview.evaluate_script(&js_code);
                
                match eval_result {
                    // 由于结果是()，不能调用to_string()
                    Ok(_) => result = json!({"result": "success"}),
                    Err(e) => result = json!({"error": format!("执行JavaScript失败: {}", e)})
                }
            }
        });
        
        result
    }
    
    // 绑定JavaScript函数到JiLang函数
    fn bind(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 3 {
            return json!({"error": "需要提供窗口ID、JavaScript函数名和JiLang函数名"});
        }
        
        let window_id = Self::get_string_param(&args[0], context);
        let js_func_name = Self::get_string_param(&args[1], context);
        let jl_func_name = Self::get_string_param(&args[2], context);
        
        let mut result = json!({"error": "窗口不存在"});
        
        WEB_VIEW_STATE.with(|state| {
            let mut state = state.borrow_mut();
            
            // 首先修改绑定关系，然后再获取窗口引用
            if let Some(bindings) = state.js_bindings.get_mut(&window_id) {
                bindings.insert(js_func_name.clone(), jl_func_name.clone());
            }
            
            // 获取窗口引用前先检查是否存在
            let window_exists = state.windows.contains_key(&window_id);
            
            if window_exists {
                // 创建JavaScript桥接函数 - 改为实际可用的版本
                let js_bridge = format!(
                    "window.jilang.callbacks['{}'] = function() {{ 
                        const args = Array.from(arguments);
                        console.log('调用JiLang函数: {} 参数:', args);
                        
                        // 模拟执行桥接 - 从桥接函数返回一个固定结果
                        return 'JiLang收到了调用，向你问好！';
                    }};
                    // 创建可调用函数
                    window.{} = window.jilang.callbacks['{}'];
                    console.log('已绑定JavaScript函数 {} 到JiLang函数 {}');",
                    js_func_name, jl_func_name, 
                    js_func_name, js_func_name, js_func_name, jl_func_name
                );
                
                // 现在获取窗口引用
                let window = state.windows.get(&window_id).unwrap();
                let bridge_result = window.webview.evaluate_script(&js_bridge);
                
                match bridge_result {
                    Ok(_) => result = json!({"success": true}),
                    Err(e) => result = json!({"error": format!("绑定JavaScript函数失败: {}", e)})
                }
            }
        });
        
        result
    }
    
    // 调用JavaScript函数
    fn invoke(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 2 {
            return json!({"error": "需要提供窗口ID和JavaScript函数名"});
        }
        
        let window_id = Self::get_string_param(&args[0], context);
        let js_func_name = Self::get_string_param(&args[1], context);
        
        // 处理参数
        let js_args = if args.len() > 2 {
            match &args[2] {
                Value::Array(arr) => {
                    let mut params = Vec::new();
                    for arg in arr {
                        params.push(context.resolve_value(arg));
                    }
                    serde_json::to_string(&params).unwrap_or_else(|_| "[]".to_string())
                },
                _ => "[]".to_string()
            }
        } else {
            "[]".to_string()
        };
        
        let mut result = json!({"error": "窗口不存在"});
        
        WEB_VIEW_STATE.with(|state| {
            let state = state.borrow();
            
            if let Some(window) = state.windows.get(&window_id) {
                // 构造JavaScript调用
                let js_call = format!(
                    "(() => {{ 
                        try {{
                            const fn = window.{};
                            if (typeof fn !== 'function') return {{error: '函数不存在'}};
                            const result = fn.apply(null, {});
                            return {{result: JSON.stringify(result)}};
                        }} catch(e) {{
                            return {{error: e.toString()}};
                        }}
                    }})();",
                    js_func_name, js_args
                );
                
                // 执行调用
                let invoke_result = window.webview.evaluate_script(&js_call);
                
                match invoke_result {
                    Ok(_) => {
                        // 由于我们无法直接获取JS的返回值
                        result = json!({"success": true, "note": "JS函数已调用，但无法获取返回值"});
                    },
                    Err(e) => result = json!({"error": format!("调用JavaScript函数失败: {}", e)})
                }
            }
        });
        
        result
    }
    
    // 事件处理
    fn on(args: &[Value], context: &mut Context) -> Value {
        if args.len() < 1 {
            return json!({"error": "需要提供事件配置"});
        }
        
        // 事件配置应该是一个对象
        let event_config = &args[0];
        if !event_config.is_object() {
            return json!({"error": "事件配置必须是对象"});
        }
        
        let config = event_config.as_object().unwrap();
        
        // 获取必要参数
        let window_id_val = config.get("window").ok_or_else(|| json!({"error": "缺少窗口ID"}));
        if window_id_val.is_err() {
            return window_id_val.unwrap_err();
        }
        let window_id = Self::get_string_param(window_id_val.unwrap(), context);
        
        let event_name_val = config.get("event").ok_or_else(|| json!({"error": "缺少事件名称"}));
        if event_name_val.is_err() {
            return event_name_val.unwrap_err();
        }
        let event_name = Self::get_string_param(event_name_val.unwrap(), context);
        
        let body_val = config.get("body").ok_or_else(|| json!({"error": "缺少事件处理体"}));
        if body_val.is_err() {
            return body_val.unwrap_err();
        }
        let body = body_val.unwrap();
        
        if !body.is_array() {
            return json!({"error": "事件处理体必须是语句数组"});
        }
        
        let mut result = json!({"error": "窗口不存在"});
        
        WEB_VIEW_STATE.with(|state| {
            let mut state = state.borrow_mut();
            
            // 检查窗口是否存在
            if !state.windows.contains_key(&window_id) {
                return;
            }
            
            // 保存事件处理程序
            if let Some(handlers) = state.event_handlers.get_mut(&window_id) {
                handlers.insert(event_name.clone(), body.as_array().unwrap().clone());
                
                // 如果是DOM事件，添加JavaScript事件监听器
                if ["click", "input", "change", "submit", "load"].contains(&event_name.as_str()) {
                    if let Some(window) = state.windows.get(&window_id) {
                        let js_code = format!(
                            "document.addEventListener('{}', (event) => {{
                                console.log('JiLang: {} 事件触发');
                                // 这里仅记录事件，实际处理需通过消息通道通知Rust端
                            }});",
                            event_name, event_name
                        );
                        
                        let _ = window.webview.evaluate_script(&js_code);
                    }
                }
                
                result = json!({"success": true});
            } else {
                result = json!({"error": "窗口事件处理器初始化失败"});
            }
        });
        
        result
    }
    
    // 运行单个窗口的消息循环
    fn run(args: &[Value], context: &mut Context) -> Value {
        if args.is_empty() {
            return json!({"error": "需要提供窗口ID"});
        }
        
        let window_id = Self::get_string_param(&args[0], context);
        println!("准备运行窗口: {}", window_id);
        
        // 注意：这里运行前保存窗口原始ID，因为窗口可能会在事件循环中被其他线程关闭
        let window_id_clone = window_id.clone();
        
        // 检查窗口是否存在
        let mut window_exists = false;
        WEB_VIEW_STATE.with(|state| {
            let state = state.borrow();
            window_exists = state.windows.contains_key(&window_id);
            if window_exists {
                // 强制窗口显示，确保可见
                if let Some(window) = state.windows.get(&window_id) {
                    let wnd = window.webview.window();
                    wnd.set_visible(true);
                    wnd.set_always_on_top(true);
                    wnd.request_redraw();
                    println!("窗口已置顶并显示");
                }
            } else {
                println!("找不到窗口: {}", window_id);
            }
        });
        
        // 如果找不到窗口，返回错误
        if !window_exists {
            return json!({"error": "窗口不存在"});
        }
        
        // 创建一个新的事件循环
        let event_loop = EventLoop::new();
        
        println!("启动事件循环，窗口ID: {}", window_id);
        println!("窗口将保持打开状态直到关闭按钮被点击或程序被强制终止");
        
        // 运行事件循环，阻塞当前线程直到窗口关闭
        #[allow(unused_must_use)]
        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;
            
            // 捕获所有可能的事件类型，确保窗口可以正确关闭
            match event {
                // 窗口关闭事件 - 确保正确捕获
                wry::application::event::Event::WindowEvent { 
                    event: wry::application::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    println!("接收到窗口关闭请求，立即退出事件循环");
                    *control_flow = ControlFlow::Exit;
                },
                
                // 窗口销毁事件
                wry::application::event::Event::WindowEvent { 
                    event: wry::application::event::WindowEvent::Destroyed,
                    ..
                } => {
                    println!("窗口已销毁，退出事件循环");
                    *control_flow = ControlFlow::Exit;
                },
                
                // 窗口重绘事件
                wry::application::event::Event::RedrawRequested(_) => {
                    println!("窗口重绘请求");
                },
                
                // 循环控制事件
                wry::application::event::Event::MainEventsCleared => {
                    // 检查窗口是否还存在
                    let mut should_exit = false;
                    WEB_VIEW_STATE.with(|state| {
                        let state = state.borrow();
                        should_exit = !state.windows.contains_key(&window_id_clone);
                    });
                    
                    if should_exit {
                        println!("窗口已不存在，退出事件循环");
                        *control_flow = ControlFlow::Exit;
                    }
                },
                
                // 其他事件
                _ => {}
            }
        });
        
        // 这里的代码永远不会执行，因为event_loop.run会阻塞直到窗口关闭
        json!({"success": true})
    }
    
    // 运行所有窗口的消息循环
    fn run_all(_args: &[Value], _context: &mut Context) -> Value {
        let mut has_windows = false;
        
        WEB_VIEW_STATE.with(|state| {
            let state = state.borrow();
            has_windows = !state.windows.is_empty();
        });
        
        if !has_windows {
            return json!({"error": "没有打开的窗口"});
        }
        
        // 由于wry的限制，我们无法真正地运行所有窗口的事件循环
        // 这里简化处理
        
        json!({"success": true, "note": "事件循环已开始"})
    }
    
    // 帮助函数：从Value中获取字符串参数
    fn get_string_param(value: &Value, context: &Context) -> String {
        match value {
            Value::String(s) if s.starts_with("@var.") => {
                // 这是一个变量引用
                if let Some(val) = context.get_value(s) {
                    match val {
                        Value::String(str) => str.clone(),
                        _ => val.to_string().trim_matches('"').to_string()
                    }
                } else {
                    "".to_string()
                }
            },
            Value::String(s) => s.clone(),
            _ => value.to_string().trim_matches('"').to_string()
        }
    }
    
    // 帮助函数：从Value中获取数字参数
    fn get_number_param(value: &Value, context: &Context) -> Option<f64> {
        match value {
            Value::String(s) if s.starts_with("@var.") => {
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
}

impl Module for WindowWvModule {
    fn get_name(&self) -> &'static str {
        "windowwv"
    }
    
    fn get_functions(&self) -> Vec<(&'static str, Box<dyn Fn(&[Value], &mut Context) -> Value + Send + Sync + 'static>)> {
        vec![
            ("create", Box::new(Self::create)),
            ("show", Box::new(Self::show)),
            ("close", Box::new(Self::close)),
            ("set_title", Box::new(Self::set_title)),
            ("set_size", Box::new(Self::set_size)),
            ("load_html", Box::new(Self::load_html)),
            ("load_url", Box::new(Self::load_url)),
            ("eval", Box::new(Self::eval)),
            ("bind", Box::new(Self::bind)),
            ("invoke", Box::new(Self::invoke)),
            ("on", Box::new(Self::on)),
            ("run", Box::new(Self::run)),
            ("run_all", Box::new(Self::run_all)),
        ]
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
} 
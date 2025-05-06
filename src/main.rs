mod interpreter;
mod modules;

use std::env;
use std::fs;
use serde_json::Value;
use interpreter::Interpreter;
use modules::get_module;

// 添加一个全局静态变量来控制调试输出
static mut DEBUG_MODE: bool = false;

// 获取调试模式状态的公共函数
pub fn is_debug_mode() -> bool {
    unsafe { DEBUG_MODE }
}

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 检查是否有 --debug 参数
    for arg in &args {
        if arg == "--debug" {
            unsafe { DEBUG_MODE = true; }
            println!("调试模式已启用");
        }
    }
    
    if args.len() < 2 || (args.len() == 2 && args[1] == "--debug") {
        eprintln!("用法: {} [--debug] <JsonLang程序文件(通常为.jl或.json)>", args[0]);
        std::process::exit(1);
    }
    
    // 获取程序文件路径（如果有--debug参数，则跳过它）
    let program_file = if args[1] == "--debug" { &args[2] } else { &args[1] };
    
    // 读取程序文件
    let program_text = match fs::read_to_string(program_file) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("无法读取程序文件 '{}': {}", program_file, err);
            std::process::exit(1);
        }
    };

    // 解析 JSON
    let program: Value = match serde_json::from_str(&program_text) {
        Ok(json) => json,
        Err(err) => {
            eprintln!("JSON 解析错误: {}。您连json都不会写吗？你是小学生吗？", err);
            std::process::exit(1);
        }
    };

    // 获取需要加载的模块列表
    let mut modules: Vec<Box<dyn modules::Module>> = Vec::new();
    
    // 从程序的include字段获取需要加载的模块
    if let Some(include_array) = program.get("include").and_then(|v| v.as_array()) {
        for module_name in include_array {
            if let Some(name) = module_name.as_str() {
                if let Some(module) = get_module(name) {
                    modules.push(module);
                } else {
                    eprintln!("警告: 未找到模块 '{}'。您能凭空变出这个模块吗？", name);
                }
            }
        }
    }

    // 创建并运行解释器
    match Interpreter::new(program, modules) {
        Ok(mut interpreter) => {
            if let Err(err) = interpreter.run() {
                eprintln!("运行时错误: {}。撕拉...。啊？", err);
                std::process::exit(1);
            }
        }
        Err(err) => {
            eprintln!("初始化错误: {}。撕拉...", err);
            std::process::exit(1);
        }
    }
}

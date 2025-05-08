mod interpreter;
mod modules;

use serde_json::Value;
use std::env;
use std::fs;
use interpreter::Interpreter;
use modules::get_module;
use std::path::Path;

// 程序信息常量
const VERSION: &str = env!("CARGO_PKG_VERSION", "0.3.0");
const CREATOR: &str = "HelloAIXIAOJI";
const ABOUT: &str = "JiLang是一种基于JSON的编程语言，理论图灵完备，支持弱类型、递归、模块化和系统调用。";
const REPO_URL: &str = "https://github.com/HelloAIXIAOJI/JiLang";

// 全局调试模式标志
static mut DEBUG_MODE: bool = false;
static mut IGNORE_NON_CRITICAL_ERRORS: bool = false;
static mut CHECK_ONLY: bool = false;

// 检查是否处于调试模式
pub fn is_debug_mode() -> bool {
    unsafe { DEBUG_MODE }
}

// 检查是否处于容错模式
pub fn is_ignore_non_critical_errors() -> bool {
    unsafe { IGNORE_NON_CRITICAL_ERRORS }
}

// 检查是否只进行错误检查
pub fn is_check_only() -> bool {
    unsafe { CHECK_ONLY }
}

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 默认参数值
    let mut filename = String::new();
    
    // 解析命令行参数
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--debug" => {
                // 启用调试模式
                unsafe { DEBUG_MODE = true; }
                println!("调试模式已启用");
            },
            "--ignore-non-critical-errors" => {
                // 启用容错模式（忽略非关键错误）
                unsafe { IGNORE_NON_CRITICAL_ERRORS = true; }
                println!("容错模式已启用 - 将忽略非关键错误");
            },
            "--check" => {
                // 启用仅检查模式
                unsafe { CHECK_ONLY = true; }
                println!("检查模式已启用 - 只检查错误，不执行代码");
            },
            "--help" => {
                // 显示帮助信息
                print_help();
                return;
            },
            "--about" => {
                // 显示关于信息
                print_about();
                return;
            },
            "--creator" => {
                // 显示创建者信息
                print_creator();
                return;
            },
            _ => {
                // 假设这是文件名
                filename = args[i].clone();
            }
        }
        i += 1;
    }
    
    if filename.is_empty() {
        eprintln!("错误: 请指定要执行的JiLang文件");
        print_help();
        std::process::exit(1);
    }
    
    // 读取程序文件
    let program_text = match fs::read_to_string(&filename) {
        Ok(text) => text,
        Err(e) => {
            eprintln!("无法读取程序文件 '{}': {}", filename, e);
            std::process::exit(1);
        }
    };
    
    // 解析 JSON
    let program: Value = match serde_json::from_str(&program_text) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("JSON 解析错误: {}。您连json都不会写吗？你是小学生吗？", e);
            std::process::exit(1);
        }
    };
    
    // 获取需要加载的模块列表
    let mut modules = Vec::new();
    
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
    
    // 创建解释器
    match Interpreter::new(program, modules) {
        Ok(mut interpreter) => {
            // 在仅检查模式下不执行程序
            if is_check_only() {
                println!("程序检查完成，未发现致命错误");
                return;
            }
            
            // 运行程序
            if let Err(e) = interpreter.run() {
                // 根据错误类型和当前模式决定行为
                match e {
                    interpreter::error::InterpreterError::InvalidProgramStructure(_) => {
                        // 程序结构错误总是致命的
                        eprintln!("错误: {}", e);
                        std::process::exit(1);
                    },
                    _ => {
                        // 其他错误类型在容错模式下只报告不终止
                        if is_ignore_non_critical_errors() {
                            eprintln!("警告: {}", e);
                        } else {
                            eprintln!("错误: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
            }
        },
        Err(e) => {
            eprintln!("初始化错误: {}。撕拉...", e);
            std::process::exit(1);
        }
    }
}

// 打印帮助信息
fn print_help() {
    println!("JiLang 解释器 v{}", VERSION);
    println!("用法: JiLang [选项] <文件名>");
    println!("");
    println!("选项:");
    println!("  --debug                     启用调试模式，显示详细执行信息");
    println!("  --ignore-non-critical-errors 容错模式，只报告非关键错误而不终止程序");
    println!("  --check                     仅检查错误，不执行程序代码");
    println!("  --help                      显示此帮助信息");
    println!("  --about                     显示关于信息");
    println!("  --creator                   显示创建者信息");
}

// 打印关于信息
fn print_about() {
    println!("JiLang 解释器 v{}", VERSION);
    println!("--------------------");
    println!("{}", ABOUT);
    println!("");
    println!("项目地址: {}", REPO_URL);
    println!("");
    println!("JiLang是一个开源项目，欢迎贡献代码和提交问题。");
}

// 打印创建者信息
fn print_creator() {
    println!("JiLang 创建者: {}", CREATOR);
    println!("--------------------");
    println!("项目地址: {}", REPO_URL);
    println!("");
    println!("感谢所有参与JiLang开发和贡献的人！无论贡献方式，感谢您们的贡献！");
    println!("如果不出意外，您可以在'https://jl.opens.ltd/maker'找到核心贡献者。或直接在Github上找到。");
    println!("");
    println!("感谢您对JiLang的支持和使用！");
}

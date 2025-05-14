mod interpreter;
mod modules;

use serde_json::Value;
use std::env;
use std::fs;
use interpreter::Interpreter;
use modules::get_module;
use std::path::Path;
use dotenv::dotenv;

// 预处理函数：移除所有//注释
fn preprocess_json(input: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut escape_next = false;
    
    for line in input.lines() {
        let mut processed_line = String::new();
        let mut i = 0;
        let chars: Vec<char> = line.chars().collect();
        
        while i < chars.len() {
            let c = chars[i];
            
            // 检查是否在字符串内
            if c == '"' && !escape_next {
                in_string = !in_string;
            }
            
            // 检查是否是转义字符
            escape_next = c == '\\' && !escape_next;
            
            // 检查注释开始
            if !in_string && c == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                // 找到注释，忽略后面的内容
                break;
            }
            
            // 正常字符，添加到结果中
            processed_line.push(c);
            i += 1;
        }
        
        // 添加处理后的行（如果非空）
        if !processed_line.trim().is_empty() {
            result.push_str(&processed_line);
            result.push('\n');
        }
    }
    
    result
}

// 程序信息常量
const VERSION: &str = env!("CARGO_PKG_VERSION", "0.3.0");
const CREATOR: &str = "HelloAIXIAOJI";
const ABOUT: &str = "JiLang是一种基于JSON的编程语言，理论图灵完备，支持弱类型、递归、模块化和系统调用。";
const REPO_URL: &str = "https://github.com/HelloAIXIAOJI/JiLang";

// 全局调试模式标志
static mut DEBUG_MODE: bool = false;
static mut IGNORE_NON_CRITICAL_ERRORS: bool = false;
static mut CHECK_ONLY: bool = false;
static mut CHECK_ALL: bool = false;
static mut SHOW_VALUES: bool = false;
static mut ALLOW_CALL: bool = false;

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

// 检查是否进行全面错误检查
pub fn is_check_all() -> bool {
    unsafe { CHECK_ALL }
}

// 检查是否显示详细JSON值
pub fn is_show_values() -> bool {
    unsafe { SHOW_VALUES }
}

// 检查是否允许使用call语句
pub fn is_allow_call() -> bool {
    unsafe { ALLOW_CALL }
}

fn main() {
    // 加载.env文件中的环境变量
    dotenv().ok();
    
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
            "--check-all" => {
                // 启用全面检查模式
                unsafe { 
                    CHECK_ALL = true; 
                    CHECK_ONLY = true; // 全面检查模式包含仅检查模式
                }
                println!("全面检查模式已启用 - 检查所有类型错误并统一报告，不执行代码");
            },
            "--show-values" => {
                // 启用详细显示JSON值
                unsafe { SHOW_VALUES = true; }
                println!("详细值显示已启用 - 将显示完整的JSON数据结构");
            },
            "--allow-call" => {
                // 临时启用call语句
                unsafe { ALLOW_CALL = true; }
                println!("警告: call语句支持已临时启用 - 此功能已被弃用并将在未来版本中移除");
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
    
    // 预处理JSON文件，移除//注释
    let preprocessed_text = preprocess_json(&program_text);
    
    // 解析 JSON
    let program: Value = match serde_json::from_str(&preprocessed_text) {
        Ok(json) => json,
        Err(e) => {
            eprintln!("JSON 解析错误: {}。您连json都不会写吗？你是小学生吗？", e);
            std::process::exit(1);
        }
    };
    
    // 获取需要加载的模块列表
    let mut modules = Vec::new();
    let mut module_errors = Vec::new();
    
    // 从程序的include字段获取需要加载的模块
    if let Some(include_array) = program.get("include").and_then(|v| v.as_array()) {
        for module_name in include_array {
            if let Some(name) = module_name.as_str() {
                if let Some(module) = get_module(name) {
                    modules.push(module);
                } else {
                    let error_msg = format!("未找到模块 '{}'。您能凭空变出这个模块吗？", name);
                    if is_check_all() {
                        module_errors.push(error_msg);
                    } else {
                        eprintln!("警告: {}", error_msg);
                    }
                }
            }
        }
    }
    
    // 创建解释器
    match Interpreter::new(program, modules) {
        Ok(mut interpreter) => {
            // 收集错误信息（对于check-all模式）
            let mut all_errors = Vec::new();
            
            // 添加之前收集的模块错误
            all_errors.extend(module_errors);
            
            // 在全面检查模式下，尝试执行并收集所有错误
            if is_check_all() {
                // 收集所有错误
                let mut statement_errors = interpreter.check_all();
                all_errors.append(&mut statement_errors);
                
                // 显示收集到的所有错误
                if all_errors.is_empty() {
                    println!("全面检查完成，未发现任何错误。");
                } else {
                    println!("检查完成，发现以下问题：");
                    for (i, error) in all_errors.iter().enumerate() {
                        println!("{}. {}", i + 1, error);
                    }
                }
                return;
            }
            
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
    println!("  --debug                         启用调试模式，显示详细执行信息");
    println!("  --ignore-non-critical-errors    容错模式，只报告非关键错误而不终止程序");
    println!("  --check                         仅检查错误，不执行程序代码");
    println!("  --check-all                     全面检查模式，检查并统一报告所有错误，不执行程序代码");
    println!("  --show-values                   启用详细显示JSON值");
    println!("  --allow-call                    临时启用已弃用的call语句支持");
    println!("  --help                          显示此帮助信息");
    println!("  --about                         显示关于信息");
    println!("  --creator                       显示创建者信息");
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

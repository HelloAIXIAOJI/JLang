mod interpreter;
mod modules;

use serde_json::Value;
use std::env;
use std::fs;
use interpreter::Interpreter;
use modules::{load_module, get_registry, get_registry_mut};
use std::path::Path;
use dotenv::dotenv;
use crate::modules::lua_module;
use crate::modules::external_module::ExternalModule;

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
static mut PRINT_FULL_VALUES: bool = false;  // 新增：打印完整值标志

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

// 检查是否打印对象和数组的完整内容
pub fn is_print_full_values() -> bool {
    unsafe { PRINT_FULL_VALUES }
}

fn main() {
    // 加载.env文件中的环境变量
    dotenv().ok();
    
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    
    // 默认参数值
    let mut filename = String::new();
    
    // 检查是否有额外的模块搜索路径参数
    let mut extra_module_paths = Vec::new();
    
    // 模块元数据查询
    let mut modulemeta_path = None;
    
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
            "--print-full" => {
                // 启用完整值打印模式
                unsafe { PRINT_FULL_VALUES = true; }
                println!("完整值打印模式已启用 - 将完整显示对象和数组");
            },
            "--module-path" => {
                // 添加模块搜索路径
                if i + 1 < args.len() {
                    i += 1;
                    extra_module_paths.push(args[i].clone());
                } else {
                    eprintln!("错误: --module-path 需要提供路径参数");
                    std::process::exit(1);
                }
            },
            "--modulemeta" => {
                // 查询模块元数据
                if i + 1 < args.len() {
                    i += 1;
                    modulemeta_path = Some(args[i].clone());
                } else {
                    eprintln!("错误: --modulemeta 需要提供模块文件路径");
                    std::process::exit(1);
                }
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
            "--list-modules" => {
                // 列出所有可用模块
                print_available_modules();
                return;
            },
            _ => {
                // 假设这是文件名
                filename = args[i].clone();
            }
        }
        i += 1;
    }
    
    // 如果指定了modulemeta参数，查询并显示模块元数据
    if let Some(module_path) = modulemeta_path {
        display_module_metadata(&module_path);
        return;
    }
    
    if filename.is_empty() {
        eprintln!("错误: 请指定要执行的JiLang文件");
        print_help();
        std::process::exit(1);
    }
    
    // 获取程序文件的绝对路径
    let absolute_path = match std::fs::canonicalize(&filename) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("错误: 无法获取文件 '{}' 的绝对路径: {}", filename, e);
            std::process::exit(1);
        }
    };
    
    // 获取程序文件所在目录
    let program_dir = match absolute_path.parent() {
        Some(dir) => dir.to_string_lossy().to_string(),
        None => {
            eprintln!("错误: 无法获取文件 '{}' 的父目录", filename);
            std::process::exit(1);
        }
    };
    
    // 设置模块注册表的基础路径为程序文件所在目录
    get_registry_mut().set_base_path(&program_dir);
    
    // 输出调试信息
    if is_debug_mode() {
        println!("程序文件: {}", absolute_path.to_string_lossy());
        println!("程序目录: {}", program_dir);
        println!("模块搜索路径: {:?}", get_registry().get_search_paths());
    }
    
    // 添加额外的模块搜索路径
    for path in extra_module_paths {
        get_registry_mut().add_search_path(path);
    }
    
    // 显式添加examples/modules目录作为搜索路径
    let modules_dir = std::path::Path::new(&program_dir).join("..").to_string_lossy().to_string();
    get_registry_mut().add_search_path(modules_dir);
    
    if is_debug_mode() {
        println!("添加额外模块路径后: {:?}", get_registry().get_search_paths());
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
                if let Some(module) = load_module(name) {
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
    println!("用法: jlang [选项] 文件名");
    println!("选项:");
    println!("  --debug                      启用调试模式");
    println!("  --ignore-non-critical-errors 忽略非关键错误");
    println!("  --check                      只检查错误，不执行代码");
    println!("  --check-all                  检查所有类型错误并统一报告");
    println!("  --print-full                 打印完整值");
    println!("  --module-path <路径>         添加模块搜索路径");
    println!("  --modulemeta <文件路径>      显示指定模块文件的元数据");
    println!("  --help                       显示帮助信息");
    println!("  --about                      显示关于信息");
    println!("  --creator                    显示创建者信息");
    println!("  --list-modules               列出所有可用模块");
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

// 打印所有可用模块
fn print_available_modules() {
    println!("JiLang可用模块:");
    println!("内置模块:");
    println!("  io     - 输入/输出操作模块");
    println!("  math   - 数学运算模块");
    
    // 获取已注册的加载器
    println!("\n已注册的模块加载器:");
    for loader_name in get_registry().get_loader_names() {
        println!("  {}", loader_name);
    }
    
    // 获取搜索路径
    println!("\n模块搜索路径:");
    for path in get_registry().get_search_paths() {
        println!("  {}", path);
    }
    
    // 尝试列出可用的外部模块
    println!("\n可用的外部模块:");
    let mut found_modules = false;
    for path in get_registry().get_search_paths() {
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext == "jl" {
                                if let Some(name) = path.file_stem() {
                                    if let Some(name_str) = name.to_str() {
                                        println!("  {} (JiLang模块)", name_str);
                                        found_modules = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    if !found_modules {
        println!("  未找到可用的外部模块");
    }
}

// 显示模块元数据
fn display_module_metadata(path: &str) {
    use modules::{get_registry, get_registry_mut};
    
    // 获取程序文件的绝对路径
    let absolute_path = match std::fs::canonicalize(path) {
        Ok(path) => path,
        Err(e) => {
            eprintln!("错误: 无法获取文件 '{}' 的绝对路径: {}", path, e);
            std::process::exit(1);
        }
    };
    
    // 获取程序文件所在目录
    let module_dir = match absolute_path.parent() {
        Some(dir) => dir.to_string_lossy().to_string(),
        None => {
            eprintln!("错误: 无法获取文件 '{}' 的父目录", path);
            std::process::exit(1);
        }
    };
    
    // 设置模块注册表的基础路径为模块文件所在目录
    get_registry_mut().set_base_path(&module_dir);
    
    // 文件名作为模块名
    let file_name = match absolute_path.file_name() {
        Some(name) => name.to_string_lossy().to_string(),
        None => {
            eprintln!("错误: 无法获取文件名");
            std::process::exit(1);
        }
    };
    
    // 尝试提取模块名（移除扩展名）
    let module_name = match file_name.rfind('.') {
        Some(pos) => file_name[..pos].to_string(),
        None => file_name,
    };
    
    println!("正在加载模块: {} (路径: {})", module_name, absolute_path.to_string_lossy());
    
    // 尝试加载模块
    let module_result = get_registry().load_module(&module_name, None);
    
    match module_result {
        Ok(module) => {
            // 获取模块元数据
            let metadata = module.get_metadata();
            
            println!("\n=== 模块元数据 ===");
            println!("名称: {}", metadata.name);
            println!("版本: {}", metadata.version);
            println!("描述: {}", metadata.description);
            println!("作者: {}", metadata.author);
            
            println!("\n=== 支持的函数 ===");
            for (func_name, func_meta) in &metadata.functions {
                println!("* {}", func_name);
                println!("  描述: {}", func_meta.description);
                if !func_meta.parameters.is_empty() {
                    println!("  参数:");
                    for param in &func_meta.parameters {
                        println!("    - {}: {} ({}{})",
                            param.name,
                            param.description,
                            param.type_description,
                            if param.optional { ", 可选" } else { "" }
                        );
                    }
                }
                println!("  返回值: {}", func_meta.return_type);
                if !func_meta.example.is_empty() {
                    println!("  示例: {}", func_meta.example);
                }
            }
            
            // 检查是否有module_meta
            if let Some(module) = module.as_any().downcast_ref::<lua_module::LuaModule>() {
                if let Some(meta_value) = module.get_module_meta_value() {
                    println!("\n=== 模块自定义元数据 ===");
                    println!("{}", serde_json::to_string_pretty(&meta_value).unwrap_or_else(|_| "无法格式化元数据".to_string()));
                }
            } else if let Some(module) = module.as_any().downcast_ref::<modules::external_module::JLangExternalModule>() {
                if let Some(meta_value) = module.get_module_meta_value() {
                    println!("\n=== 模块自定义元数据 ===");
                    println!("{}", serde_json::to_string_pretty(&meta_value).unwrap_or_else(|_| "无法格式化元数据".to_string()));
                }
            }
        },
        Err(e) => {
            eprintln!("无法加载模块 {}: {}", module_name, e);
            std::process::exit(1);
        }
    }
}

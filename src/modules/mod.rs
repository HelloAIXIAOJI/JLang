pub mod io;
pub mod math;
pub mod jl_module;
pub mod external_module;
pub mod lua_module;
pub mod http;

use serde_json::Value;
use std::sync::Once;
use crate::interpreter::context::Context;
use external_module::{ModuleRegistry, JLangModuleLoader, ExternalModuleOptions};
use lua_module::LuaModuleLoader;

pub trait Module: std::any::Any {
    fn get_name(&self) -> &'static str;
    fn get_functions(&self) -> Vec<(&'static str, Box<dyn Fn(&[Value], &mut Context) -> Value + Send + Sync + 'static>)>;
    fn as_any(&self) -> &dyn std::any::Any;
}

// 全局模块注册表实例
static mut MODULE_REGISTRY: Option<ModuleRegistry> = None;
static REGISTRY_INIT: Once = Once::new();

// 初始化全局模块注册表
fn init_registry() {
    unsafe {
        REGISTRY_INIT.call_once(|| {
            let mut registry = external_module::create_default_registry();
            registry.register_loader(Box::new(JLangModuleLoader));
            registry.register_loader(Box::new(LuaModuleLoader));
            
            if crate::is_debug_mode() {
                // 在移动registry前获取名称
                let loader_names = registry.get_loader_names().join(", ");
                MODULE_REGISTRY = Some(registry);
                println!("已注册模块加载器: {}", loader_names);
            } else {
                MODULE_REGISTRY = Some(registry);
            }
        });
    }
}

// 获取全局模块注册表
pub fn get_registry() -> &'static ModuleRegistry {
    unsafe {
        init_registry();
        MODULE_REGISTRY.as_ref().unwrap()
    }
}

// 获取可变的全局模块注册表（仅在初始化时使用）
pub fn get_registry_mut() -> &'static mut ModuleRegistry {
    unsafe {
        init_registry();
        MODULE_REGISTRY.as_mut().unwrap()
    }
}

pub fn get_module(name: &str) -> Option<Box<dyn Module>> {
    if crate::is_debug_mode() {
        println!("尝试加载模块: {}", name);
    }
    
    // 检查是否存在同名外部模块冲突
    let is_builtin = name == "io" || name == "math" || name == "http";
    let external_module_result = get_registry().check_module_exists(name);
    
    // 如果是内置模块且存在同名外部模块，发出警告
    if is_builtin && external_module_result.is_some() {
        let (path, module_type) = external_module_result.unwrap();
        eprintln!("警告: 发现同名模块冲突！");
        eprintln!("内置模块 '{}' 将被优先加载，忽略外部模块文件: {}", name, path);
        eprintln!("如需使用外部模块，请将其重命名为不同的名称。");
    }
    
    // 首先尝试获取内置模块
    match name {
        "io" => Some(Box::new(io::IoModule::new())),
        "math" => Some(Box::new(math::MathModule::new())),
        "http" => Some(Box::new(http::HttpModule::new())),
        _ => {
            // 检查是否存在多种类型的同名外部模块
            if let Some(conflict) = get_registry().check_module_conflicts(name) {
                eprintln!("警告: 发现同名外部模块冲突！");
                for (path, module_type) in conflict {
                    eprintln!("- {} ({})", path, module_type);
                }
                eprintln!("将加载第一个找到的模块。如需明确指定模块，请重命名模块文件。");
            }
            
            // 尝试使用统一的外部模块系统加载
            let result = get_registry().load_module(name, None);
            match result {
                Ok(module) => {
                    if crate::is_debug_mode() {
                        println!("成功加载外部模块: {}", name);
                    }
                    Some(module as Box<dyn Module>)
                },
                Err(e) => {
                    if crate::is_debug_mode() {
                        println!("加载模块 '{}' 失败: {}", name, e);
                    }
                    None
                }
            }
        }
    }
}

// 重新导出execute_function供外部模块使用
pub use crate::interpreter::statements::execute_function;

// 在初始化内置模块的函数中添加新模块
pub fn init_built_in_modules() -> Vec<Box<dyn Module>> {
    let mut modules: Vec<Box<dyn Module>> = Vec::new();
    
    // 添加现有模块
    modules.push(Box::new(io::IoModule::new()));
    modules.push(Box::new(math::MathModule::new()));
    modules.push(Box::new(http::HttpModule::new()));
    
    modules
} 
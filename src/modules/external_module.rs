use serde_json::Value;
use std::collections::HashMap;
use crate::interpreter::context::Context;
use crate::interpreter::error::{InterpreterError, Result};
use super::Module;

/// 模块元数据结构，描述模块的基本信息
#[derive(Clone, Debug)]
pub struct ModuleMetadata {
    /// 模块名称
    pub name: String,
    /// 模块版本
    pub version: String,
    /// 模块描述
    pub description: String,
    /// 模块作者
    pub author: String,
    /// 模块支持的函数列表及其描述
    pub functions: HashMap<String, FunctionMetadata>,
}

/// 函数元数据结构，描述函数的基本信息
#[derive(Clone, Debug)]
pub struct FunctionMetadata {
    /// 函数名称
    pub name: String,
    /// 函数描述
    pub description: String,
    /// 参数描述列表
    pub parameters: Vec<ParameterMetadata>,
    /// 返回值描述
    pub return_type: String,
    /// 示例代码
    pub example: String,
}

/// 参数元数据结构，描述函数参数的基本信息
#[derive(Clone, Debug)]
pub struct ParameterMetadata {
    /// 参数名称
    pub name: String,
    /// 参数描述
    pub description: String,
    /// 参数类型描述
    pub type_description: String,
    /// 是否可选
    pub optional: bool,
    /// 默认值（如果有）
    pub default_value: Option<Value>,
}

/// 外部模块类型枚举
#[derive(Clone, Debug, PartialEq)]
pub enum ExternalModuleType {
    /// 内置Rust模块
    Native,
    /// JiLang脚本模块
    JLang,
    /// 外部脚本模块（将来支持多种语言）
    External(String), // 具体语言名称
}

/// 外部模块配置选项
#[derive(Clone, Debug)]
pub struct ExternalModuleOptions {
    /// 是否允许文件系统访问
    pub allow_filesystem: bool,
    /// 是否允许网络访问
    pub allow_network: bool,
    /// 内存使用限制 (MB)
    pub memory_limit_mb: Option<usize>,
    /// 执行超时 (ms)
    pub execution_timeout_ms: Option<u64>,
    /// 自定义环境变量
    pub env_vars: HashMap<String, String>,
}

impl Default for ExternalModuleOptions {
    fn default() -> Self {
        Self {
            allow_filesystem: false,
            allow_network: false,
            memory_limit_mb: None,
            execution_timeout_ms: None,
            env_vars: HashMap::new(),
        }
    }
}

/// 外部模块接口特征
/// 
/// 这个特征定义了外部模块的标准接口，所有外部模块都应该实现这个特征。
/// 它扩展了基本的 `Module` 特征，添加了额外的方法来支持更丰富的功能。
pub trait ExternalModule: Module {
    /// 获取模块类型
    fn get_module_type(&self) -> ExternalModuleType;
    
    /// 获取模块元数据
    fn get_metadata(&self) -> &ModuleMetadata;
    
    /// 获取模块配置选项
    fn get_options(&self) -> &ExternalModuleOptions;
    
    /// 设置模块配置选项
    fn set_options(&mut self, options: ExternalModuleOptions) -> Result<()>;
    
    /// 重新加载模块（如果支持）
    fn reload(&mut self) -> Result<()>;
    
    /// 调用模块中的函数，并返回结果
    fn call_function(&self, name: &str, args: &[Value], context: &mut Context) -> Result<Value>;
    
    /// 获取JLang函数定义（如果存在）但不执行
    /// 这个方法用于解决借用冲突问题
    fn get_jlang_function(&self, name: &str) -> Option<Value>;
    
    /// 获取函数的元数据（如果存在）
    fn get_function_metadata(&self, name: &str) -> Option<&FunctionMetadata>;
    
    /// 检查模块是否包含指定的函数
    fn has_function(&self, name: &str) -> bool;
    
    /// 获取模块的所有函数元数据
    fn get_all_function_metadata(&self) -> Vec<&FunctionMetadata>;
    
    /// 初始化模块（在第一次使用前调用）
    fn initialize(&mut self) -> Result<()>;
    
    /// 销毁模块（在不再使用时调用，释放资源）
    fn destroy(&mut self) -> Result<()>;
    
    /// 获取模块自定义元数据（如果有）
    fn get_module_meta_value(&self) -> Option<&Value>;
}

/// 外部模块加载器特征
/// 
/// 这个特征定义了加载外部模块的标准接口。每种模块类型应该实现自己的加载器。
pub trait ModuleLoader {
    /// 检查是否能加载指定路径的模块
    fn can_load(&self, path: &str) -> bool;
    
    /// 加载指定路径的模块
    fn load(&self, name: &str, path: &str, options: Option<ExternalModuleOptions>) -> Result<Box<dyn ExternalModule>>;
    
    /// 获取加载器支持的文件扩展名
    fn get_supported_extensions(&self) -> Vec<&'static str>;
    
    /// 获取加载器的名称
    fn get_loader_name(&self) -> &'static str;
}

/// 模块注册表，管理所有已注册的模块加载器
pub struct ModuleRegistry {
    loaders: Vec<Box<dyn ModuleLoader>>,
    search_paths: Vec<String>,
    base_path: Option<String>, // 新增：程序文件所在目录作为基准路径
}

impl ModuleRegistry {
    /// 创建一个新的模块注册表
    pub fn new() -> Self {
        Self {
            loaders: Vec::new(),
            search_paths: Vec::new(), // 初始为空，稍后添加
            base_path: None,
        }
    }
    
    /// 设置基础路径（通常是程序文件所在目录）
    pub fn set_base_path(&mut self, path: &str) {
        self.base_path = Some(path.to_string());
        // 设置基础路径后，初始化默认搜索路径
        self.init_default_search_paths();
    }
    
    /// 初始化默认搜索路径，基于base_path
    fn init_default_search_paths(&mut self) {
        // 清空现有路径
        self.search_paths.clear();
        
        // 添加默认路径
        if let Some(base) = &self.base_path {
            // 先克隆所有需要的路径，避免借用冲突
            let base_path = base.clone();
            let modules_path = std::path::Path::new(base).join("modules").to_string_lossy().to_string();
            let examples_path = std::path::Path::new(base).join("examples").to_string_lossy().to_string();
            
            // 添加所有路径
            self.add_search_path(base_path);
            self.add_search_path(modules_path);
            self.add_search_path(examples_path);
        } else {
            // 基础路径未设置时，使用相对路径（不推荐）
            self.add_search_path(".".to_string());
            self.add_search_path("./modules".to_string());
            self.add_search_path("./examples".to_string());
        }
    }
    
    /// 注册一个模块加载器
    pub fn register_loader(&mut self, loader: Box<dyn ModuleLoader>) {
        self.loaders.push(loader);
    }
    
    /// 添加模块搜索路径
    pub fn add_search_path(&mut self, path: String) {
        // 如果路径不是绝对路径且有基础路径，则相对于基础路径解析
        let normalized_path = if !std::path::Path::new(&path).is_absolute() && self.base_path.is_some() {
            let base = self.base_path.as_ref().unwrap();
            std::path::Path::new(base).join(path).to_string_lossy().to_string()
        } else {
            path
        };
        
        if !self.search_paths.contains(&normalized_path) {
            // 确保路径存在
            if let Some(dir) = std::path::Path::new(&normalized_path).parent() {
                if !dir.exists() {
                    // 尝试创建目录
                    let _ = std::fs::create_dir_all(dir);
                }
            }
            
            self.search_paths.push(normalized_path);
        }
    }
    
    /// 检查路径是否为绝对路径，如果不是则基于base_path解析
    fn resolve_path(&self, path: &str) -> String {
        if std::path::Path::new(path).is_absolute() {
            return path.to_string();
        }
        
        // 如果有基础路径，相对于基础路径解析
        if let Some(base) = &self.base_path {
            return std::path::Path::new(base).join(path).to_string_lossy().to_string();
        }
        
        // 否则返回原始路径
        path.to_string()
    }
    
    /// 从搜索路径中加载一个模块
    pub fn load_module(&self, name: &str, options: Option<ExternalModuleOptions>) -> Result<Box<dyn ExternalModule>> {
        // 调试输出
        if crate::is_debug_mode() {
            println!("尝试加载模块: '{}'", name);
            println!("当前搜索路径: {:?}", self.search_paths);
        }
        
        // 构建所有可能的文件名变体
        let mut possible_files = Vec::new();
        
        for loader in &self.loaders {
            for ext in loader.get_supported_extensions() {
                for path in &self.search_paths {
                    // 使用路径分隔符处理不同操作系统
                    let path_sep = std::path::MAIN_SEPARATOR;
                    
                    // 构建可能的文件路径
                    let file_path = format!("{}{}{}.{}", path, path_sep, name, ext);
                    let index_path = format!("{}{}{}{}{}.{}", path, path_sep, name, path_sep, "index", ext);
                    
                    if crate::is_debug_mode() {
                        println!("检查文件: {}", file_path);
                    }
                    
                    possible_files.push((file_path, loader.as_ref()));
                    possible_files.push((index_path, loader.as_ref()));
                }
            }
        }
        
        // 尝试所有可能的路径
        for (file_path, loader) in &possible_files {
            if loader.can_load(file_path) {
                if crate::is_debug_mode() {
                    println!("找到模块文件: {}", file_path);
                }
                return loader.load(name, file_path, options);
            }
        }
        
        // 未找到模块，返回详细的错误信息
        let search_paths = self.search_paths.join("\n  - ");
        let tested_paths = possible_files.iter()
            .map(|(path, _)| path.clone())
            .collect::<Vec<_>>()
            .join("\n  - ");
        
        Err(InterpreterError::ModuleError(
            format!("无法找到模块 '{}':\n搜索路径:\n  - {}\n尝试的文件路径:\n  - {}\n支持的扩展名: {}", 
                name, 
                search_paths,
                tested_paths,
                self.loaders.iter()
                    .flat_map(|l| l.get_supported_extensions())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        ))
    }
    
    /// 获取所有注册的加载器名称
    pub fn get_loader_names(&self) -> Vec<&'static str> {
        self.loaders.iter()
            .map(|loader| loader.get_loader_name())
            .collect()
    }
    
    /// 获取所有搜索路径
    pub fn get_search_paths(&self) -> &[String] {
        &self.search_paths
    }
    
    /// 检查指定名称的模块是否存在，返回找到的第一个模块路径和类型
    pub fn check_module_exists(&self, name: &str) -> Option<(String, String)> {
        let mut possible_files = Vec::new();
        
        for loader in &self.loaders {
            for ext in loader.get_supported_extensions() {
                for path in &self.search_paths {
                    let path_sep = std::path::MAIN_SEPARATOR;
                    
                    // 构建可能的文件路径
                    let file_path = format!("{}{}{}.{}", path, path_sep, name, ext);
                    let index_path = format!("{}{}{}{}{}.{}", path, path_sep, name, path_sep, "index", ext);
                    
                    possible_files.push((file_path, loader.as_ref()));
                    possible_files.push((index_path, loader.as_ref()));
                }
            }
        }
        
        // 检查所有可能的路径
        for (file_path, loader) in &possible_files {
            if loader.can_load(file_path) {
                return Some((file_path.clone(), loader.get_loader_name().to_string()));
            }
        }
        
        None
    }
    
    /// 检查是否存在同名但不同类型的模块文件，返回所有找到的冲突
    pub fn check_module_conflicts(&self, name: &str) -> Option<Vec<(String, String)>> {
        let mut found_modules = Vec::new();
        
        for loader in &self.loaders {
            for ext in loader.get_supported_extensions() {
                for path in &self.search_paths {
                    let path_sep = std::path::MAIN_SEPARATOR;
                    
                    // 构建可能的文件路径
                    let file_path = format!("{}{}{}.{}", path, path_sep, name, ext);
                    let index_path = format!("{}{}{}{}{}.{}", path, path_sep, name, path_sep, "index", ext);
                    
                    if loader.can_load(&file_path) {
                        found_modules.push((file_path, loader.get_loader_name().to_string()));
                    }
                    
                    if loader.can_load(&index_path) {
                        found_modules.push((index_path, loader.get_loader_name().to_string()));
                    }
                }
            }
        }
        
        // 如果找到多个模块，返回冲突列表
        if found_modules.len() > 1 {
            return Some(found_modules);
        }
        
        None
    }
}

/// 创建默认的模块注册表实例
pub fn create_default_registry() -> ModuleRegistry {
    // 创建注册表
    let mut registry = ModuleRegistry::new();
    
    // 检测程序文件的路径
    let exe_path = std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|p| p.to_string_lossy().to_string()));
    
    // 设置基础路径
    if let Some(path) = exe_path {
        registry.set_base_path(&path);
    } else {
        // 如果获取不到可执行文件路径，使用当前目录
        registry.set_base_path(".");
    }
    
    // 将系统环境变量中的模块路径添加到搜索路径
    if let Ok(paths) = std::env::var("JLANG_MODULE_PATH") {
        for path in paths.split(';') {
            if !path.is_empty() {
                registry.add_search_path(path.to_string());
            }
        }
    }
    
    // 返回注册表
    registry
}

/// JiLang模块加载器 - 用于加载.jl文件模块
pub struct JLangModuleLoader;

impl ModuleLoader for JLangModuleLoader {
    fn can_load(&self, path: &str) -> bool {
        let file_exists = std::path::Path::new(path).exists();
        let has_jl_ext = path.ends_with(".jl");
        
        if crate::is_debug_mode() {
            println!("检查模块文件: {} (存在: {}, 扩展名正确: {})", 
                     path, file_exists, has_jl_ext);
        }
        
        file_exists && has_jl_ext
    }
    
    fn load(&self, name: &str, path: &str, options: Option<ExternalModuleOptions>) -> Result<Box<dyn ExternalModule>> {
        if crate::is_debug_mode() {
            println!("加载模块: {} 从文件: {}", name, path);
        }
        
        // 检查文件是否存在
        if !std::path::Path::new(path).exists() {
            return Err(InterpreterError::ModuleError(
                format!("模块文件不存在: {}", path)
            ));
        }
        
        // 这里调用现有的JlModule创建逻辑，但将其包装为ExternalModule
        let jl_module = super::jl_module::JlModule::new(name, path)?;
        
        // 读取文件内容以提取module_meta
        let content = std::fs::read_to_string(path)
            .map_err(|e| InterpreterError::ModuleError(format!("无法读取文件 '{}': {}", path, e)))?;
        
        // 解析JSON
        let program: Value = serde_json::from_str(&content)
            .map_err(|e| InterpreterError::ModuleError(format!("无效的JSON格式: {}", e)))?;
        
        // 提取module_meta
        let module_meta = program.get("module_meta").cloned();
        
        if crate::is_debug_mode() {
            println!("成功加载模块: {}", name);
            // 输出模块中的函数
            println!("模块 {} 中的函数:", name);
            for (fname, _) in jl_module.get_functions() {
                println!("  - {}", fname);
            }
            
            if let Some(meta) = &module_meta {
                println!("模块定义了自定义元数据:");
                println!("{}", serde_json::to_string_pretty(&meta).unwrap_or_else(|_| "无法格式化元数据".to_string()));
            }
        }
        
        let metadata = create_metadata_for_jl_module(name, path)?;
        
        Ok(Box::new(JLangExternalModule {
            internal_module: jl_module,
            metadata,
            module_meta,
            options: options.unwrap_or_default(),
        }))
    }
    
    fn get_supported_extensions(&self) -> Vec<&'static str> {
        vec!["jl"]
    }
    
    fn get_loader_name(&self) -> &'static str {
        "JiLang模块加载器"
    }
}

/// 从JL模块创建元数据
fn create_metadata_for_jl_module(name: &str, path: &str) -> Result<ModuleMetadata> {
    // 读取文件内容
    let content = std::fs::read_to_string(path)
        .map_err(|e| InterpreterError::ModuleError(format!("无法读取文件 '{}': {}", path, e)))?;
    
    // 解析JSON
    let program: Value = serde_json::from_str(&content)
        .map_err(|e| InterpreterError::ModuleError(format!("无效的JSON格式: {}", e)))?;
    
    // 提取元数据
    let mut metadata = ModuleMetadata {
        name: name.to_string(),
        version: "1.0.0".to_string(),
        description: "JiLang模块".to_string(),
        author: "未知".to_string(),
        functions: HashMap::new(),
    };
    
    // 从模块中提取元信息，优先使用module_meta（0.4.0新格式）
    if let Some(meta) = program.get("module_meta").and_then(|m| m.as_object()) {
        if let Some(version) = meta.get("version").and_then(|v| v.as_str()) {
            metadata.version = version.to_string();
        }
        if let Some(description) = meta.get("description").and_then(|v| v.as_str()) {
            metadata.description = description.to_string();
        }
        if let Some(author) = meta.get("author").and_then(|v| v.as_str()) {
            metadata.author = author.to_string();
        }
    } 
    // 向后兼容：检查老式metadata格式
    else if let Some(meta) = program.get("metadata").and_then(|m| m.as_object()) {
        if let Some(version) = meta.get("version").and_then(|v| v.as_str()) {
            metadata.version = version.to_string();
        }
        if let Some(description) = meta.get("description").and_then(|v| v.as_str()) {
            metadata.description = description.to_string();
        }
        if let Some(author) = meta.get("author").and_then(|v| v.as_str()) {
            metadata.author = author.to_string();
        }
    }
    
    // 提取函数信息
    if let Some(program_obj) = program.get("program").and_then(|p| p.as_object()) {
        for (func_name, func_def) in program_obj {
            if func_name == "main" {
                continue; // 跳过main函数
            }
            
            // 创建函数元数据
            let mut function_meta = FunctionMetadata {
                name: func_name.clone(),
                description: "JiLang函数".to_string(),
                parameters: Vec::new(),
                return_type: "Any".to_string(),
                example: "".to_string(),
            };
            
            // 提取参数信息（如果有）
            if let Some(params) = func_def.get("params").and_then(|p| p.as_object()) {
                for (param_name, param_value) in params {
                    let param_meta = ParameterMetadata {
                        name: param_name.clone(),
                        description: "参数".to_string(),
                        type_description: if let Some(type_str) = param_value.as_str() {
                            type_str.to_string()
                        } else {
                            "Any".to_string()
                        },
                        optional: param_value.is_null(),
                        default_value: if param_value.is_null() { None } else { Some(param_value.clone()) },
                    };
                    function_meta.parameters.push(param_meta);
                }
            }
            
            // 从注释中提取函数描述（如果有）
            if let Some(body) = func_def.get("body").and_then(|b| b.as_array()) {
                if !body.is_empty() {
                    if let Some(first_stmt) = body.first() {
                        if let Some(obj) = first_stmt.as_object() {
                            if let Some(comment) = obj.get("comment") {
                                if let Some(comment_str) = comment.as_str() {
                                    function_meta.description = comment_str.to_string();
                                } else if let Some(comment_arr) = comment.as_array() {
                                    if !comment_arr.is_empty() {
                                        if let Some(desc) = comment_arr[0].as_str() {
                                            function_meta.description = desc.to_string();
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            // 添加到元数据
            metadata.functions.insert(func_name.clone(), function_meta);
        }
    }
    
    Ok(metadata)
}

/// JiLang模块的ExternalModule实现
pub struct JLangExternalModule {
    internal_module: super::jl_module::JlModule,
    metadata: ModuleMetadata,
    module_meta: Option<Value>, // 存储模块自定义元数据
    options: ExternalModuleOptions,
}

impl Module for JLangExternalModule {
    fn get_name(&self) -> &'static str {
        self.internal_module.get_name()
    }
    
    fn get_functions(&self) -> Vec<(&'static str, Box<dyn Fn(&[Value], &mut Context) -> Value + Send + Sync + 'static>)> {
        self.internal_module.get_functions()
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl ExternalModule for JLangExternalModule {
    fn get_module_type(&self) -> ExternalModuleType {
        ExternalModuleType::JLang
    }
    
    fn get_metadata(&self) -> &ModuleMetadata {
        &self.metadata
    }
    
    fn get_options(&self) -> &ExternalModuleOptions {
        &self.options
    }
    
    fn set_options(&mut self, options: ExternalModuleOptions) -> Result<()> {
        self.options = options;
        Ok(())
    }
    
    fn reload(&mut self) -> Result<()> {
        // JiLang模块目前不支持重新加载
        Err(InterpreterError::ModuleError(
            "JiLang模块不支持重新加载".to_string()
        ))
    }
    
    // 获取模块元数据值
    fn get_module_meta_value(&self) -> Option<&Value> {
        self.module_meta.as_ref()
    }
    
    // 添加一个新方法，获取函数定义但不执行
    // 这个方法可以避免call_function时的借用冲突
    fn get_jlang_function(&self, name: &str) -> Option<Value> {
        self.internal_module.get_function(name).cloned()
    }
    
    fn call_function(&self, name: &str, args: &[Value], context: &mut Context) -> Result<Value> {
        // 获取函数定义
        if let Some(func_def) = self.internal_module.get_function(name) {
            // 构建参数对象
            let mut params_map = serde_json::Map::new();
            
            // 获取函数参数定义
            if let Some(params) = func_def.get("params").and_then(|p| p.as_object()) {
                for (i, (param_name, _)) in params.iter().enumerate() {
                    if let Some(arg) = args.get(i) {
                        params_map.insert(param_name.clone(), arg.clone());
                    } else {
                        return Err(InterpreterError::FunctionError(
                            format!("缺少参数: {}", param_name)
                        ));
                    }
                }
            }
            
            let params = Value::Object(params_map);
            // 调用函数
            let result = crate::interpreter::statements::execute_function(func_def, context, Some(&params))?;
            Ok(result)
        } else {
            Err(InterpreterError::FunctionError(
                format!("函数 '{}' 不存在", name)
            ))
        }
    }
    
    // 获取函数的元数据（如果存在）
    fn get_function_metadata(&self, name: &str) -> Option<&FunctionMetadata> {
        self.metadata.functions.get(name)
    }
    
    fn has_function(&self, name: &str) -> bool {
        self.internal_module.get_function(name).is_some()
    }
    
    fn get_all_function_metadata(&self) -> Vec<&FunctionMetadata> {
        self.metadata.functions.values().collect()
    }
    
    fn initialize(&mut self) -> Result<()> {
        // JiLang模块不需要特殊初始化
        Ok(())
    }
    
    fn destroy(&mut self) -> Result<()> {
        // JiLang模块不需要特殊销毁
        Ok(())
    }
} 
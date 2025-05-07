#[derive(Debug)]
pub enum InterpreterError {
    InvalidProgramStructure(String),
    VariableError(String),
    FunctionError(String),
    ModuleError(String),
    RuntimeError(String),
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidProgramStructure(msg) => write!(f, "程序结构错误: {}。你管这叫代码？！这是垃圾！重写！", msg),
            Self::VariableError(msg) => write!(f, "变量错误: {}。呜哇！变量酱好混乱～", msg),
            Self::FunctionError(msg) => write!(f, "函数错误: {}。笨蛋！函数不是这样用的！", msg),
            Self::ModuleError(msg) => write!(f, "模块错误: {}。哼！模块加载失败了啦～", msg),
            Self::RuntimeError(msg) => write!(f, "运行时错误: {}。啊啦～程序员君不行呢～", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, InterpreterError>; 

// 错误消息常量
pub mod error_messages {
    // mod.rs相关错误消息
    pub mod interpreter {
        // 程序结构相关错误
        pub const MISSING_PROGRAM_MAIN_BODY: &str = "程序缺少 'program.main.body' 字段，这什么破代码！给我重写！";
        pub const PROGRAM_MAIN_BODY_NOT_ARRAY: &str = "'program.main.body' 必须是一个数组，看不懂文档吗？！";
        pub const STATEMENT_EMPTY: &str = "语句对象为空，你写了个寂寞？！";
        pub const STATEMENT_NOT_OBJECT: &str = "语句必须是一个对象，连这都不会，别写代码了！";
    }

    // Context.rs 相关错误消息
    pub mod context {
        // 程序结构相关错误
        pub const PROGRAM_NOT_OBJECT: &str = "程序必须是一个 JSON 对象，你交的什么垃圾？！";
        pub const CONST_NOT_OBJECT: &str = "'const' 必须是一个对象，这种错误都能犯，真是服了！";
        
        // 函数名冲突相关错误
        pub fn function_name_conflict_builtin(name: &str) -> String {
            format!("函数名 '{}' 与内置语句冲突，笨蛋！函数不是这样定义的！", name)
        }
        
        pub fn function_name_conflict_module(name: &str) -> String {
            format!("函数名 '{}' 与模块函数冲突，笨蛋！函数名字撞车了！", name)
        }
        
        // 变量相关错误
        pub fn constant_modification(name: &str) -> String {
            format!("无法修改常量 '{}'，呜呜呜～常量是不可变的啦～", name)
        }
        
        // 模块函数调用相关错误
        pub fn module_function_not_found(module: &str, function: &str) -> String {
            format!("模块 '{}' 中未找到函数 '{}'，噗～这个功能不存在的说～", module, function)
        }
        
        pub fn module_not_found(name: &str) -> String {
            format!("未找到模块 '{}'，诶嘿～这个模块根本不存在啦～", name)
        }
    }
    
    // Statement.rs 相关错误消息
    pub mod statement {
        // 通用的语句错误
        pub fn unknown_statement_type(name: &str) -> String {
            format!("未知的语句类型: {}，啊啦～这是什么奇怪的语句～", name)
        }
        
        pub const STATEMENT_EMPTY: &str = "语句对象为空，喂喂！语句不能是空的哦～";
        pub const STATEMENT_NOT_OBJECT: &str = "语句必须是一个对象，嘿嘿～这样写是不行的～";
        
        // 参数相关错误
        pub fn missing_parameter(param: &str) -> String {
            format!("缺少参数 '{}'，哎呀～参数不见了呢～", param)
        }
        
        pub fn param_must_be_obj(statement: &str) -> String {
            format!("'{}' 语句的参数必须是一个对象，嗯嗯～参数类型不对呢～", statement)
        }
        
        pub fn param_must_be_array(statement: &str) -> String {
            format!("'{}' 语句的参数必须是一个数组，唔～数组呢？数组不见了～", statement)
        }
        
        // 函数相关错误
        pub const FUNCTION_PARAMS_MUST_BE_OBJ: &str = "函数参数定义必须是对象，诶多～这个函数参数写错啦！";
        pub const FUNCTION_MISSING_PARAMS: &str = "函数缺少参数定义，喂！参数定义哪去了？";
        pub const FUNCTION_CALL_MISSING_NAME: &str = "函数调用缺少函数名，你要调用哪个函数啊，笨蛋！";
        pub const INVALID_FUNCTION_CALL: &str = "无效的函数调用，哼～函数调用失败啦！";
        pub const FUNCTION_MISSING_BODY: &str = "函数缺少 'body' 字段，欸？函数体不见了？";
        pub const FUNCTION_BODY_NOT_ARRAY: &str = "函数 'body' 必须是一个数组，呐呐～函数体必须是数组哦～";
        
        // 控制流相关错误
        pub mod control_flow {
            // if语句错误
            pub const IF_MISSING_FIELDS: &str = "'if' 语句缺少必要的字段，喂！条件判断写错了啦！";
            
            // while语句错误
            pub const WHILE_MISSING_FIELDS: &str = "'while' 语句缺少 'condition' 或 'body' 字段，啊嘞？while循环缺东西！";
            pub const WHILE_ARGS_NOT_OBJ: &str = "'while' 语句的参数必须是一个对象，嘿～这不是正确的while循环！";
            
            // for语句错误
            pub const FOR_MISSING_FIELDS: &str = "'for' 语句缺少必要的字段，哎呀呀～for循环写错了啦～";
            pub const FOR_RANGE_INVALID: &str = "'range' 必须是一个包含两个数字的数组，啊咧～range范围不对哦～";
            pub const FOR_ARGS_NOT_OBJ: &str = "'for' 语句的参数必须是一个对象，咦？for语句参数类型不对～";
        }
        
        // switch语句错误
        pub mod switch {
            pub const MISSING_EXPR_OR_CASES: &str = "'switch' 语句缺少 'expr' 或 'cases' 字段，嗯哼～switch写错啦～";
            pub const ARGS_NOT_OBJ: &str = "'switch' 语句的参数必须是一个对象，噫～switch参数不对哦～";
            pub const CASES_NOT_ARRAY: &str = "'cases' 必须是一个数组，啊咧咧～cases应该是数组啦～";
            pub const CASE_NOT_OBJ: &str = "case 必须是一个对象，喵～case应该是对象才对～";
            pub const CASE_MISSING_FIELDS: &str = "case 缺少 'value' 或 'body' 字段，呜～case缺少必要的字段～";
            pub const CASE_BODY_NOT_ARRAY: &str = "case 'body' 必须是一个数组，哇哦～case的body应该是数组呀～";
            pub const DEFAULT_BODY_NOT_ARRAY: &str = "default case 'body' 必须是一个数组，咿呀～default的body要是数组～";
        }
        
        // 数组操作错误
        pub mod array {
            pub const CREATE_SIZE_NOT_NUMBER: &str = "'array.create' 的 'size' 参数必须是一个数字，呜姆～数组大小应该是数字啦～";
            
            pub const PUSH_MISSING_ARGS: &str = "'array.push' 需要至少两个参数：数组和要添加的元素，嘤嘤～缺少push参数～";
            pub const PUSH_FIRST_ARG_NOT_ARRAY_REF: &str = "'array.push' 的第一个参数必须是一个数组变量引用，喵喵～这不是数组引用～";
            pub const PUSH_FIRST_ARG_NOT_STRING_REF: &str = "'array.push' 的第一个参数必须是一个字符串变量引用，呜喵～参数类型不对～";
            pub fn var_not_found(name: &str) -> String {
                format!("变量 '{}' 不存在，咦？这个变量去哪里了？", name)
            }
            pub fn var_not_array(name: &str) -> String {
                format!("变量 '{}' 不是一个数组，哇～这不是数组啦～", name)
            }
            
            pub const POP_MISSING_ARGS: &str = "'array.pop' 需要一个参数：数组引用，呐～pop需要参数哦～";
            pub const POP_ARG_NOT_ARRAY_REF: &str = "'array.pop' 的参数必须是一个数组变量引用，哎呀～这不是数组变量～";
            pub const POP_ARG_NOT_STRING_REF: &str = "'array.pop' 的参数必须是一个字符串变量引用，唔～变量引用写错了～";
            
            pub const GET_MISSING_ARGS: &str = "'array.get' 需要两个参数：数组和索引，啊啦～get参数不够呢～";
            pub const GET_FIRST_ARG_NOT_ARRAY_REF: &str = "'array.get' 的第一个参数必须是一个数组变量引用，诶嘿～第一个参数错啦～";
            pub const GET_FIRST_ARG_NOT_ARRAY: &str = "'array.get' 的第一个参数必须是一个数组，噗～这个不是数组啦～";
            pub const GET_SECOND_ARG_NOT_INDEX: &str = "'array.get' 的第二个参数必须是一个数字索引，喵呜～索引必须是数字～";
            pub fn var_not_exist(name: &str) -> String {
                format!("变量 '{}' 不存在，呜呜～找不到这个变量～", name)
            }
            
            pub const SET_MISSING_ARGS: &str = "'array.set' 需要三个参数：数组、索引和新值，哇哦～set参数不够哦～";
            pub const SET_FIRST_ARG_NOT_ARRAY_REF: &str = "'array.set' 的第一个参数必须是一个数组变量引用，咦咦～这不是数组引用～";
            pub const SET_FIRST_ARG_NOT_STRING_REF: &str = "'array.set' 的第一个参数必须是一个字符串变量引用，嘤～参数类型错误～";
            pub const SET_SECOND_ARG_NOT_INDEX: &str = "'array.set' 的第二个参数必须是一个数字索引，呀咧～索引要是数字啦～";
            
            pub const LENGTH_MISSING_ARGS: &str = "'array.length' 需要一个参数：数组，哎哟～length需要参数～";
            pub const LENGTH_ARG_NOT_ARRAY_REF: &str = "'array.length' 的参数必须是一个数组变量引用，啊嘞～这不是数组变量～";
            pub const LENGTH_ARG_NOT_ARRAY: &str = "'array.length' 的参数必须是一个数组，呜呼～这个不是数组啦～";
            
            pub const SLICE_MISSING_ARGS: &str = "'array.slice' 需要至少两个参数：数组和开始索引，喂喂～slice参数太少啦～";
            pub const SLICE_FIRST_ARG_NOT_ARRAY_REF: &str = "'array.slice' 的第一个参数必须是一个数组变量引用，咕～这不是数组引用～";
            pub const SLICE_FIRST_ARG_NOT_ARRAY: &str = "'array.slice' 的第一个参数必须是一个数组，呀～这不是数组啦～";
            pub const SLICE_SECOND_ARG_NOT_INDEX: &str = "'array.slice' 的第二个参数必须是一个数字索引，嗯嗯～索引应该是数字～";
            pub const SLICE_THIRD_ARG_NOT_INDEX: &str = "'array.slice' 的第三个参数必须是一个数字索引，哦～第三个参数也要是数字～";

            // 新增的错误消息函数
            pub fn get_first_arg_not_array_ref(name: &str) -> String {
                format!("'array.get' 的第一个参数 '{}' 必须是一个数组变量引用，嘿～这不对哦～", name)
            }

            pub fn first_arg_not_array(name: &str) -> String {
                format!("'{}' 不是一个数组，诶多～这不是数组啦～", name)
            }

            pub fn get_second_arg_must_be_number(name: &str) -> String {
                format!("'{}' 必须是一个数字索引，喵喵～索引要用数字～", name)
            }

            pub fn set_first_arg_not_array_ref(name: &str) -> String {
                format!("'array.set' 的第一个参数 '{}' 必须是一个数组变量引用，呜～参数错误～", name)
            }

            pub fn set_first_arg_not_string_ref() -> String {
                "'array.set' 的第一个参数必须是一个字符串变量引用，哎呀～参数类型不对～".to_string()
            }

            pub fn set_second_arg_must_be_number() -> String {
                "'array.set' 的第二个参数必须是一个数字索引，嘤嘤～索引要是数字啦～".to_string()
            }

            pub fn get_third_arg_must_be_number() -> String {
                "'array.slice' 的第三个参数必须是一个数字索引，咿呀～这个也要是数字～".to_string()
            }
        }
        
        // exec命令执行错误
        pub mod exec {
            pub const MISSING_CMD: &str = "'exec' 语句缺少 'cmd' 字段，啊咧～命令哪去了？";
            pub const ARGS_NOT_OBJ: &str = "'exec' 语句的参数必须是一个对象，喵呜～参数不是对象啦～";
            pub fn execution_failed(err: &str) -> String {
                format!("执行命令失败: {}，哎哟～命令执行炸了～", err)
            }
        }
    }
} 
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
            Self::InvalidProgramStructure(msg) => write!(f, "程序结构错误: {}。回去重修JsonLang文档！", msg),
            Self::VariableError(msg) => write!(f, "变量错误: {}。杂鱼~", msg),
            Self::FunctionError(msg) => write!(f, "函数错误: {}。杂鱼~", msg),
            Self::ModuleError(msg) => write!(f, "模块错误: {}。杂鱼~", msg),
            Self::RuntimeError(msg) => write!(f, "运行时错误: {}。杂鱼~", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, InterpreterError>; 
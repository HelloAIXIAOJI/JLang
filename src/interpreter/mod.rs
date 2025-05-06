pub mod context;
pub mod error;
pub mod statement;

use serde_json::Value;
use crate::modules::Module;
use context::Context;
use error::{InterpreterError, Result};
use statement::execute_statement;

pub struct Interpreter {
    context: Context,
}

impl Interpreter {
    pub fn new(program: Value, modules: Vec<Box<dyn Module>>) -> Result<Self> {
        let context = Context::new(program, modules)?;
        Ok(Self { context })
    }

    pub fn run(&mut self) -> Result<()> {
        // 获取主程序体
        let program_body = self.context.program.get("program")
            .and_then(|p| p.get("main"))
            .and_then(|m| m.get("body"))
            .ok_or_else(|| InterpreterError::InvalidProgramStructure(
                "杂鱼~程序缺少 'program.main.body' 字段".to_string()
            ))?;

        // 验证主程序体是数组
        let statements = program_body.as_array()
            .ok_or_else(|| InterpreterError::InvalidProgramStructure(
                "杂鱼~'program.main.body' 必须是一个数组".to_string()
            ))?;

        // 执行每个语句
        for stmt in statements.to_vec() {
            if let Some(obj) = stmt.as_object() {
                if let Some((stmt_type, args)) = obj.iter().next() {
                    execute_statement(stmt_type, args, &mut self.context)?;
                } else {
                    return Err(InterpreterError::RuntimeError(
                        "杂鱼~语句对象为空".to_string()
                    ));
                }
            } else {
                return Err(InterpreterError::RuntimeError(
                    "杂鱼~语句必须是一个对象".to_string()
                ));
            }
        }

        Ok(())
    }
} 
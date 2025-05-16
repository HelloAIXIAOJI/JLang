pub mod context;
pub mod error;
pub mod statements;
pub mod variable_reference;

use serde_json::Value;
use crate::modules::Module;
use crate::{is_ignore_non_critical_errors, is_check_only, is_check_all};
use context::Context;
use error::{InterpreterError, Result};
use error::error_messages::interpreter;
use statements::execute_statement;

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
                interpreter::MISSING_PROGRAM_MAIN_BODY.to_string()
            ))?;

        // 验证主程序体是数组
        let statements = program_body.as_array()
            .ok_or_else(|| InterpreterError::InvalidProgramStructure(
                interpreter::PROGRAM_MAIN_BODY_NOT_ARRAY.to_string()
            ))?;

        // 如果是仅检查模式，则不执行语句
        if is_check_only() && !is_check_all() {
            return Ok(());
        }

        // 执行每个语句
        for stmt in statements.to_vec() {
            if let Some(obj) = stmt.as_object() {
                if let Some((stmt_type, args)) = obj.iter().next() {
                    // 将完整语句对象传递给execute_statement
                    match execute_statement(stmt_type, args, &mut self.context, Some(&stmt)) {
                        Ok(_) => {},
                        Err(e) => {
                            // 在全面检查模式下，只收集错误而不终止
                            if is_check_all() {
                                // 在check_all模式下不做任何处理，我们已经记录了错误
                                // 通过run的返回值传递到主程序来显示
                                return Err(e);
                            }
                            // 在容错模式下，对于非关键错误只报告错误但继续执行
                            else if is_ignore_non_critical_errors() {
                                match &e {
                                    InterpreterError::InvalidProgramStructure(_) => return Err(e),
                                    _ => {
                                        // 报告错误但不终止执行
                                        eprintln!("警告: {}，但继续执行", e);
                                        continue;
                                    }
                                }
                            } else {
                                // 在正常模式下，任何错误都终止执行
                                return Err(e);
                            }
                        }
                    }
                } else {
                    // 空语句，始终被视为关键错误
                    return Err(InterpreterError::RuntimeError(
                        interpreter::STATEMENT_EMPTY.to_string()
                    ));
                }
            } else {
                // 无效语句对象，始终被视为关键错误
                return Err(InterpreterError::RuntimeError(
                    interpreter::STATEMENT_NOT_OBJECT.to_string()
                ));
            }
        }

        Ok(())
    }

    // 添加新方法用于全面检查，返回所有发现的错误
    pub fn check_all(&mut self) -> Vec<String> {
        let mut errors = Vec::new();

        // 获取主程序体
        let program_body = match self.context.program.get("program")
            .and_then(|p| p.get("main"))
            .and_then(|m| m.get("body")) {
                Some(body) => body,
                None => {
                    errors.push(interpreter::MISSING_PROGRAM_MAIN_BODY.to_string());
                    return errors;
                }
            };

        // 验证主程序体是数组
        let statements = match program_body.as_array() {
            Some(stmts) => stmts.clone(), // 克隆语句数组，避免借用冲突
            None => {
                errors.push(interpreter::PROGRAM_MAIN_BODY_NOT_ARRAY.to_string());
                return errors;
            }
        };

        // 检查每个语句
        for (i, stmt) in statements.iter().enumerate() {
            if let Some(obj) = stmt.as_object() {
                if let Some((stmt_type, args)) = obj.iter().next() {
                    match execute_statement(stmt_type, args, &mut self.context, Some(stmt)) {
                        Ok(_) => {},
                        Err(e) => {
                            // 记录错误信息及其位置
                            errors.push(format!("第{}个语句错误: {}", i + 1, e));
                        }
                    }
                } else {
                    errors.push(format!("第{}个语句错误: {}", i + 1, interpreter::STATEMENT_EMPTY));
                }
            } else {
                errors.push(format!("第{}个语句错误: {}", i + 1, interpreter::STATEMENT_NOT_OBJECT));
            }
        }

        errors
    }
} 
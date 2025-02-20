mod ast;
mod variables;

use variables::Variables;

use std::collections::HashMap;

use crate::vm::executable::{Executable, OpCode};

use anyhow::Result;
use ast::Program;

const STACK_SIZE: u32 = 100;

pub fn compile(input: &str) -> Result<String> {
    let ast: Program = serde_json::from_str(input)?;

    let variables = Variables::new(ast.variables)?;

    let mut code= Vec::new();
    
    let exec = Executable::new(STACK_SIZE as u32, variables.len() as u32, code);
    return Ok(exec.to_text());
}

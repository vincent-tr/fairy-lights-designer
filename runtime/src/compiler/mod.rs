mod ast;
mod variables;

use log::info;
use variables::Variables;

use crate::vm::executable::Executable;

use anyhow::Result;
use ast::Program;

const STACK_SIZE: u32 = 100;

pub fn compile(input: &str) -> Result<String> {
    let program: Program = serde_json::from_str(input)?;

    info!("Got input program:\n{}", program);

    let variables = Variables::new(program.variables)?;

    let mut code= Vec::new();
    
    let exec = Executable::new(STACK_SIZE as u32, variables.len() as u32, code);

    info!("Compiled into executable:\n{}", exec);

    return Ok(exec.to_text());
}

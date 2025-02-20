mod ast;

use crate::vm::opcodes;

use anyhow::Result;
use ast::Program;

pub fn compile(input: &str) -> Result<String> {
    let ast: Program = serde_json::from_str(input)?;
    return Ok(format!("{:?}", ast));
}
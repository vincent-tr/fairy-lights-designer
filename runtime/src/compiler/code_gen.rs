use crate::vm::executable::OpCode;
use anyhow::Result;

pub struct CodeGen {
    code: Vec<OpCode>,
}

pub struct Updateable {
    index: usize,
}

impl Updateable {
    pub fn compute_relative_offset(&self, index: usize) -> i32 {
        (index as i32) - (self.index as i32)
    }

    pub fn update_jump(&self, code: &mut CodeGen, relative_offset: i32) -> Result<()> {
        code.code[self.index] = OpCode::Jump { relative_offset: relative_offset.try_into()? };

        Ok(())
    }

    pub fn update_jump_if(&self, code: &mut CodeGen, relative_offset: i32) -> Result<()> {
        code.code[self.index] = OpCode::JumpIf { relative_offset: relative_offset.try_into()? };

        Ok(())
    }
}

impl CodeGen {
    pub fn new() -> Self {
        CodeGen { code: Vec::new() }
    }

    pub fn emit(&mut self, op: OpCode) -> Updateable {
        let index = self.code.len();

        self.code.push(op);

        Updateable { index }
    }

    pub fn build(self) -> Vec<OpCode> {
        self.code
    }

    pub fn current_index(&self) -> usize {
        self.code.len()
    }

    pub fn compute_relative_offset(&self, index: usize) -> i32 {
        (index as i32) - (self.current_index() as i32)
    }
}

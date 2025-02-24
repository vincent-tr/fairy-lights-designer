use crate::vm::{executable::OpCode, i24::i24};

use super::{CodeGen, Updateable};
use anyhow::{Context, Result};

pub struct LoopManager {
    label_begin: usize,
    jumps_to_end: Vec<Updateable>,
}

impl LoopManager {
    fn begin(code: &mut CodeGen) -> Self {
        Self {
            label_begin: code.current_index(),
            jumps_to_end: Vec::new(),
        }
    }

    fn emit_continue(&mut self, code: &mut CodeGen) -> Result<()> {
        // jump to label_begin
        code.emit(OpCode::Jump {
            relative_offset: code.compute_relative_offset(self.label_begin).try_into()?,
        });

        Ok(())
    }

    fn emit_break(&mut self, code: &mut CodeGen) -> Result<()> {
        // emit dummy jump for now
        let updateable = code.emit(OpCode::Jump {
            relative_offset: i24::ZERO,
        });

        self.jumps_to_end.push(updateable);

        Ok(())
    }

    fn end(self, code: &mut CodeGen) -> Result<()> {
        // end_label is now the current index
        let label_end = code.current_index();

        // fix all jumps to end
        for jump in self.jumps_to_end {
            jump.update_jump(code, jump.compute_relative_offset(label_end).try_into()?)?;
        }

        Ok(())
    }
}

pub struct LoopManagerStack {
    stack: Vec<LoopManager>,
}

impl LoopManagerStack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn end(self) -> Result<()> {
        if !self.stack.is_empty() {
            return Err(anyhow::anyhow!("LoopManagerStack not empty"));
        }

        Ok(())
    }

    pub fn begin_loop(&mut self, code: &mut CodeGen) {
        self.stack.push(LoopManager::begin(code));
    }

    pub fn end_loop(&mut self, code: &mut CodeGen) -> Result<()> {
        let manager = self
            .stack
            .pop()
            .context("end_loop called without begin_loop")?;

        manager.end(code)
    }

    pub fn emit_continue(&mut self, code: &mut CodeGen) -> Result<()> {
        let current = self
            .stack
            .last_mut()
            .context("emit_continue called without begin_loop")?;

        current.emit_continue(code)
    }

    pub fn emit_break(&mut self, code: &mut CodeGen) -> Result<()> {
        let current = self
            .stack
            .last_mut()
            .context("emit_continue called without begin_loop")?;

        current.emit_break(code)
    }
}

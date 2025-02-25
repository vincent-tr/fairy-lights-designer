use std::{sync::Arc, time::Duration};

use super::{ExternalApi, OpCode};
use anyhow::Result;
use wasm_timer::SystemTime;

pub struct Machine {
    locals: Box<[i32]>,
    stack: Box<[i32]>,
    stack_index: usize,
    instructions: Box<[OpCode]>,
    instruction_index: usize,
    api: Arc<dyn ExternalApi>,
    wakeup_time: SystemTime,
}

impl Machine {
    pub fn load_executable(exec: super::Executable, api: Arc<dyn ExternalApi>) -> Self {
        Self {
            locals: vec![0; exec.locals_size()].into_boxed_slice(),
            stack: vec![0; exec.stack_size()].into_boxed_slice(),
            stack_index: 0,
            instructions: exec.code().into(),
            instruction_index: 0,
            api,
            wakeup_time: SystemTime::now(),
        }
    }

    pub fn get_local(&self, index: usize) -> Result<i32> {
        let local = self
            .locals
            .get(index)
            .ok_or_else(|| anyhow::anyhow!("Invalid local index: {}", index))?;
        Ok(*local)
    }

    pub fn set_local(&mut self, index: usize, value: i32) -> Result<()> {
        let local = self
            .locals
            .get_mut(index)
            .ok_or_else(|| anyhow::anyhow!("Invalid local index: {}", index))?;
        *local = value;
        Ok(())
    }

    pub fn push(&mut self, value: i32) -> Result<()> {
        if self.stack_index == self.stack.len() {
            anyhow::bail!("Stack overflow");
        }

        self.stack[self.stack_index] = value;
        self.stack_index += 1;

        Ok(())
    }

    pub fn pop(&mut self) -> Result<i32> {
        if self.stack_index == 0 {
            anyhow::bail!("Stack underflow");
        }

        self.stack_index -= 1;
        Ok(self.stack[self.stack_index])
    }

    pub fn fetch_instruction(&mut self) -> Result<OpCode> {
        let instruction = self
            .instructions
            .get(self.instruction_index)
            .copied()
            .ok_or_else(|| {
                anyhow::anyhow!("Invalid instruction index: {}", self.instruction_index)
            })?;
        self.instruction_index += 1;
        Ok(instruction)
    }

    pub fn jump(&mut self, relative_offset: i32) -> Result<()> {
        // instruction_index points to the next instruction, but relative offset is relative to the current instruction
        let new_index = self.instruction_index as i32 - 1 + relative_offset;
        if new_index < 0 || new_index as usize >= self.instructions.len() {
            anyhow::bail!("Invalid jump target: {}", new_index);
        }

        self.instruction_index = new_index as usize;
        Ok(())
    }

    pub fn sleep(&mut self, duration: Duration) {
        self.wakeup_time = SystemTime::now() + duration;
    }

    pub fn external_api(&self) -> &dyn ExternalApi {
        &*self.api
    }

    pub fn sleeping(&self) -> bool {
        self.wakeup_time > SystemTime::now()
    }

}

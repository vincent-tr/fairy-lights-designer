pub mod executable;
pub mod i24;
mod machine;
mod instructions;

use std::sync::Arc;

use anyhow::Result;
use executable::{Executable, OpCode};
use log::{error, info};
use machine::Machine;

pub trait ExternalApi {
    fn rand(&self, min: i32, max: i32) -> i32;

    fn len(&self) -> usize;
    fn get(&self, index: usize) -> (u8, u8, u8);
    fn set(&self, index: usize, color: (u8, u8, u8));
}

pub struct VM {
    api: Arc<dyn ExternalApi>,
    state: State,
}

impl VM {
    pub fn new(api: Box<dyn ExternalApi>) -> Self {
        Self { 
            api: Arc::from(api),
            state: State::new(),
        }
    }

    pub fn running(&self) -> bool {
        self.state.running()
    }

    pub fn load_executable(&mut self, exec: Executable) {
        info!("Loading executable: {}", exec);

        let machine = Machine::load_executable(exec, self.api.clone());
        self.state.start(machine);
    }

    pub fn tick(&mut self) {
        match &mut self.state {
            State::Running(state) => {
                match state.tick() {
                    Ok(()) => {}
                    Err(e) => {
                        error!("{}", e);
                        self.state.stop();
                    }
                }
            }
            State::Stopped => {}
        }
    }

}

enum State {
    Running(RunningState),
    Stopped,
}

impl State {
    pub fn new() -> Self {
        State::Stopped
    }

    pub fn running(&self) -> bool {
        match self {
            State::Running(_) => true,
            State::Stopped => false,
        }
    }

    pub fn start(&mut self, machine: Machine) {
        *self = State::Running(RunningState::new(machine));
    }

    pub fn stop(&mut self) {
        *self = State::Stopped;
    }
}

struct RunningState {
    machine: Machine,
}

impl Drop for RunningState {
    fn drop(&mut self) {
        info!("VM stopped");
    }
}

impl RunningState {
    pub fn new(machine: Machine) -> Self {
        info!("VM started");
        Self { machine }
    }

    pub fn tick(&mut self) -> Result<()> {
        let mut loop_guard = LoopGuard::new();
        
        loop {
            if self.machine.sleeping() {
                break;
            }

            loop_guard.next()?;

            let opcode = self.machine.fetch_instruction()?;
            instructions::execute(&mut self.machine, opcode)?;
        }

        Ok(())
    }
}

struct LoopGuard {
    count: usize,
}

impl LoopGuard {
    const LIMIT: usize = 10000;

    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn next(&mut self) -> Result<()> {
        self.count += 1;
        if self.count > Self::LIMIT {
            anyhow::bail!("Infinite loop detected");
        }

        Ok(())
    }
}
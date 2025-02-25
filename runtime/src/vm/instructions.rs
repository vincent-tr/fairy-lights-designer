
use std::time::Duration;

use super::{i24::i24, Machine, OpCode};
use anyhow::Result;

pub fn execute(machine: &mut Machine, opcode: OpCode) -> Result<()> {
    // debug!("Executing opcode: {:?}", opcode);
    match opcode {
        OpCode::PushConstant { value } => push_constant(machine, value),
        OpCode::PushVariable { index } => push_variable(machine, index),
        OpCode::PopVariable { index } => pop_variable(machine, index),
        OpCode::Pop => pop(machine),
        OpCode::Equal => comparer(machine, |op1, op2| op1 == op2),
        OpCode::NotEqual => comparer(machine, |op1, op2| op1 != op2),
        OpCode::Less => comparer(machine, |op1, op2| op1 < op2),
        OpCode::LessEqual => comparer(machine, |op1, op2| op1 <= op2),
        OpCode::And => logic(machine, |op1, op2| op1 && op2),
        OpCode::Or => logic(machine, |op1, op2| op1 || op2),
        OpCode::Not => not(machine),
        OpCode::Add => arithmetic(machine, |op1, op2| op1 + op2),
        OpCode::Sub => arithmetic(machine, |op1, op2| op1 - op2),
        OpCode::Mul => arithmetic(machine, |op1, op2| op1 * op2),
        OpCode::Div => arithmetic(machine, |op1, op2| op1 / op2),
        OpCode::Pow => pow(machine),
        OpCode::Mod => arithmetic(machine, |op1, op2| op1 % op2),
        OpCode::Jump { relative_offset } => jump(machine, relative_offset),
        OpCode::JumpIf { relative_offset } => jump_if(machine, relative_offset),
        OpCode::Rand => rand(machine),
        OpCode::Len => len(machine),
        OpCode::GetRed => get_red(machine),
        OpCode::GetGreen => get_green(machine),
        OpCode::GetBlue => get_blue(machine),
        OpCode::Set => set(machine),
        OpCode::Sleep => sleep(machine),
    }
}

fn push_constant(machine: &mut Machine, value: i24) -> Result<()> {
    machine.push(value.try_into()?)?;

    Ok(())
}

fn push_variable(machine: &mut Machine, index: u8) -> Result<()> {
    let value = machine.get_local(index as usize)?;
    machine.push(value)?;

    Ok(())
}

fn pop_variable(machine: &mut Machine, index: u8) -> Result<()> {
    let value = machine.pop()?;
    machine.set_local(index as usize, value)?;

    Ok(())
}

fn pop(machine: &mut Machine) -> Result<()> {
    machine.pop()?;

    Ok(())
}

fn comparer(machine: &mut Machine, op: fn(i32, i32) -> bool) -> Result<()> {
    let op2 = machine.pop()?;
    let op1 = machine.pop()?;

    let result = if op(op1, op2) { 1 } else { 0 };

    machine.push(result)?;

    Ok(())
}

fn logic(machine: &mut Machine, op: fn(bool, bool) -> bool) -> Result<()> {
    let op2 = machine.pop()?;
    let op1 = machine.pop()?;

    let result = if op(op1 != 0, op2 != 0) { 1 } else { 0 };

    machine.push(result)?;

    Ok(())
}

fn not(machine: &mut Machine) -> Result<()> {
    let op = machine.pop()?;

    let result = if op != 0 { 0 } else { 1 };

    machine.push(result)?;

    Ok(())
}

fn arithmetic(machine: &mut Machine, op: fn(i32, i32) -> i32) -> Result<()> {
    let op2 = machine.pop()?;
    let op1 = machine.pop()?;

    let result = op(op1, op2);

    machine.push(result)?;

    Ok(())
}

fn pow(machine: &mut Machine) -> Result<()> {
    let op2 = machine.pop()?;
    let op1 = machine.pop()?;

    if op2 < 0 {
        anyhow::bail!("Runtime error: Exponent must be non-negative");
    }

    let result = op1.pow(op2 as u32);

    machine.push(result)?;

    Ok(())
}

fn jump(machine: &mut Machine, relative_offset: i24) -> Result<()> {
    let offset = relative_offset.try_into()?;
    machine.jump(offset)?;

    Ok(())
}

fn jump_if(machine: &mut Machine, relative_offset: i24) -> Result<()> {
    let condition = machine.pop()?;
    if condition != 0 {
        let offset = relative_offset.try_into()?;
        machine.jump(offset)?;
    }

    Ok(())
}

fn rand(machine: &mut Machine) -> Result<()> {
    let max = machine.pop()?;
    let min = machine.pop()?;

    let result = machine.external_api().rand(min, max);

    machine.push(result)?;

    Ok(())
}

fn len(machine: &mut Machine) -> Result<()> {
    let result = machine.external_api().len();

    machine.push(result.try_into()?)?;

    Ok(())
}

fn get_red(machine: &mut Machine) -> Result<()> {
    let index = machine.pop()?;

    if index < 0 {
        anyhow::bail!("Runtime error: Index must be non-negative");
    }

    let (red, _green, _blue) = machine.external_api().get(index as usize);

    machine.push(red as i32)?;

    Ok(())
}

fn get_green(machine: &mut Machine) -> Result<()> {
    let index = machine.pop()?;

    if index < 0 {
        anyhow::bail!("Runtime error: Index must be non-negative");
    }

    let (_red, green, _blue) = machine.external_api().get(index as usize);

    machine.push(green as i32)?;

    Ok(())
}

fn get_blue(machine: &mut Machine) -> Result<()> {
    let index = machine.pop()?;

    if index < 0 {
        anyhow::bail!("Runtime error: Index must be non-negative");
    }

    let (_red, _green, blue) = machine.external_api().get(index as usize);

    machine.push(blue as i32)?;

    Ok(())
}

fn set(machine: &mut Machine) -> Result<()> {
    let blue = machine.pop()?;
    let green = machine.pop()?;
    let red = machine.pop()?;
    let index = machine.pop()?;

    if index < 0 {
        anyhow::bail!("Runtime error: Index must be non-negative");
    }

    if red < 0 || red > 255 {
        anyhow::bail!("Runtime error: Red must be in the range 0-255");
    }

    if green < 0 || green > 255 {
        anyhow::bail!("Runtime error: Red must be in the range 0-255");
    }

    if blue < 0 || blue > 255 {
        anyhow::bail!("Runtime error: Red must be in the range 0-255");
    }

    machine.external_api().set(index as usize, (red as u8, green as u8, blue as u8));

    Ok(())
}

fn sleep(machine: &mut Machine) -> Result<()> {
    let duration = machine.pop()?;
    let duration = Duration::from_millis(duration as u64);
    machine.sleep(duration);

    Ok(())
}

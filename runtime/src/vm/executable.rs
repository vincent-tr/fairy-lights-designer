use std::io::{self, Cursor};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use super::i24::i24;

const MAGIC: u32 = 0x00BABE00;

pub struct Executable {
    stack_size: u32,
    locals_size: u32,
    code: Vec<OpCode>,
}

impl Executable {
    pub fn from_raw(raw: &[u8]) -> io::Result<Self> {
        let mut reader = Cursor::new(raw);

        if !reader.read_u32::<LittleEndian>()? == MAGIC {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid magic number"));
        }

        let stack_size = reader.read_u32::<LittleEndian>()?;
        let locals_size = reader.read_u32::<LittleEndian>()?;

        let mut code = Vec::new();
        while (reader.position() as usize) < reader.get_ref().len() {
            let op =  OpCode::from_raw(reader.read_u32::<LittleEndian>()?);
            code.push(op);
        }

        Ok(Self { 
            stack_size,
            locals_size,
            code,
        })
    }

    pub fn ro_raw(&self) -> io::Result<Box<[u8]>> {
        let mut writer = Cursor::new(Vec::new());

        writer.write_u32::<LittleEndian>(MAGIC)?;
        writer.write_u32::<LittleEndian>(self.stack_size)?;
        writer.write_u32::<LittleEndian>(self.locals_size)?;

        for op in &self.code {
            writer.write_u32::<LittleEndian>(op.to_raw())?;
        }

        Ok(writer.into_inner().into_boxed_slice())
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    // Stack management
    PushConstant { value: i24 },
    PushTrue,
    PushFalse,
    PushVariable { index: u8 },
    PopVariable { index: u8 },
    Pop,

    // Compare
    Eq,
    Neq,
    Lt,
    Lte,
    Gt,
    Gte,
  
    // Logic
    And,
    Or,
    Not,

    // Jump
    Jump { relative_offset: i24 },
    JumIf { relative_offset: i24 },

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    
    // Api
    Len,
    Get,
    Set,
    Sleep,
}

static_assertions::const_assert_eq!(std::mem::size_of::<OpCode>(), 4);

impl OpCode {
    pub fn from_raw(raw: u32) -> Self {
        unsafe { std::mem::transmute(raw) }
    }

    pub fn to_raw(&self) -> u32 {
        unsafe { std::mem::transmute(*self) }
    }

    pub fn push_constant(value: i32) -> anyhow::Result<Self> {
        Ok(Self::PushConstant { value: i24::try_from(value)? })
    }

    pub fn push_true() -> Self {
        Self::PushTrue
    }

    pub fn push_false() -> Self {
        Self::PushFalse
    }

    pub fn push_variable(index: u8) -> Self {
        Self::PushVariable { index }
    }

    pub fn pop_variable(index: u8) -> Self {
        Self::PopVariable { index }
    }

    pub fn pop() -> Self {
        Self::Pop
    }

    pub fn eq() -> Self {
        Self::Eq
    }

    pub fn neq() -> Self {
        Self::Neq
    }

    pub fn lt() -> Self {
        Self::Lt
    }

    pub fn lte() -> Self {
        Self::Lte
    }

    pub fn gt() -> Self {
        Self::Gt
    }

    pub fn gte() -> Self {
        Self::Gte
    }

    pub fn and() -> Self {
        Self::And
    }

    pub fn or() -> Self {
        Self::Or
    }

    pub fn not() -> Self {
        Self::Not
    }

    pub fn jump(relative_offset: i32) -> anyhow::Result<Self> {
        Ok(Self::Jump { relative_offset: i24::try_from(relative_offset)? })
    }

    pub fn jump_if(relative_offset: i32) -> anyhow::Result<Self> {
        Ok(Self::JumIf { relative_offset: i24::try_from(relative_offset)? })
    }

    pub fn add() -> Self {
        Self::Add
    }

    pub fn sub() -> Self {
        Self::Sub
    }

    pub fn mul() -> Self {
        Self::Mul
    }

    pub fn div() -> Self {
        Self::Div
    }

    pub fn pow() -> Self {
        Self::Pow
    }

    pub fn mod_() -> Self {
        Self::Mod
    }

    pub fn len() -> Self {
        Self::Len
    }

    pub fn get() -> Self {
        Self::Get
    }

    pub fn set() -> Self {
        Self::Set
    }

    pub fn sleep() -> Self {
        Self::Sleep
    }
}

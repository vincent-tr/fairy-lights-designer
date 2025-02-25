use std::{fmt, io::Cursor};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use super::i24::i24;
use anyhow::Result;

const MAGIC: u32 = 0x00BABE00;

pub struct Executable {
    stack_size: u32,
    locals_size: u32,
    code: Vec<OpCode>,
}

impl Executable {
    pub fn from_raw(raw: &[u8]) -> Result<Self> {
        let mut reader = Cursor::new(raw);

        if reader.read_u32::<LittleEndian>()? != MAGIC {
            anyhow::bail!("Invalid magic number");
        }

        if reader.read_u32::<LittleEndian>()? != Self::compute_crc(raw) {
            anyhow::bail!("Invalid CRC");
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

    pub fn to_raw(&self) -> Box<[u8]> {
        const CRC_OFFSET: u64 = 4; // after magic

        let mut writer = Cursor::new(Vec::new());

        writer.write_u32::<LittleEndian>(MAGIC).unwrap();
        writer.write_u32::<LittleEndian>(0).unwrap(); // CRC placeholder
        writer.write_u32::<LittleEndian>(self.stack_size).unwrap();
        writer.write_u32::<LittleEndian>(self.locals_size).unwrap();

        for op in &self.code {
            writer.write_u32::<LittleEndian>(op.to_raw()).unwrap();
        }

        let mut raw = writer.into_inner().into_boxed_slice();
        let crc = Self::compute_crc(&raw);        

        let mut writer = Cursor::new(&mut *raw);
        writer.set_position(CRC_OFFSET);
        writer.write_u32::<LittleEndian>(crc).unwrap();

        raw
    }

    fn compute_crc(raw: &[u8]) -> u32 {
        // skip first 2 u32: MAGIC and CRC
        let data = &raw[8..];

        crc::Crc::<u32>::new(&crc::CRC_32_CKSUM).checksum(data)
    }

    pub fn from_text(text: &str) -> Result<Self> {
        use base64::{Engine as _, engine::general_purpose};

        let raw = general_purpose::STANDARD_NO_PAD.decode(text)?;
        Self::from_raw(&raw)
    }

    pub fn to_text(&self) -> String {
        use base64::{Engine as _, engine::general_purpose};

        general_purpose::STANDARD_NO_PAD.encode(self.to_raw())
    }

    pub fn new(stack_size: u32, locals_size: u32, code: Vec<OpCode>) -> Self {
        Self {
            stack_size,
            locals_size,
            code,
        }
    }

    pub fn stack_size(&self) -> usize {
        self.stack_size as usize
    }

    pub fn locals_size(&self) -> usize {
        self.locals_size as usize
    }

    pub fn code(&self) -> &[OpCode] {
        &self.code
    }
}

impl fmt::Display for Executable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Executable")?;
        writeln!(f, "  StackSize={}", self.stack_size)?;
        writeln!(f, "  LocalsSize={}", self.locals_size)?;
        writeln!(f, "")?;

        for op in &self.code {
            writeln!(f, "  {}, ", op)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum OpCode {
    // Stack management
    PushConstant { value: i24 },
    PushVariable { index: u8 },
    PopVariable { index: u8 },
    Pop,

    // Compare
    Equal,
    NotEqual,
    Less,
    LessEqual,
  
    // Logic
    And,
    Or,
    Not,

    // Jump
    Jump { relative_offset: i24 },
    JumpIf { relative_offset: i24 },

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Mod,
    
    // Api
    Rand,
    Len,
    GetRed,
    GetGreen,
    GetBlue,
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
}

impl fmt::Display for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OpCode::PushConstant { value } => write!(f, "PushConstant({})", Into::<i32>::into(*value)),
            OpCode::PushVariable { index } => write!(f, "PushVariable({})", index),
            OpCode::PopVariable { index } => write!(f, "PopVariable({})", index),
            OpCode::Pop => write!(f, "Pop"),
            OpCode::Equal => write!(f, "Equal"),
            OpCode::NotEqual => write!(f, "NotEqual"),
            OpCode::Less => write!(f, "Less"),
            OpCode::LessEqual => write!(f, "LessEqual"),
            OpCode::And => write!(f, "And"),
            OpCode::Or => write!(f, "Or"),
            OpCode::Not => write!(f, "Not"),
            OpCode::Jump { relative_offset } => write!(f, "Jump({})", Into::<i32>::into(*relative_offset)),
            OpCode::JumpIf { relative_offset } => write!(f, "JumpIf({})", Into::<i32>::into(*relative_offset)),
            OpCode::Add => write!(f, "Add"),
            OpCode::Sub => write!(f, "Sub"),
            OpCode::Mul => write!(f, "Mul"),
            OpCode::Div => write!(f, "Div"),
            OpCode::Pow => write!(f, "Pow"),
            OpCode::Mod => write!(f, "Mod"),
            OpCode::Rand => write!(f, "Rand"),
            OpCode::Len => write!(f, "Len"),
            OpCode::GetRed => write!(f, "GetRed"),
            OpCode::GetGreen => write!(f, "GetGreen"),
            OpCode::GetBlue => write!(f, "GetBlue"),
            OpCode::Set => write!(f, "Set"),
            OpCode::Sleep => write!(f, "Sleep"),
        }
    }
}

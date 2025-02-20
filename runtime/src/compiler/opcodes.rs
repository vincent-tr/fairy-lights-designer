
#[repr(C, align(1))]
struct i24([u8; 3]);

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
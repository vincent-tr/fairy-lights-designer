// Inspired from https://github.com/jmg049/i24

#[derive(Debug, Clone, Copy)]
#[repr(C, align(1))]
#[allow(non_camel_case_types)]
pub struct i24([u8; 3]);

impl i24 {
    pub const ZERO: i24 = i24([0, 0, 0]);
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum ZeroByte {
    Zero = 0,
}

#[cfg(target_endian = "little")]
#[repr(C, align(4))]
struct I24Repr {
    data: [u8; 3],
    // most significant byte at the end
    most_significant_byte: ZeroByte,
}

impl TryFrom<i32> for i24 {
    type Error = anyhow::Error;

    fn try_from(value: i32) -> anyhow::Result<Self> {
        if value > 0x7FFFFF || value < -0x800000 {
            anyhow::bail!("i24 only accepts values between -0x800000 and 0x7FFFFF!")
        }
        
        let repr = unsafe { std::mem::transmute::<i32, I24Repr>(value) };
        Ok(i24(repr.data))
    }
}

impl Into<i32> for i24 {
    fn into(self) -> i32 {
      let repr = I24Repr {
          data: self.0,
          most_significant_byte: ZeroByte::Zero,
      };

      unsafe { std::mem::transmute::<I24Repr, i32>(repr) }
    }
}
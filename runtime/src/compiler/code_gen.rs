use crate::vm::executable::OpCode;


pub struct CodeGen {
  code: Vec<OpCode>,
}

impl CodeGen {
  pub fn new() -> Self {
      CodeGen {
          code: Vec::new(),
      }
  }

  pub fn emit(&mut self, op: OpCode) {
      self.code.push(op);
  }

  pub fn build(self) -> Vec<OpCode> {
      self.code
  }
}

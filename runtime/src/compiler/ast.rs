use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub variables: Vec<String>,
    pub body: Node,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum Node {
  Sequence(Sequence),
  Naked(Naked),
  Compare(Compare),
  Logic(Logic),
  Not(Not),
  LiteralBoolean(LiteralBoolean),
  If(If),
  Repeat(Repeat),
  Until(Until),
  While(While),
  For(For),
  Break(Break),
  Continue(Continue),
  Literal(Literal),
  Arithmetic(Arithmetic),
  Between(Between),
  Rand(Rand),
  GetVariable(GetVariable),
  SetVariable(SetVariable),
  Len(Len),
  Get(Get),
  Set(Set),
  Sleep(Sleep),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequence {
    pub items: Vec<Box<Node>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Naked {
  pub value: Box<Node>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompareOperator {
  Eq,
  Neq,
  Lt,
  Lte,
  Gt,
  Gte,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Compare {
  pub op: CompareOperator,
  pub op1: Box<Node>,
  pub op2: Box<Node>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogicOperator {
  And,
  Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logic {
  pub op: CompareOperator,
  pub op1: Box<Node>,
  pub op2: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Not {
  pub value: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteralBoolean {
  pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfBranch {
  pub condition: Box<Node>,
  pub body: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct If {
  pub branches: Vec<IfBranch>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repeat {
  pub times: Box<Node>,
  pub body: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Until {
  pub condition: Box<Node>,
  pub body: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct While {
  pub condition: Box<Node>,
  pub body: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct For {
  pub variable: String,
  pub from: Box<Node>,
  pub to: Box<Node>,
  pub by: Box<Node>,
  pub body: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Break {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Continue {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Literal {
  pub value: i32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ArithmeticOperator {
  Add,
  Sub,
  Mul,
  Div,
  Pow,
  Mod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Arithmetic {
  pub op: ArithmeticOperator,
  pub op1: Box<Node>,
  pub op2: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Between {
  pub value: Box<Node>,
  pub low: Box<Node>,
  pub high: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rand {
  pub min: Box<Node>,
  pub max: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVariable {
  pub variable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetVariable {
  pub variable: String,
  pub value: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Len {
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GetColor {
  Red,
  Green,
  Blue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Get {
  pub index: Box<Node>,
  pub color: GetColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
  pub index: Box<Node>,
  pub red: Box<Node>,
  pub green: Box<Node>,
  pub blue: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sleep {
  delay: Box<Node>,
}

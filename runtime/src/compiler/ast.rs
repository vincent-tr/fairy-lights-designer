use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub variables: Vec<String>,
    pub body: Node,
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut writer = AstDisplayWriter::new();
        writer.writeln("Program");

        writer.indent();

        for variable in &self.variables {
            writer.write("Variable(");
            writer.write(variable);
            writer.writeln(")");
        }

        writer.writeln("");

        self.body.display(&mut writer);

        write!(f, "{}", writer.finish())
    }
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

impl AstDisplay for Node {
    fn display(&self, writer: &mut AstDisplayWriter) {
        match self {
            Node::Sequence(s) => s.display(writer),
            Node::Naked(n) => n.display(writer),
            Node::Compare(c) => c.display(writer),
            Node::Logic(l) => l.display(writer),
            Node::Not(n) => n.display(writer),
            Node::LiteralBoolean(l) => l.display(writer),
            Node::If(i) => i.display(writer),
            Node::Repeat(r) => r.display(writer),
            Node::Until(u) => u.display(writer),
            Node::While(w) => w.display(writer),
            Node::For(f) => f.display(writer),
            Node::Break(b) => b.display(writer),
            Node::Continue(c) => c.display(writer),
            Node::Literal(l) => l.display(writer),
            Node::Arithmetic(a) => a.display(writer),
            Node::Between(b) => b.display(writer),
            Node::Rand(r) => r.display(writer),
            Node::GetVariable(g) => g.display(writer),
            Node::SetVariable(s) => s.display(writer),
            Node::Len(l) => l.display(writer),
            Node::Get(g) => g.display(writer),
            Node::Set(s) => s.display(writer),
            Node::Sleep(s) => s.display(writer),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequence {
    pub items: Vec<Box<Node>>,
}

impl AstDisplay for Sequence {
    fn display(&self, writer: &mut AstDisplayWriter) {
        for item in &self.items {
            item.display(writer);
            writer.writeln("");
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Naked {
    pub value: Box<Node>,
}

impl AstDisplay for Naked {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Naked(value=");
        self.value.display(writer);
        writer.write(")");
    }
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

impl AstDisplay for Compare {
    fn display(&self, writer: &mut AstDisplayWriter) {
        match self.op {
            CompareOperator::Eq => writer.write("Eq"),
            CompareOperator::Neq => writer.write("Neq"),
            CompareOperator::Lt => writer.write("Lt"),
            CompareOperator::Lte => writer.write("Lte"),
            CompareOperator::Gt => writer.write("Gt"),
            CompareOperator::Gte => writer.write("Gte"),
        }

        writer.write("(op1=");
        self.op1.display(writer);
        writer.write(", op2=");
        self.op2.display(writer);
        writer.write(")");
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LogicOperator {
    And,
    Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logic {
    pub op: LogicOperator,
    pub op1: Box<Node>,
    pub op2: Box<Node>,
}

impl AstDisplay for Logic {
    fn display(&self, writer: &mut AstDisplayWriter) {
        match self.op {
            LogicOperator::And => writer.write("And"),
            LogicOperator::Or => writer.write("Or"),
        }

        writer.write("(op1=");
        self.op1.display(writer);
        writer.write(", op2=");
        self.op2.display(writer);
        writer.write(")");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Not {
    pub value: Box<Node>,
}

impl AstDisplay for Not {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Not(value=");
        self.value.display(writer);
        writer.write(")");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteralBoolean {
    pub value: bool,
}

impl AstDisplay for LiteralBoolean {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write(&format!("LiteralBoolean({})", self.value));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IfBranch {
    pub condition: Option<Box<Node>>,
    pub body: Box<Node>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct If {
    pub branches: Vec<IfBranch>,
}

impl AstDisplay for If {
    fn display(&self, writer: &mut AstDisplayWriter) {
        for (index, branch) in self.branches.iter().enumerate() {
            let first = index == 0;

            if let Some(condition) = &branch.condition {
                if first {
                    writer.write("If");
                } else {
                    writer.write("ElseIf");
                }
                writer.write("(condition=");
                condition.display(writer);
                writer.writeln(")");
            } else {
                writer.writeln("Else");
            }

            writer.indent();
            self.body.display(writer);
            writer.finish_line();
            writer.dedent();
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repeat {
    pub times: Box<Node>,
    pub body: Box<Node>,
}

impl AstDisplay for Repeat {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Until(times=");
        self.times.display(writer);
        writer.writeln(")");

        writer.indent();
        self.body.display(writer);
        writer.finish_line();
        writer.dedent();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Until {
    pub condition: Box<Node>,
    pub body: Box<Node>,
}

impl AstDisplay for Until {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Until(condition=");
        self.condition.display(writer);
        writer.writeln(")");

        writer.indent();
        self.body.display(writer);
        writer.finish_line();
        writer.dedent();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct While {
    pub condition: Box<Node>,
    pub body: Box<Node>,
}

impl AstDisplay for While {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("While(condition=");
        self.condition.display(writer);
        writer.writeln(")");

        writer.indent();
        self.body.display(writer);
        writer.finish_line();
        writer.dedent();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct For {
    pub variable: String,
    pub from: Box<Node>,
    pub to: Box<Node>,
    pub by: Box<Node>,
    pub body: Box<Node>,
}

impl AstDisplay for For {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("For(from=");
        self.from.display(writer);
        writer.write(", to=");
        self.to.display(writer);
        writer.write(", by=");
        self.by.display(writer);
        writer.write(")");

        writer.indent();
        writer.writeln("");
        self.body.display(writer);
        writer.writeln("");
        writer.dedent();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Break {}

impl AstDisplay for Break {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Break");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Continue {}

impl AstDisplay for Continue {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Continue");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Literal {
    pub value: i32,
}

impl AstDisplay for Literal {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write(&format!("Literal({})", self.value));
    }
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

impl AstDisplay for Arithmetic {
    fn display(&self, writer: &mut AstDisplayWriter) {
        match self.op {
            ArithmeticOperator::Add => writer.write("Add"),
            ArithmeticOperator::Sub => writer.write("Sub"),
            ArithmeticOperator::Mul => writer.write("Mul"),
            ArithmeticOperator::Div => writer.write("Div"),
            ArithmeticOperator::Pow => writer.write("Pow"),
            ArithmeticOperator::Mod => writer.write("Mod"),
        }

        writer.write("(op1=");
        self.op1.display(writer);
        writer.write(", op2=");
        self.op2.display(writer);
        writer.write(")");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Between {
    pub value: Box<Node>,
    pub low: Box<Node>,
    pub high: Box<Node>,
}

impl AstDisplay for Between {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Between(value=");
        self.value.display(writer);
        writer.write(", low=");
        self.low.display(writer);
        writer.write(", high=");
        self.high.display(writer);
        writer.write(")");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rand {
    pub min: Box<Node>,
    pub max: Box<Node>,
}

impl AstDisplay for Rand {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Rand(min=");
        self.min.display(writer);
        writer.write(", max=");
        self.max.display(writer);
        writer.write(")");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetVariable {
    pub variable: String,
}

impl AstDisplay for GetVariable {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("GetVariable(variable=");
        writer.write(&self.variable);
        writer.write(")");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetVariable {
    pub variable: String,
    pub value: Box<Node>,
}

impl AstDisplay for SetVariable {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("SetVariable(variable=");
        writer.write(&self.variable);
        writer.write(", value=");
        self.value.display(writer);
        writer.write(")");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Len {}

impl AstDisplay for Len {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Len()");
    }
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

impl AstDisplay for Get {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Get(index=");
        self.index.display(writer);
        writer.write(", color=");

        match self.color {
            GetColor::Red => writer.write("Red"),
            GetColor::Green => writer.write("Green"),
            GetColor::Blue => writer.write("Blue"),
        }

        writer.write(")");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Set {
    pub index: Box<Node>,
    pub red: Box<Node>,
    pub green: Box<Node>,
    pub blue: Box<Node>,
}

impl AstDisplay for Set {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Set(index=");
        self.index.display(writer);
        writer.write(", red=");
        self.red.display(writer);
        writer.write(", green=");
        self.green.display(writer);
        writer.write(", blue=");
        self.blue.display(writer);
        writer.write(")");
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sleep {
    delay: Box<Node>,
}

impl AstDisplay for Sleep {
    fn display(&self, writer: &mut AstDisplayWriter) {
        writer.write("Sleep(delay=");
        self.delay.display(writer);
        writer.write(")");
    }
}

trait AstDisplay {
    fn display(&self, writer: &mut AstDisplayWriter);
}

struct AstDisplayWriter {
    indent: usize,
    lines: Vec<String>,
    current_line: String,
}

impl AstDisplayWriter {
    fn new() -> Self {
        Self { 
            indent: 0,
            lines: Vec::new(),
            current_line: String::new(),
        }
    }

    fn indent(&mut self) {
        self.indent += 1;
    }

    fn dedent(&mut self) {
        self.indent -= 1;
    }

    fn write(&mut self, s: &str) {
        self.current_line.push_str(s);
    }

    fn writeln(&mut self, s: &str) {
        self.write(s);
        self.newline();
    }

    fn finish_line(&mut self) {
        if !self.current_line.is_empty() {
            self.newline();
        }
    }

    fn newline(&mut self) {
        self.lines.push("  ".repeat(self.indent) + &self.current_line);
        self.current_line.clear();
    }

    fn finish(self) -> String {
        self.lines.join("\n")
    }
}
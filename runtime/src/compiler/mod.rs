mod ast;
mod variables;
mod code_gen;

use code_gen::CodeGen;
use log::info;
use variables::Variables;

use crate::vm::executable::{Executable, OpCode};

use anyhow::Result;
use ast::Program;

const STACK_SIZE: u32 = 100;

pub fn compile(input: &str) -> Result<String> {
    let program: Program = serde_json::from_str(input)?;

    info!("Got input program:\n{}", program);

    let variables = Variables::new(program.variables)?;
    let mut compiler = Compiler::new(variables);

    compiler.node(&program.body)?;
    let exec = compiler.generate();

    info!("Compiled into executable:\n{}", exec);

    return Ok(exec.to_text());
}

struct Compiler {
    code: CodeGen,
    variables: Variables,
}

impl Compiler {
    pub fn new(variables: Variables) -> Self {
        Compiler {
            code: CodeGen::new(),
            variables,
        }
    }

    pub fn generate(self) -> Executable {
        Executable::new(STACK_SIZE as u32, self.variables.len() as u32, self.code.build())
    }

    pub fn node(&mut self, node: &ast::Node) -> Result<()> {
        match node {
            ast::Node::Sequence(sequence) => self.sequence(sequence),
            ast::Node::Naked(naked) => self.naked(naked),
            ast::Node::Compare(compare) => self.compare(compare),
            ast::Node::Logic(logic) => self.logic(logic),
            ast::Node::Not(not) => self.not(not),
            ast::Node::LiteralBoolean(literal_boolean) => self.literal_boolean(literal_boolean),
            ast::Node::If(if_) => self.if_(if_),
            ast::Node::Repeat(repeat) => self.repeat(repeat),
            ast::Node::Until(until) => self.until(until),
            ast::Node::While(while_) => self.while_(while_),
            ast::Node::For(for_) => self.for_(for_),
            ast::Node::Break(break_) => self.break_(break_),
            ast::Node::Continue(continue_) => self.continue_(continue_),
            ast::Node::Literal(literal) => self.literal(literal),
            ast::Node::Arithmetic(arithmetic) => self.arithmetic(arithmetic),
            ast::Node::Between(between) => self.between(between),
            ast::Node::Rand(rand) => self.rand(rand),
            ast::Node::GetVariable(get_variable) => self.get_variable(get_variable),
            ast::Node::SetVariable(set_variable) => self.set_variable(set_variable),
            ast::Node::Len(len) => self.len(len),
            ast::Node::Get(get) => self.get(get),
            ast::Node::Set(set) => self.set(set),
            ast::Node::Sleep(sleep) => self.sleep(sleep),
        }
    }

    fn sequence(&mut self, sequence: &ast::Sequence) -> Result<()> {
        for node in sequence.items.iter() {
            self.node(node)?;
        }

        Ok(())
    }

    fn naked(&mut self, naked: &ast::Naked) -> Result<()> {
        self.node(&naked.value)?;
        self.code.emit(OpCode::Pop);

        Ok(())
    }

    fn compare(&mut self, compare: &ast::Compare) -> Result<()> {
        self.node(&compare.op1)?;
        self.node(&compare.op2)?;

        match compare.op {
            ast::CompareOperator::Eq => self.code.emit(OpCode::Equal),
            ast::CompareOperator::Neq => self.code.emit(OpCode::NotEqual),
            ast::CompareOperator::Lt => self.code.emit(OpCode::Less),
            ast::CompareOperator::Lte => self.code.emit(OpCode::LessEqual),
            ast::CompareOperator::Gt => self.code.emit(OpCode::Greater),
            ast::CompareOperator::Gte => self.code.emit(OpCode::GreaterEqual),
        }

        Ok(())
    }

    fn logic(&mut self, logic: &ast::Logic) -> Result<()> {
        self.node(&logic.op1)?;
        self.node(&logic.op2)?;

        match logic.op {
            ast::LogicOperator::And => self.code.emit(OpCode::And),
            ast::LogicOperator::Or => self.code.emit(OpCode::Or),
        }

        Ok(())
    }

    fn not(&mut self, not: &ast::Not) -> Result<()> {
        self.node(&not.value)?;
        self.code.emit(OpCode::Not);

        Ok(())
    }

    fn literal_boolean(&mut self, literal_boolean: &ast::LiteralBoolean) -> Result<()> {
        if literal_boolean.value {
            self.code.emit(OpCode::PushTrue);
        } else {
            self.code.emit(OpCode::PushFalse);
        }

        Ok(())
    }

    fn if_(&mut self, if_: &ast::If) -> Result<()> {
        todo!()
    }

    fn repeat(&mut self, repeat: &ast::Repeat) -> Result<()> {
        todo!()
    }

    fn until(&mut self, until: &ast::Until) -> Result<()> {
        todo!()
    }

    fn while_(&mut self, while_: &ast::While) -> Result<()> {
        todo!()
    }

    fn for_(&mut self, for_: &ast::For) -> Result<()> {
        todo!()
    }

    fn break_(&mut self, break_: &ast::Break) -> Result<()> {
        todo!()
    }

    fn continue_(&mut self, continue_: &ast::Continue) -> Result<()> {
        todo!()
    }

    fn literal(&mut self, literal: &ast::Literal) -> Result<()> {
        self.code.emit(OpCode::PushConstant { value: literal.value.try_into()? });

        Ok(())
    }

    fn arithmetic(&mut self, arithmetic: &ast::Arithmetic) -> Result<()> {
        self.node(&arithmetic.op1)?;
        self.node(&arithmetic.op2)?;

        match arithmetic.op {
            ast::ArithmeticOperator::Add => self.code.emit(OpCode::Add),
            ast::ArithmeticOperator::Sub => self.code.emit(OpCode::Sub),
            ast::ArithmeticOperator::Mul => self.code.emit(OpCode::Mul),
            ast::ArithmeticOperator::Div => self.code.emit(OpCode::Div),
            ast::ArithmeticOperator::Pow => self.code.emit(OpCode::Pow),
            ast::ArithmeticOperator::Mod => self.code.emit(OpCode::Mod),
        }

        Ok(())
    }

    fn between(&mut self, between: &ast::Between) -> Result<()> {
        todo!()
    }

    fn rand(&mut self, rand: &ast::Rand) -> Result<()> {
        todo!()
    }

    fn get_variable(&mut self, get_variable: &ast::GetVariable) -> Result<()> {
        self.code.emit(OpCode::PushVariable { index: self.variables.get_index(&get_variable.variable)? });

        Ok(())
    }

    fn set_variable(&mut self, set_variable: &ast::SetVariable) -> Result<()> {
        self.node(&set_variable.value)?;
        self.code.emit(OpCode::PopVariable { index: self.variables.get_index(&set_variable.variable)? });

        Ok(())
    }

    fn len(&mut self, len: &ast::Len) -> Result<()> {
        self.code.emit(OpCode::Len);

        Ok(())
    }

    fn get(&mut self, get: &ast::Get) -> Result<()> {
        self.node(&get.index)?;

        match get.color {
            ast::GetColor::Red => self.code.emit(OpCode::GetRed),
            ast::GetColor::Green => self.code.emit(OpCode::GetGreen),
            ast::GetColor::Blue => self.code.emit(OpCode::GetBlue),
        }

        Ok(())
    }

    fn set(&mut self, set: &ast::Set) -> Result<()> {
        self.node(&set.index)?;
        self.node(&set.red)?;
        self.node(&set.green)?;
        self.node(&set.blue)?;
        self.code.emit(OpCode::Set);

        Ok(())
    }

    fn sleep(&mut self, sleep: &ast::Sleep) -> Result<()> {
        self.node(&sleep.delay)?;
        self.code.emit(OpCode::Sleep);

        Ok(())
    }

    
}
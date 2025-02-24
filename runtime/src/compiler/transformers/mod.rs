mod compare;
mod between;

use std::mem::swap;

use anyhow::Result;

use super::ast;

pub use compare::Compare;
pub use between::Between;

pub struct VariableAllocator<'a> {
    variables: &'a mut Vec<String>,
}

impl<'a> VariableAllocator<'a> {
    pub fn new(variables: &'a mut Vec<String>) -> Self {
        Self { variables }
    }

    pub fn new_variable(&mut self) -> String {
        let variable = format!("$$var_{}", self.variables.len());
        self.variables.push(variable.clone());
        variable
    }
}

pub trait Transformer {
    fn transform(&mut self, node: ast::Node) -> Result<ast::Node> {
        match node {
            ast::Node::Sequence(sequence) => self.transform_sequence(sequence),
            ast::Node::Naked(naked) => self.transform_naked(naked),
            ast::Node::Compare(compare) => self.transform_compare(compare),
            ast::Node::Logic(logic) => self.transform_logic(logic),
            ast::Node::Not(not) => self.transform_not(not),
            ast::Node::LiteralBoolean(literal_boolean) => {
                self.transform_literal_boolean(literal_boolean)
            }
            ast::Node::If(if_) => self.transform_if(if_),
            ast::Node::Repeat(repeat) => self.transform_repeat(repeat),
            ast::Node::Until(until) => self.transform_until(until),
            ast::Node::While(while_) => self.transform_while(while_),
            ast::Node::For(for_) => self.transform_for(for_),
            ast::Node::Loop(loop_) => self.transform_loop(loop_),
            ast::Node::Break(break_) => self.transform_break(break_),
            ast::Node::Continue(continue_) => self.transform_continue(continue_),
            ast::Node::Literal(literal) => self.transform_literal(literal),
            ast::Node::Arithmetic(arithmetic) => self.transform_arithmetic(arithmetic),
            ast::Node::Between(between) => self.transform_between(between),
            ast::Node::Rand(rand) => self.transform_rand(rand),
            ast::Node::GetVariable(get_variable) => self.transform_get_variable(get_variable),
            ast::Node::SetVariable(set_variable) => self.transform_set_variable(set_variable),
            ast::Node::Len(len) => self.transform_len(len),
            ast::Node::Get(get) => self.transform_get(get),
            ast::Node::Set(set) => self.transform_set(set),
            ast::Node::Sleep(sleep) => self.transform_sleep(sleep),
        }
    }

    fn transform_inplace(&mut self, node: &mut ast::Node) -> Result<()> {
        // Create temp node, just to swap with the original node
        // Will be overwritten with result
        let mut tmp_node = ast::Node::Break(ast::Break {});
        swap(&mut tmp_node, node);

        *node = self.transform(tmp_node)?;

        Ok(())
    }

    fn transform_sequence(&mut self, mut sequence: ast::Sequence) -> Result<ast::Node> {
        for item in sequence.items.iter_mut() {
            self.transform_inplace(item)?;
        }

        Ok(ast::Node::Sequence(sequence))
    }

    fn transform_naked(&mut self, mut naked: ast::Naked) -> Result<ast::Node> {
        self.transform_inplace(&mut naked.value)?;

        Ok(ast::Node::Naked(naked))
    }

    fn transform_compare(&mut self, mut compare: ast::Compare) -> Result<ast::Node> {
        self.transform_inplace(&mut compare.op1)?;
        self.transform_inplace(&mut compare.op2)?;

        Ok(ast::Node::Compare(compare))
    }

    fn transform_logic(&mut self, mut logic: ast::Logic) -> Result<ast::Node> {
        self.transform_inplace(&mut logic.op1)?;
        self.transform_inplace(&mut logic.op2)?;

        Ok(ast::Node::Logic(logic))
    }

    fn transform_not(&mut self, mut not: ast::Not) -> Result<ast::Node> {
        self.transform_inplace(&mut not.value)?;

        Ok(ast::Node::Not(not))
    }

    fn transform_literal_boolean(&mut self, literal_boolean: ast::LiteralBoolean) -> Result<ast::Node> {
        Ok(ast::Node::LiteralBoolean(literal_boolean))
    }

    fn transform_if(&mut self, mut if_: ast::If) -> Result<ast::Node> {
        for branch in if_.branches.iter_mut() {
            if let Some(condition) = &mut branch.condition {
                self.transform_inplace(condition)?;
            }
            self.transform_inplace(&mut branch.body)?;
        }

        Ok(ast::Node::If(if_))
    }

    fn transform_repeat(&mut self, mut repeat: ast::Repeat) -> Result<ast::Node> {
        self.transform_inplace(&mut repeat.times)?;
        self.transform_inplace(&mut repeat.body)?;

        Ok(ast::Node::Repeat(repeat))
    }

    fn transform_until(&mut self, mut until: ast::Until) -> Result<ast::Node> {
        self.transform_inplace(&mut until.condition)?;
        self.transform_inplace(&mut until.body)?;

        Ok(ast::Node::Until(until))
    }

    fn transform_while(&mut self, mut while_: ast::While) -> Result<ast::Node> {
        self.transform_inplace(&mut while_.condition)?;
        self.transform_inplace(&mut while_.body)?;

        Ok(ast::Node::While(while_))
    }

    fn transform_for(&mut self, mut for_: ast::For) -> Result<ast::Node> {
        self.transform_inplace(&mut for_.from)?;
        self.transform_inplace(&mut for_.to)?;
        self.transform_inplace(&mut for_.by)?;
        self.transform_inplace(&mut for_.body)?;

        Ok(ast::Node::For(for_))
    }

    fn transform_loop(&mut self, mut loop_: ast::Loop) -> Result<ast::Node> {
        self.transform_inplace(&mut loop_.body)?;

        Ok(ast::Node::Loop(loop_))
    }

    fn transform_break(&mut self, break_: ast::Break) -> Result<ast::Node> {
        Ok(ast::Node::Break(break_))
    }

    fn transform_continue(&mut self, continue_: ast::Continue) -> Result<ast::Node> {
        Ok(ast::Node::Continue(continue_))
    }

    fn transform_literal(&mut self, literal: ast::Literal) -> Result<ast::Node> {
        Ok(ast::Node::Literal(literal))
    }

    fn transform_arithmetic(&mut self, mut arithmetic: ast::Arithmetic) -> Result<ast::Node> {
        self.transform_inplace(&mut arithmetic.op1)?;
        self.transform_inplace(&mut arithmetic.op2)?;

        Ok(ast::Node::Arithmetic(arithmetic))
    }

    fn transform_between(&mut self, mut between: ast::Between) -> Result<ast::Node> {
        self.transform_inplace(&mut between.value)?;
        self.transform_inplace(&mut between.low)?;
        self.transform_inplace(&mut between.high)?;

        Ok(ast::Node::Between(between))
    }

    fn transform_rand(&mut self, mut rand: ast::Rand) -> Result<ast::Node> {
        self.transform_inplace(&mut rand.min)?;
        self.transform_inplace(&mut rand.max)?;

        Ok(ast::Node::Rand(rand))
    }

    fn transform_get_variable(&mut self, get_variable: ast::GetVariable) -> Result<ast::Node> {
        Ok(ast::Node::GetVariable(get_variable))
    }

    fn transform_set_variable(&mut self, mut set_variable: ast::SetVariable) -> Result<ast::Node> {
        self.transform_inplace(&mut set_variable.value)?;

        Ok(ast::Node::SetVariable(set_variable))
    }

    fn transform_len(&mut self, len: ast::Len) -> Result<ast::Node> {
        Ok(ast::Node::Len(len))
    }

    fn transform_get(&mut self, mut get: ast::Get) -> Result<ast::Node> {
        self.transform_inplace(&mut get.index)?;

        Ok(ast::Node::Get(get))
    }
    fn transform_set(&mut self, mut set: ast::Set) -> Result<ast::Node> {
        self.transform_inplace(&mut set.index)?;
        self.transform_inplace(&mut set.red)?;
        self.transform_inplace(&mut set.green)?;
        self.transform_inplace(&mut set.blue)?;

        Ok(ast::Node::Set(set))
    }

    fn transform_sleep(&mut self, mut sleep: ast::Sleep) -> Result<ast::Node> {
        self.transform_inplace(&mut sleep.delay)?;

        Ok(ast::Node::Sleep(sleep))
    }
}

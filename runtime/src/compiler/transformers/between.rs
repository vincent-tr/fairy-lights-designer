use std::cell::RefCell;

use super::{Transformer, VariableAllocator};
use anyhow::Result;

use super::ast;

pub struct Between<'a> {
    variable_allocator: RefCell<VariableAllocator<'a>>,
}

impl<'a> Between<'a> {
    pub fn new(variable_allocator: RefCell<VariableAllocator<'a>>) -> Self {
        Self { variable_allocator }
    }
}

impl Transformer for Between<'_> {
    fn transform_between(&mut self, mut between: ast::Between) -> Result<ast::Node> {
        self.transform_inplace(&mut between.value)?;
        self.transform_inplace(&mut between.low)?;
        self.transform_inplace(&mut between.high)?;

        let variable = self.variable_allocator.borrow_mut().new_variable();

        // tranform into low <= value && value < between.high
        Ok(ast::Node::Sequence(ast::Sequence {
            items: vec![
                Box::new(ast::Node::SetVariable(ast::SetVariable {
                    variable: variable.clone(),
                    value: between.value,
                })),
                Box::new(ast::Node::Logic(ast::Logic {
                    op: ast::LogicOperator::And,
                    op1: Box::new(ast::Node::Compare(ast::Compare {
                        op: ast::CompareOperator::Lte,
                        op1: between.low,
                        op2: Box::new(ast::Node::GetVariable(ast::GetVariable {
                            variable: variable.clone(),
                        })),
                    })),
                    op2: Box::new(ast::Node::Compare(ast::Compare {
                        op: ast::CompareOperator::Lt,
                        op1: Box::new(ast::Node::GetVariable(ast::GetVariable {
                            variable: variable.clone(),
                        })),
                        op2: between.high,
                    })),
                })),
            ],
        }))
    }
}

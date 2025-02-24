use std::mem::swap;

use super::Transformer;
use anyhow::Result;

use super::ast;

pub struct Compare {
}

impl Compare {
    pub fn new() -> Self {
        Self {}
    }
}

impl Transformer for Compare {
    fn transform_compare(&mut self, mut compare: ast::Compare) -> Result<ast::Node> {
        self.transform_inplace(&mut compare.op1)?;
        self.transform_inplace(&mut compare.op2)?;

        match compare.op {
            ast::CompareOperator::Gt => {
                compare.op = ast::CompareOperator::Lt;
                swap(&mut compare.op1, &mut compare.op2);
            }
            ast::CompareOperator::Gte => {
                compare.op = ast::CompareOperator::Lte;
                swap(&mut compare.op1, &mut compare.op2);
            }
            _  => {
                // Do nothing
            }
        }

        Ok(ast::Node::Compare(compare))
    }
}
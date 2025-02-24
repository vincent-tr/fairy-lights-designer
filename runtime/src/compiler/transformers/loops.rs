use std::cell::RefCell;

use super::{Transformer, VariableAllocator};
use anyhow::Result;

use super::ast;

pub struct Loops<'a> {
    variable_allocator: &'a RefCell<VariableAllocator<'a>>,
}

impl<'a> Loops<'a> {
    pub fn new(variable_allocator: &'a RefCell<VariableAllocator<'a>>) -> Self {
        Self { variable_allocator }
    }
}

impl Transformer for Loops<'_> {
    fn transform_while(&mut self, mut while_: ast::While) -> Result<ast::Node> {
        self.transform_inplace(&mut while_.condition)?;
        self.transform_inplace(&mut while_.body)?;

        // transform
        //
        // while condition {
        //     body
        // }
        //
        // into
        //
        // loop {
        //   if !condition {
        //     break;
        //   }
        //   body
        // }

        Ok(ast::Node::Loop(ast::Loop {
            body: Box::new(ast::Node::Sequence(ast::Sequence {
                items: vec![
                    Box::new(ast::Node::If(ast::If {
                        branches: vec![ast::IfBranch {
                            condition: Some(Box::new(ast::Node::Not(ast::Not {
                                value: while_.condition,
                            }))),
                            body: Box::new(ast::Node::Break(ast::Break {})),
                        }],
                    })),
                    while_.body,
                ],
            })),
        }))
    }

    fn transform_until(&mut self, mut until: ast::Until) -> Result<ast::Node> {
        self.transform_inplace(&mut until.condition)?;
        self.transform_inplace(&mut until.body)?;

        // transform
        //
        // until condition {
        //     body
        // }
        //
        // into
        //
        // loop {
        //   if condition {
        //     break;
        //   }
        //   body
        // }

        Ok(ast::Node::Loop(ast::Loop {
            body: Box::new(ast::Node::Sequence(ast::Sequence {
                items: vec![
                    Box::new(ast::Node::If(ast::If {
                        branches: vec![ast::IfBranch {
                            condition: Some(until.condition),
                            body: Box::new(ast::Node::Break(ast::Break {})),
                        }],
                    })),
                    until.body,
                ],
            })),
        }))
    }

    fn transform_for(&mut self, mut for_: ast::For) -> Result<ast::Node> {
        self.transform_inplace(&mut for_.from)?;
        self.transform_inplace(&mut for_.to)?;
        self.transform_inplace(&mut for_.by)?;
        self.transform_inplace(&mut for_.body)?;

        let variable = for_.variable;
        let by_var = self.variable_allocator.borrow_mut().new_variable();
        let to_var = self.variable_allocator.borrow_mut().new_variable();

        // transform
        //
        // for i = from to to by by {
        //   body
        // }
        //
        // into
        // by_var = by;
        // to_var = to;
        // i = from - by_var;
        // loop {
        //   i = i + by_var;
        //   if i >= to_var {
        //     break;
        //   }
        //   body
        // }

        Ok(ast::Node::Sequence(ast::Sequence {
            items: vec![
                Box::new(ast::Node::SetVariable(ast::SetVariable {
                    variable: by_var.clone(),
                    value: for_.by,
                })),
                Box::new(ast::Node::SetVariable(ast::SetVariable {
                    variable: to_var.clone(),
                    value: for_.to,
                })),
                Box::new(ast::Node::SetVariable(ast::SetVariable {
                    variable: variable.clone(),
                    value: Box::new(ast::Node::Arithmetic(ast::Arithmetic {
                        op: ast::ArithmeticOperator::Sub,
                        op1: for_.from,
                        op2: Box::new(ast::Node::GetVariable(ast::GetVariable {
                            variable: by_var.clone(),
                        })),
                    })),
                })),
                Box::new(ast::Node::Loop(ast::Loop {
                    body: Box::new(ast::Node::Sequence(ast::Sequence {
                        items: vec![
                            Box::new(ast::Node::SetVariable(ast::SetVariable {
                                variable: variable.clone(),
                                value: Box::new(ast::Node::Arithmetic(ast::Arithmetic {
                                    op: ast::ArithmeticOperator::Add,
                                    op1: Box::new(ast::Node::GetVariable(ast::GetVariable {
                                        variable: variable.clone(),
                                    })),
                                    op2: Box::new(ast::Node::GetVariable(ast::GetVariable {
                                        variable: by_var.clone(),
                                    })),
                                })),
                            })),
                            Box::new(ast::Node::If(ast::If {
                                branches: vec![ast::IfBranch {
                                    condition: Some(Box::new(ast::Node::Compare(ast::Compare {
                                        op: ast::CompareOperator::Gte,
                                        op1: Box::new(ast::Node::GetVariable(ast::GetVariable {
                                            variable: variable.clone(),
                                        })),
                                        op2: Box::new(ast::Node::GetVariable(ast::GetVariable {
                                            variable: to_var.clone(),
                                        })),
                                    }))),
                                    body: Box::new(ast::Node::Break(ast::Break {})),
                                }],
                            })),
                            for_.body,
                        ],
                    })),
                })),
            ],
        }))
    }
}

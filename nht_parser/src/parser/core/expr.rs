use crate::parser::core::nodes::Nodes;
use crate::parser::core::ops::{BinaryOp, UnaryOp};
use crate::parser::core::types::Types;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Nodes),
    Binary {
        left: Box<Expr>,
        operator: Box<BinaryOp>,
        right: Box<Expr>,
    },
    Unary {
        operator: Box<UnaryOp>,
        operand: Box<Expr>,
    },
    Assignment {
        identifier: String,
        value: Box<Expr>,
    },
    Declaration {
        identifier: String,
        var_type: Option<Types>,
        value: Box<Expr>,
    },
    Function {
        identifier: String,
        parameters: Vec<Expr>,
        body: Box<Expr>,
        return_type: Option<Types>,
    },
    Return {
        value: Box<Expr>,
    },
    Block(Vec<Expr>),
    IfElse {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
}

impl Expr {
    pub fn new_int_literal(value: i64) -> Self {
        Expr::Literal(Nodes::Integer(value))
    }

    pub fn new_string_literal(value: String) -> Self {
        Expr::Literal(Nodes::String(value))
    }

    pub fn new_boolean_literal(value: bool) -> Self {
        Expr::Literal(Nodes::Boolean(value))
    }

    pub fn new_identifier(name: &str) -> Self {
        Expr::Literal(Nodes::Identifier(name.to_string()))
    }
}

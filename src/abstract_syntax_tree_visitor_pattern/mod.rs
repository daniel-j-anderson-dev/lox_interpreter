//! A re-implementation of [super::abstract_syntax_tree] that uses the Visitor design pattern

pub mod printer;

use crate::token::Token;
use std::ops::Deref;

pub trait ExpressionVisitor<R> {
    fn visit_binary_expression(&self, expression: &Binary) -> R;
    fn visit_unary_expression(&self, expression: &Unary) -> R;
    fn visit_grouping_expression(&self, expression: &Grouping) -> R;
    fn visit_literal_expression(&self, expression: &Literal) -> R;
}

pub enum Expression<'a> {
    Binary(Binary<'a>),
    Unary(Unary<'a>),
    Grouping(Grouping<'a>),
    Literal(Literal<'a>),
}
impl Expression<'_> {
    pub fn accept_visitor<R>(&self, visitor: &impl ExpressionVisitor<R>) -> R {
        match self {
            Expression::Binary(binary) => visitor.visit_binary_expression(binary),
            Expression::Unary(unary) => visitor.visit_unary_expression(unary),
            Expression::Grouping(grouping) => visitor.visit_grouping_expression(grouping),
            Expression::Literal(literal) => visitor.visit_literal_expression(literal),
        }
    }
}

pub struct Binary<'a> {
    left_operand: Box<Expression<'a>>,
    operator: Token<'a>,
    right_operand: Box<Expression<'a>>,
}
impl Binary<'_> {
    pub fn left_operand(&self) -> &Expression<'_> {
        self.left_operand.deref()
    }
    pub fn operator(&self) -> &Token<'_> {
        &self.operator
    }
    pub fn right_operand(&self) -> &Expression<'_> {
        self.right_operand.deref()
    }
}

pub struct Unary<'a> {
    operator: Token<'a>,
    right_operand: Box<Expression<'a>>,
}
impl Unary<'_> {
    pub fn operator(&self) -> &Token<'_> {
        &self.operator
    }
    pub fn right_operand(&self) -> &Expression<'_> {
        self.right_operand.deref()
    }
}

pub struct Grouping<'a>(Box<Expression<'a>>);
impl Grouping<'_> {
    pub fn inner_expression(&self) -> &Expression<'_> {
        self.0.deref()
    }
}

pub struct Literal<'a>(Token<'a>);
impl Literal<'_> {
    pub fn token(&self) -> &Token<'_> {
        &self.0
    }
}

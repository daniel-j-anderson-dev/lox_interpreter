use super::*;

pub struct AbstractSyntaxTreePrinter;
impl AbstractSyntaxTreePrinter {
    pub fn print(&self, expression: &Expression) -> String {
        expression.accept_visitor(self)
    }
}
impl ExpressionVisitor<String> for AbstractSyntaxTreePrinter {
    fn visit_binary_expression(&self, expression: &Binary) -> String {
        parenthesizes(
            expression.operator().lexeme(),
            &[expression.left_operand(), expression.right_operand()],
        )
    }

    fn visit_unary_expression(&self, expression: &Unary) -> String {
        parenthesizes(
            expression.operator().lexeme(),
            &[expression.right_operand()],
        )
    }

    fn visit_grouping_expression(&self, expression: &Grouping) -> String {
        parenthesizes("group", &[expression.inner_expression()])
    }

    fn visit_literal_expression(&self, expression: &Literal) -> String {
        expression.token().lexeme().to_owned()
    }
}

fn parenthesizes(name: &str, expressions: &[&Expression]) -> String {
    let mut output = String::new();

    output.push('(');
    output.push_str(name);

    for expression in expressions {
        output.push(' ');
        output.push_str(&AbstractSyntaxTreePrinter.print(expression));
    }

    output.push(')');

    output
}

#[test]
fn ast_print() {
    use crate::token::TokenKind;

    const EXPECTED: &'static str = "(* (- 123) (group 45.67))";

    let expression = Expression::Binary(Binary {
        left_operand: Box::new(Expression::Unary(Unary {
            operator: Token::new(TokenKind::Minus, "-", 0),
            right_operand: Box::new(Expression::Literal(Literal(Token::new(
                TokenKind::Number,
                "123",
                0,
            )))),
        })),
        operator: Token::new(TokenKind::Star, "*", 0),
        right_operand: Box::new(Expression::Grouping(Grouping(Box::new(
            Expression::Literal(Literal(Token::new(TokenKind::Number, "45.67", 0))),
        )))),
    });

    let output = AbstractSyntaxTreePrinter.print(&expression);

    assert_eq!(output, EXPECTED);
}

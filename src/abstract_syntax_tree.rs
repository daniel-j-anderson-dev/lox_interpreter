use super::token::Token;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression<'a> {
    Binary {
        left_operand: Box<Expression<'a>>,
        operator: Token<'a>,
        right_operand: Box<Expression<'a>>,
    },
    Unary {
        operator: Token<'a>,
        right_operand: Box<Expression<'a>>,
    },
    Grouping(Box<Expression<'a>>),
    Literal(Token<'a>),
}
impl Display for Expression<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn parenthesizes(name: &str, expressions: &[&Expression]) -> String {
            let mut output = String::new();

            output.push('(');
            output.push_str(name);

            for expression in expressions {
                output.push(' ');
                output.push_str(&expression.to_string());
            }
            output.push(')');

            output
        }

        let output = match self {
            Expression::Binary {
                left_operand,
                operator,
                right_operand,
            } => parenthesizes(operator.lexeme(), &[left_operand, right_operand]),
            Expression::Unary {
                operator,
                right_operand,
            } => parenthesizes(operator.lexeme(), &[right_operand]),
            Expression::Grouping(expression) => parenthesizes("group", &[expression]),
            Expression::Literal(literal) => literal.lexeme().to_owned(),
        };

        write!(f, "{}", output)
    }
}

#[test]
fn ast_print() {
    use crate::token::TokenKind;

    const EXPECTED: &'static str = "(* (- 123) (group 45.67))";

    let expression = Expression::Binary {
        left_operand: Box::new(Expression::Unary {
            operator: Token::new(TokenKind::Minus, "-", 0, 0),
            right_operand: Box::new(Expression::Literal(Token::new(
                TokenKind::NumberLiteral,
                "123",
                0,
                0,
            ))),
        }),
        operator: Token::new(TokenKind::Star, "*", 0, 0),
        right_operand: Box::new(Expression::Grouping(Box::new(Expression::Literal(
            Token::new(TokenKind::NumberLiteral, "45.67", 0, 0),
        )))),
    };

    let output = expression.to_string();

    assert_eq!(output, EXPECTED);
}

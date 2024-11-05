use crate::{
    abstract_syntax_tree::Expression,
    lexer::{Lexer, LexerError},
    token::{Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current_token_index: usize,
}
impl<'a> Parser<'a> {
    pub const fn new(tokens: Vec<Token<'a>>) -> Self {
        Self {
            tokens,
            current_token_index: 0,
        }
    }
    fn consume_current_token_of_kind(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.is_current_token(*kind) {
                self.consume_current_token();
                return true;
            }
        }

        false
    }
    fn is_current_token(&self, kind: TokenKind) -> bool {
        !self.is_at_end() && self.peek_current_token().kind() == kind
    }
    fn consume_current_token(&mut self) {
        if !self.is_at_end() {
            self.current_token_index += 1;
        }
    }
    fn is_at_end(&self) -> bool {
        self.peek_current_token().is_end_of_file()
    }
    fn peek_current_token(&self) -> Token<'a> {
        self.tokens[self.current_token_index]
    }
    fn peek_previous_token(&self) -> Token<'a> {
        self.tokens[self.current_token_index - 1]
    }
}
impl<'a> TryFrom<Lexer<'a>> for Parser<'a> {
    type Error = ParseError;
    fn try_from(value: Lexer<'a>) -> Result<Self, Self::Error> {
        let tokens = value.collect::<Result<_, _>>()?;
        Ok(Self::new(tokens))
    }
}
impl<'a> Parser<'a> {
    fn expression_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError> {
        self.equality_rule()
    }
    fn equality_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError> {
        let mut expression = self.comparison_rule()?;

        while self.consume_current_token_of_kind(TokenKind::EQUALITY_OPERATORS) {
            expression = Box::new(Expression::Binary {
                left_operand: expression,
                operator: self.peek_previous_token(),
                right_operand: self.comparison_rule()?,
            });
        }

        Ok(expression)
    }
    fn comparison_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError> {
        let mut expression = self.term_rule()?;

        while self.consume_current_token_of_kind(TokenKind::COMPARISON_OPERATORS) {
            expression = Box::new(Expression::Binary {
                left_operand: expression,
                operator: self.peek_previous_token(),
                right_operand: self.term_rule()?,
            });
        }

        Ok(expression)
    }
    fn term_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError> {
        let mut expression = self.factor_rule()?;

        while self.consume_current_token_of_kind(TokenKind::TERM_OPERATORS) {
            expression = Box::new(Expression::Binary {
                left_operand: expression,
                operator: self.peek_previous_token(),
                right_operand: self.factor_rule()?,
            });
        }

        Ok(expression)
    }
    fn factor_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError> {
        let mut expression = self.unary_rule()?;

        while self.consume_current_token_of_kind(TokenKind::FACTOR_OPERATORS) {
            expression = Box::new(Expression::Binary {
                left_operand: expression,
                operator: self.peek_previous_token(),
                right_operand: self.unary_rule()?,
            });
        }

        Ok(expression)
    }
    fn unary_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError> {
        if self.consume_current_token_of_kind(TokenKind::UNARY_OPERATORS) {
            Ok(Box::new(Expression::Unary {
                operator: self.peek_previous_token(),
                right_operand: self.unary_rule()?,
            }))
        } else {
            self.primary_rule()
        }
    }
    fn primary_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError> {
        if self.consume_current_token_of_kind(&[TokenKind::False]) {
            return Ok(Box::new(Expression::Literal(self.peek_previous_token())));
        }
        if self.consume_current_token_of_kind(&[TokenKind::True]) {
            return Ok(Box::new(Expression::Literal(self.peek_previous_token())));
        }
        if self.consume_current_token_of_kind(&[TokenKind::Nil]) {
            return Ok(Box::new(Expression::Literal(self.peek_previous_token())));
        }
        if self.consume_current_token_of_kind(&[TokenKind::Number, TokenKind::String]) {
            return Ok(Box::new(Expression::Literal(self.peek_previous_token())));
        }
        if self.consume_current_token_of_kind(&[TokenKind::LeftParentheses]) {
            let expression = self.expression_rule()?;
            if !self.consume_current_token_of_kind(&[TokenKind::RightParentheses]) {
                return Err(ParseError {
                    kind: ParseErrorKind::MissingRightParenthesis,
                    line_number: self.peek_current_token().line_number(),
                });
            }
            return Ok(Box::new(Expression::Grouping(expression)));
        }

        Err(ParseError {
            kind: ParseErrorKind::ExpectedExpression,
            line_number: self.peek_current_token().line_number(),
        })
    }
}

#[derive(Debug)]
pub struct ParseError {
    kind: ParseErrorKind,
    line_number: usize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind {
    MissingRightParenthesis,
    ExpectedExpression,
    UnaryExpressionMissingOperand,
    LexerError(LexerError),
}
impl From<LexerError> for ParseError {
    fn from(value: LexerError) -> Self {
        Self {
            line_number: value.line_number(),
            kind: ParseErrorKind::LexerError(value),
        }
    }
}

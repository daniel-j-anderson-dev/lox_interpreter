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
    type Error = ParseError<'a>;
    fn try_from(value: Lexer<'a>) -> Result<Self, Self::Error> {
        let tokens = value.collect::<Result<_, _>>()?;
        Ok(Self::new(tokens))
    }
}
impl<'a> Parser<'a> {
    fn expression_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError<'a>> {
        self.equality_rule()
    }
    fn equality_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError<'a>> {
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
    fn comparison_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError<'a>> {
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
    fn term_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError<'a>> {
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
    fn factor_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError<'a>> {
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
    fn unary_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError<'a>> {
        if self.consume_current_token_of_kind(TokenKind::UNARY_OPERATORS) {
            Ok(Box::new(Expression::Unary {
                operator: self.peek_previous_token(),
                right_operand: self.unary_rule()?,
            }))
        } else {
            self.primary_rule()
        }
    }
    fn primary_rule(&mut self) -> Result<Box<Expression<'a>>, ParseError<'a>> {
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
                    token: self.peek_current_token(),
                });
            }
            return Ok(Box::new(Expression::Grouping(expression)));
        }

        Err(ParseError {
            kind: ParseErrorKind::ExpectedExpression,
            token: self.peek_current_token(),
        })
    }
}

#[derive(Debug)]
pub struct ParseError<'a> {
    kind: ParseErrorKind<'a>,
    token: Token<'a>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseErrorKind<'a> {
    MissingRightParenthesis,
    ExpectedExpression,
    UnaryExpressionMissingOperand,
    LexerError(LexerError<'a>),
}
impl<'a> From<LexerError<'a>> for ParseError<'a> {
    fn from(value: LexerError<'a>) -> Self {
        Self {
            token: value.token(),
            kind: ParseErrorKind::LexerError(value),
        }
    }
}
impl std::fmt::Display for ParseErrorKind<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseErrorKind::MissingRightParenthesis => write!(f, "Missing closing parenthesis"),
            ParseErrorKind::ExpectedExpression => write!(f, "No rule matched. Expected expression"),
            ParseErrorKind::UnaryExpressionMissingOperand => {
                write!(f, "Unary operator must have an expression after")
            }
            ParseErrorKind::LexerError(lexer_error) => write!(f, "{}", lexer_error),
        }
    }
}

impl std::fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error parsing {:?} token: \"{}\" on line {}: {}",
            self.token.kind(),
            self.token.lexeme(),
            self.token.line_number(),
            self.kind
        )
    }
}

#[test]
fn test_parser() {
    const SOURCE: &str = include_str!("../simple_example.lox");
    let lexer = Lexer::new(SOURCE);
    let mut parser = Parser::try_from(lexer).unwrap();

    loop {
        match parser.equality_rule() {
            Ok(expression) => println!("{}", expression),
            Err(parse_error) if !parse_error.token.is_end_of_file() => eprintln!("{}", parse_error),
            _ => break,
        }
    }
}

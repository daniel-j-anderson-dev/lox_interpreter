use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    kind: TokenKind,
    lexeme: &'a str,
    line: usize,
    column: usize,
}
impl<'a> Token<'a> {
    pub const fn new(kind: TokenKind, lexeme: &'a str, line: usize, column: usize) -> Self {
        Self {
            kind,
            lexeme,
            line,
            column,
        }
    }
    pub const fn end_of_file(line: usize, column: usize) -> Token<'static> {
        Token {
            kind: TokenKind::EndOfFile,
            lexeme: "",
            line,
            column,
        }
    }
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }
    pub const fn lexeme(&self) -> &'a str {
        self.lexeme
    }
    pub const fn is_end_of_file(&self) -> bool {
        self.kind.is_end_of_file()
    }
    pub const fn line(&self) -> usize {
        self.line
    }
    pub const fn column(&self) -> usize {
        self.column
    }
}
impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{:?} Ln {:>3}, Col {:>3}  {:?}",
            " ".repeat(16 - self.kind.as_str().len()),
            self.kind,
            self.line,
            self.column,
            self.lexeme
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Unrecognized,
    EndOfFile,
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    Identifier,
    StringLiteral,
    NumberLiteral,
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}
impl TokenKind {
    pub fn parse_keyword(identifier_lexeme: &str) -> Self {
        match identifier_lexeme {
            "and" => TokenKind::And,
            "class" => TokenKind::Class,
            "else" => TokenKind::Else,
            "false" => TokenKind::False,
            "for" => TokenKind::For,
            "fun" => TokenKind::Fun,
            "if" => TokenKind::If,
            "nil" => TokenKind::Nil,
            "or" => TokenKind::Or,
            "print" => TokenKind::Print,
            "return" => TokenKind::Return,
            "super" => TokenKind::Super,
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "var" => TokenKind::Var,
            "while" => TokenKind::While,
            _ => TokenKind::Identifier,
        }
    }
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Unrecognized => "Unrecognized",
            Self::EndOfFile => "EndOfFile",
            Self::LeftParentheses => "LeftParentheses",
            Self::RightParentheses => "RightParentheses",
            Self::LeftBrace => "LeftBrace",
            Self::RightBrace => "RightBrace",
            Self::Comma => "Comma",
            Self::Dot => "Dot",
            Self::Minus => "Minus",
            Self::Plus => "Plus",
            Self::Semicolon => "Semicolon",
            Self::Slash => "Slash",
            Self::Star => "Star",
            Self::Bang => "Bang",
            Self::BangEqual => "BangEqual",
            Self::Equal => "Equal",
            Self::EqualEqual => "EqualEqual",
            Self::Greater => "Greater",
            Self::GreaterEqual => "GreaterEqual",
            Self::Less => "Less",
            Self::LessEqual => "LessEqual",
            Self::Identifier => "Identifier",
            Self::StringLiteral => "StringLiteral",
            Self::NumberLiteral => "NumberLiteral",
            Self::And => "And",
            Self::Class => "Class",
            Self::Else => "Else",
            Self::False => "False",
            Self::Fun => "Fun",
            Self::For => "For",
            Self::If => "If",
            Self::Nil => "Nil",
            Self::Or => "Or",
            Self::Print => "Print",
            Self::Return => "Return",
            Self::Super => "Super",
            Self::This => "This",
            Self::True => "True",
            Self::Var => "Var",
            Self::While => "While",
        }
    }
    pub const fn is_end_of_file(&self) -> bool {
        matches!(self, TokenKind::EndOfFile)
    }
    pub fn is_any(&self, kinds: &[TokenKind]) -> bool {
        kinds.contains(self)
    }
    pub const EQUALITY_OPERATORS: &[Self] = &[TokenKind::BangEqual, TokenKind::EqualEqual];
    pub const COMPARISON_OPERATORS: &[Self] = &[
        TokenKind::Less,
        TokenKind::LessEqual,
        TokenKind::Greater,
        TokenKind::GreaterEqual,
    ];
    pub const TERM_OPERATORS: &[Self] = &[TokenKind::Plus, TokenKind::Minus];
    pub const FACTOR_OPERATORS: &[Self] = &[TokenKind::Star, TokenKind::Slash];
    pub const UNARY_OPERATORS: &[Self] = &[TokenKind::Bang, TokenKind::Minus];
}

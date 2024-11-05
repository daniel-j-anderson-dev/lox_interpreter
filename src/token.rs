use std::fmt::{Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Token<'a> {
    kind: TokenKind,
    lexeme: &'a str,
    line_number: usize,
}
impl<'a> Token<'a> {
    pub const fn new(kind: TokenKind, lexeme: &'a str, line_number: usize) -> Self {
        Self {
            kind,
            lexeme,
            line_number,
        }
    }
    pub const fn end_of_file(line_number: usize) -> Token<'static> {
        Token {
            kind: TokenKind::EndOfFile,
            lexeme: "",
            line_number,
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
    pub const fn line_number(&self) -> usize {
        self.line_number
    }
}
impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {:?} {}", self.line_number, self.kind, self.lexeme)
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
    String,
    Number,
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
    pub const fn is_end_of_file(&self) -> bool {
        match self {
            TokenKind::EndOfFile => true,
            _ => false,
        }
    }
    pub fn is_any(&self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if *self == *kind {
                return true;
            }
        }

        false
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

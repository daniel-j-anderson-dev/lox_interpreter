use std::fmt::Display;

#[derive(Debug)]
pub struct Token<'a> {
    kind: TokenKind,
    lexeme: &'a str,
}
impl<'a> Token<'a> {
    pub const fn new(kind: TokenKind, lexeme: &'a str) -> Self {
        Self { kind, lexeme }
    }
    pub const fn end_of_file() -> Token<'static> {
        Token { kind: TokenKind::EndOfFile, lexeme: "" }
    }
    pub const fn kind(&self) -> TokenKind {
        self.kind
    }
    pub fn is_end_of_file(&self) -> bool {
        self.kind == TokenKind::EndOfFile
    }
}
impl Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {}", self.kind, self.lexeme)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
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
}

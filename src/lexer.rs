use unicode_segmentation::UnicodeSegmentation;

use crate::{
    lexer::error::{LexerError, LexerErrorKind},
    token::{
        Token,
        TokenKind::{self, *},
    },
};

/// Lazily split lox source code into [Token]s.
/// When used as an [Iterator]: [None] represents a [EndOfFile]
pub struct Lexer<'a> {
    source: &'a str,
    lexeme_start: usize,
    /// index of the byte currently being processed. one after the last byte in the current lexeme
    lexeme_end: usize,
    line: usize,
    column: usize,
    end_of_file_emitted: bool,
}
// the whole point
impl<'a> Lexer<'a> {
    pub fn next_token(&mut self) -> Result<Token<'a>, LexerError<'a>> {
        // [1]
        if self.out_of_source_bytes() {
            self.end_of_file_emitted = true;
            return Ok(self.token(EndOfFile));
        }

        self.start_lexeme();

        // SAFETY: the current byte is available. See [1]
        let previous_byte = unsafe { self.current_byte_unchecked() };
        self.extend_lexeme();

        match previous_byte {
            b'(' => Ok(self.token(LeftParentheses)),
            b')' => Ok(self.token(RightParentheses)),
            b'{' => Ok(self.token(LeftBrace)),
            b'}' => Ok(self.token(RightBrace)),
            b',' => Ok(self.token(Comma)),
            b'.' => Ok(self.token(Dot)),
            b'-' => Ok(self.token(Minus)),
            b'+' => Ok(self.token(Plus)),
            b';' => Ok(self.token(Semicolon)),
            b'*' => Ok(self.token(Star)),
            b'!' => Ok(
                if let LexemeStatus::Extended = self.extend_lexeme_if(byte_is(b'=')) {
                    self.token(BangEqual)
                } else {
                    self.token(Bang)
                },
            ),
            b'=' => Ok(
                if let LexemeStatus::Extended = self.extend_lexeme_if(byte_is(b'=')) {
                    self.token(EqualEqual)
                } else {
                    self.token(Equal)
                },
            ),
            b'<' => Ok(
                if let LexemeStatus::Extended = self.extend_lexeme_if(byte_is(b'=')) {
                    self.token(LessEqual)
                } else {
                    self.token(Less)
                },
            ),
            b'>' => Ok(
                if let LexemeStatus::Extended = self.extend_lexeme_if(byte_is(b'=')) {
                    self.token(GreaterEqual)
                } else {
                    self.token(Greater)
                },
            ),
            b'/' if self.current_byte_is(b'/') => {
                self.extend_lexeme_while(byte_is_not(b'\n'));
                self.next_token()
            }
            b'/' => Ok(self.token(Slash)),
            b'"' => {
                self.extend_lexeme_while(byte_is_not(b'"'));
                if self.out_of_source_bytes() {
                    Err(self.error(LexerErrorKind::UnterminatedStringLiteral))
                } else {
                    self.extend_lexeme();
                    Ok(Token::new(
                        StringLiteral,
                        trim_first_and_last(self.lexeme()),
                        self.line,
                        self.column,
                    ))
                }
            }
            b'\n' => {
                self.line += 1;
                self.column = 0;
                self.next_token()
            }
            _ if previous_byte.is_ascii_whitespace() => {
                self.extend_lexeme_while(byte_is_non_newline_whitespace);
                self.next_token()
            }
            _ if previous_byte.is_ascii_digit() => {
                self.extend_lexeme_while(byte_is_digit);
                if let LexemeStatus::Extended = self.extend_lexeme_if(byte_is(b'.')) {
                    self.extend_lexeme_while(byte_is_digit);
                }
                Ok(self.token(NumberLiteral))
            }
            _ if byte_is_identifier(previous_byte) => {
                self.extend_lexeme_while(byte_is_identifier);
                let kind = TokenKind::parse_keyword(self.lexeme());
                Ok(self.token(kind))
            }
            _ => {
                self.extend_lexeme_while(byte_is_unrecognized);
                Err(self.error(LexerErrorKind::Unrecognized))
            }
        }
    }
}

// constructors
impl<'a> Lexer<'a> {
    pub const fn new(source: &'a str) -> Self {
        Self {
            source,
            lexeme_start: 0,
            lexeme_end: 0,
            line: 1,
            column: 0,
            end_of_file_emitted: false,
        }
    }
}

// accessors
impl<'a> Lexer<'a> {
    fn out_of_source_bytes(&self) -> bool {
        self.lexeme_end >= self.source.len()
    }

    /// # Safety
    /// Caller must guarantee that `self.lexeme_end` >= `self.source.len()` (use [Lexer::current_byte_available] to check)
    unsafe fn current_byte_unchecked(&self) -> u8 {
        debug_assert!(self.lexeme_end < self.source.len());
        *unsafe { self.source.as_bytes().get_unchecked(self.lexeme_end) }
    }

    fn current_byte(&self) -> Option<u8> {
        self.source.as_bytes().get(self.lexeme_end).copied()
    }

    /// Returns the current lexeme of `source` defined by the range `lexeme_start..lexeme_end`
    fn lexeme(&self) -> &'a str {
        self
            .source
            .get(self.lexeme_start..self.lexeme_end)
            .unwrap_or("")
    }

    fn current_byte_is(&self, target: u8) -> bool {
        self.current_byte().is_some_and(|b| b == target)
    }
}

// mutators
impl<'a> Lexer<'a> {
    /// consumes the current lexeme so that a new a new token can be lexed.
    fn start_lexeme(&mut self) {
        self.column += count_grapheme_clusters(self.lexeme());
        self.lexeme_start = self.lexeme_end;
    }

    /// Increments `self.lexeme_end` making the current lexeme one byte larger
    fn extend_lexeme(&mut self) {
        self.lexeme_end = self.lexeme_end.saturating_add(1);
    }

    /// If the current byte makes `predicate` return `true` then the lexeme will extended. See [LexemeStatus]
    fn extend_lexeme_if(&mut self, predicate: impl FnOnce(u8) -> bool) -> LexemeStatus {
        if let Some(current_byte) = self.current_byte()
            && predicate(current_byte)
        {
            self.extend_lexeme();
            LexemeStatus::Extended
        } else {
            LexemeStatus::NotExtended
        }
    }

    /// keep extending the lexeme until `predicate` returns false
    fn extend_lexeme_while(&mut self, mut predicate: impl FnMut(u8) -> bool) {
        while let LexemeStatus::Extended = self.extend_lexeme_if(&mut predicate) {}
    }
}

/// The return type of [Lexer::extend_lexeme_if].
/// It represents if the lexeme was extended because the current byte made `predicate return true`
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LexemeStatus {
    Extended,
    NotExtended,
}

// helpers
impl<'a> Lexer<'a> {
    /// Creates a new [Token] using [Self::lexeme] for the lexeme and the given [TokenKind]
    fn token(&self, kind: TokenKind) -> Token<'a> {
        let lexeme = if let EndOfFile = kind {
            ""
        } else {
            self.lexeme()
        };
        Token::new(kind, lexeme, self.line, self.column)
    }
    fn error(&mut self, kind: LexerErrorKind) -> LexerError<'a> {
        LexerError::new(kind, self.token(Unrecognized))
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexerError<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        use core::ops::Not;
        self.end_of_file_emitted.not().then(|| self.next_token())
    }
}

fn byte_is_unrecognized(b: u8) -> bool {
    match b {
        b'(' | b')' | b'{' | b'}' | b',' | b'.' | b'-' | b'+' | b';' | b'*' | b'!' | b'='
        | b'<' | b'>' | b'/' | b'"'
            if b.is_ascii_alphanumeric() || b.is_ascii_whitespace() || b == b'_' =>
        {
            true
        }
        _ => true,
    }
}
fn byte_is_non_newline_whitespace(b: u8) -> bool {
    b != b'\n' && b.is_ascii_whitespace()
}
fn byte_is_digit(b: u8) -> bool {
    b.is_ascii_digit()
}
fn byte_is_identifier(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
fn byte_is(target: u8) -> impl Fn(u8) -> bool {
    move |b| b == target
}
fn byte_is_not(target: u8) -> impl Fn(u8) -> bool {
    move |b| b != target
}
fn trim_first_and_last(s: &str) -> &str {
    s.get(1..s.len().saturating_sub(1)).unwrap_or("")
}
/// Returns the number of extended grapheme clusters in a `s`. see [str::graphemes] from the [unicode_segmentation] crate
fn count_grapheme_clusters(s: &str) -> usize {
    s.graphemes(true).count()
}

pub mod error {
    use crate::token::Token;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LexerErrorKind {
        Unrecognized,
        UnterminatedStringLiteral,
        NumberTrailingDot,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct LexerError<'a> {
        kind: LexerErrorKind,
        token: Token<'a>,
    }
    impl<'a> LexerError<'a> {
        pub const fn new(kind: LexerErrorKind, token: Token<'a>) -> Self {
            Self { kind, token }
        }
        pub const fn token(&self) -> Token<'a> {
            self.token
        }
        pub const fn kind(&self) -> LexerErrorKind {
            self.kind
        }
    }
    impl core::fmt::Display for LexerError<'_> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "Ln {} Col {}: Error lexing: {:?} {:?}",
                self.token.line(),
                self.token.column(),
                self.token.lexeme(),
                self.kind
            )
        }
    }
    impl std::error::Error for LexerError<'_> {}
}

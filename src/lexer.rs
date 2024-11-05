use std::fmt::Display;

use super::token::{Token, TokenKind};

/// Lazily split lox source code into tokens.
/// When used as an [Iterator]: [None] represents a [TokenKind::EndOfFile]
pub struct Lexer<'a> {
    source: &'a str,
    lexeme_start: usize,
    /// index of the byte currently being processed. one after the last byte in the current lexeme
    lexeme_end: usize,
    line_number: usize,
    end_of_file_emitted: bool,
}
impl<'a> Iterator for Lexer<'a> {
    type Item = Result<Token<'a>, LexerError<'a>>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.end_of_file_emitted {
            return None;
        }

        match self.next_token() {
            Ok(token) => Some(Ok(token)),
            Err(error) => Some(Err(error)),
        }
    }
}
impl<'a> Lexer<'a> {
    pub const fn new(source: &'a str) -> Self {
        Self {
            source,
            lexeme_start: 0,
            lexeme_end: 0,
            end_of_file_emitted: false,
            line_number: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Token<'a>, LexerError<'a>> {
        if !self.current_byte_available() {
            self.end_of_file_emitted = true;
            return Ok(Token::end_of_file(self.line_number));
        }

        self.lexeme_start = self.lexeme_end;

        let previous_byte = self.get_current_byte();

        self.consume_current_byte();

        let token = match previous_byte {
            b'(' => self.get_current_token(TokenKind::LeftParentheses),
            b')' => self.get_current_token(TokenKind::RightParentheses),
            b'{' => self.get_current_token(TokenKind::LeftBrace),
            b'}' => self.get_current_token(TokenKind::RightBrace),
            b',' => self.get_current_token(TokenKind::Comma),
            b'.' => self.get_current_token(TokenKind::Dot),
            b'-' => self.get_current_token(TokenKind::Minus),
            b'+' => self.get_current_token(TokenKind::Plus),
            b';' => self.get_current_token(TokenKind::Semicolon),
            b'*' => self.get_current_token(TokenKind::Star),
            b'!' if self.current_byte_available() && self.get_current_byte() == b'=' => {
                self.consume_current_byte();
                self.get_current_token(TokenKind::BangEqual)
            }
            b'!' => self.get_current_token(TokenKind::Bang),
            b'=' if self.current_byte_available() && self.get_current_byte() == b'=' => {
                self.consume_current_byte();
                self.get_current_token(TokenKind::EqualEqual)
            }
            b'=' => self.get_current_token(TokenKind::Equal),
            b'<' if self.current_byte_available() && self.get_current_byte() == b'=' => {
                self.consume_current_byte();
                self.get_current_token(TokenKind::LessEqual)
            }
            b'<' => self.get_current_token(TokenKind::Less),
            b'>' if self.current_byte_available() && self.get_current_byte() == b'=' => {
                self.consume_current_byte();
                self.get_current_token(TokenKind::GreaterEqual)
            }
            b'>' => self.get_current_token(TokenKind::Greater),
            b'/' if self.current_byte_available() && self.get_current_byte() == b'/' => {
                self.consume_comment_line();
                self.next_token()?
            }
            b'/' => self.get_current_token(TokenKind::Slash),
            b'"' => {
                self.consume_string_literal()?;

                // ignore start and end '"'
                let string_literal_lexeme =
                    &self.source[self.lexeme_start + 1..self.lexeme_end - 1];
                Token::new(TokenKind::String, string_literal_lexeme, self.line_number)
            }
            number if number.is_ascii_digit() => {
                self.consume_number_literal()?;
                self.get_current_token(TokenKind::Number)
            }
            alpha if alpha.is_ascii_alphabetic() || alpha == b'_' => {
                self.consume_identifier();
                let token_kind = TokenKind::parse_keyword(self.get_current_lexeme());
                self.get_current_token(token_kind)
            }
            whitespace if whitespace.is_ascii_whitespace() => {
                if whitespace == b'\n' {
                    self.line_number += 1;
                }
                self.consume_whitespace();
                self.next_token()?
            }
            _ => {
                self.consume_unrecognized_lexeme();
                let unrecognized_token = self.get_current_token(TokenKind::Unrecognized);
                return Err(self.error(unrecognized_token, LexerErrorKind::Unrecognized));
            }
        };

        Ok(token)
    }

    /// Increments `self.lexeme_end` making the current lexeme one byte larger
    fn consume_current_byte(&mut self) {
        self.lexeme_end += 1;
    }

    fn current_byte_available(&self) -> bool {
        self.lexeme_end < self.source.len()
    }
    fn next_byte_available(&self) -> bool {
        self.lexeme_end + 1 < self.source.len()
    }

    /// # Panics
    /// when `self.lexeme_end` >= `self.source.len()`. use [Self::current_byte_available] to check
    fn get_current_byte(&self) -> u8 {
        self.source.as_bytes()[self.lexeme_end]
    }
    /// # Panics
    /// when `self.lexeme_end + 1` >= `self.source.len()`. use [Self::next_byte_available] to check
    fn get_next_byte(&self) -> u8 {
        self.source.as_bytes()[self.lexeme_end + 1]
    }

    /// Returns the current lexeme defined by the range `self.lexeme_start..self.lexeme_end`
    fn get_current_lexeme(&self) -> &'a str {
        &self.source[self.lexeme_start..self.lexeme_end]
    }

    /// Creates a new [Token] using [Self::get_current_lexeme] for the lexeme and the given [TokenKind]
    fn get_current_token(&self, kind: TokenKind) -> Token<'a> {
        Token::new(kind, self.get_current_lexeme(), self.line_number)
    }

    /// Makes the current lexeme include all bytes up to and including the first `'\n'`. Only call after `"//"` is found
    fn consume_comment_line(&mut self) {
        while self.current_byte_available() && self.get_current_byte() != b'\n' {
            self.consume_current_byte();
        }
    }
    /// Makes the current lexeme include all bytes up to the first non-ascii whitespace (see [u8::is_ascii_whitespace])
    fn consume_whitespace(&mut self) {
        while self.current_byte_available() && self.get_current_byte().is_ascii_whitespace() {
            if self.get_current_byte() == b'\n' {
                self.line_number += 1;
            }
            self.consume_current_byte();
        }
    }

    /// Makes the current lexeme include all bytes up to and including the closing `'"'`. Only call after an opening '"'
    /// # Error
    /// When there is no closing `'"'`
    fn consume_string_literal(&mut self) -> Result<(), LexerError<'a>> {
        while self.current_byte_available() {
            let current_byte = self.get_current_byte();

            self.consume_current_byte();

            if current_byte == b'"' {
                return Ok(());
            }
        }

        let token = self.get_current_token(TokenKind::String);
        Err(self.error(token, LexerErrorKind::UnterminatedStringLiteral))
    }
    fn consume_number_literal(&mut self) -> Result<(), LexerError<'a>> {
        // consume all digit bytes before the dot
        while self.current_byte_available() && self.get_current_byte().is_ascii_digit() {
            self.consume_current_byte();
        }

        if !self.current_byte_available() {
            return Ok(());
        }

        if self.get_current_byte() == b'.' {
            // there must be a number after the dot
            if !self.next_byte_available() || !self.get_next_byte().is_ascii_digit() {
                let token = self.get_current_token(TokenKind::Number);
                return Err(self.error(token, LexerErrorKind::NumberTrailingDot));
            }

            // consume the dot
            self.consume_current_byte();

            while self.current_byte_available() && self.get_current_byte().is_ascii_digit() {
                self.consume_current_byte();
            }
        }

        Ok(())
    }
    fn consume_identifier(&mut self) {
        while self.current_byte_available()
            && (self.get_current_byte().is_ascii_alphanumeric() || self.get_current_byte() == b'_')
        {
            self.consume_current_byte();
        }
    }
    fn is_current_byte_unrecognized(&self) -> bool {
        match self.get_current_byte() {
            b'(' | b')' | b'{' | b'}' | b',' | b'.' | b'-' | b'+' | b';' | b'*' | b'!' | b'='
            | b'<' | b'>' | b'/' | b'"' => true,
            b if b.is_ascii_alphanumeric() || b.is_ascii_whitespace() || b == b'_' => false,
            _ => true,
        }
    }
    fn consume_unrecognized_lexeme(&mut self) {
        while self.current_byte_available() && self.is_current_byte_unrecognized() {
            self.consume_current_byte();
        }
    }
}

// Error helpers
impl<'a> Lexer<'a> {
    fn calculate_lexeme_position(&self) -> (usize, usize) {
        use unicode_segmentation::UnicodeSegmentation;

        let mut column_number = 1;

        for (i, _c) in self
            .source
            .lines()
            .nth(self.line_number - 1)
            .unwrap()
            .grapheme_indices(true)
        {
            if i == self.lexeme_start {
                break;
            }

            column_number += 1;
        }

        (self.line_number, column_number)
    }
    fn error(&mut self, token: Token<'a>, kind: LexerErrorKind) -> LexerError<'a> {
        let (line_number, column_number) = self.calculate_lexeme_position();

        LexerError {
            kind,
            token,
            line_number,
            column_number,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexerErrorKind {
    Unrecognized,
    UnterminatedStringLiteral,
    NumberTrailingDot,
}
impl Display for LexerErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerErrorKind::NumberTrailingDot => write!(f, "{:?}", self),
            LexerErrorKind::UnterminatedStringLiteral => write!(f, "{:?}", self),
            LexerErrorKind::Unrecognized => write!(f, "Unrecognized token"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexerError<'a> {
    kind: LexerErrorKind,
    token: Token<'a>,
    line_number: usize,
    column_number: usize,
}
impl<'a> LexerError<'a> {
    pub const fn line_number(&self) -> usize {
        self.line_number
    }
    pub const fn token(&self) -> Token<'a> {
        self.token
    }
}
impl Display for LexerError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error lexing {} at line {}, column {}: {}",
            self.token.lexeme(),
            self.line_number,
            self.column_number,
            self.kind,
        )
    }
}
impl std::error::Error for LexerError<'_> {}

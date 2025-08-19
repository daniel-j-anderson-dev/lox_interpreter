use State::*;

use crate::token::{Token, TokenKind};

#[derive(Debug, Clone, Copy)]
pub enum State {
    Start,
    LeftParentheses,
    RightParentheses,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Asterisk,
    CheckForBangEqual,
    Bang,
    BangEqual,
    CheckForEqualEqual,
    Equal,
    EqualEqual,
    CheckForGreaterEqual,
    Greater,
    GreaterEqual,
    CheckForLessEqual,
    Less,
    LessEqual,
    OpenQuote,
    StringBody,
    StringLiteral,
    NewLine,
}
impl State {
    pub const fn is_final(&self) -> bool {
        match self {
            LeftParentheses | RightParentheses | LeftBrace | RightBrace | Comma | Dot | Minus
            | Plus | Semicolon | Asterisk | Bang | Equal | Greater | Less | BangEqual
            | EqualEqual | GreaterEqual | LessEqual | StringLiteral => true,
            _ => false,
        }
    }
    pub const fn is_symbol_consumed(&self) -> bool {
        match self {
            Start | LeftParentheses | RightParentheses | LeftBrace | RightBrace | Comma | Dot
            | Minus | Plus | Semicolon | Asterisk | BangEqual | EqualEqual | GreaterEqual
            | LessEqual | CheckForBangEqual | CheckForEqualEqual | CheckForLessEqual
            | CheckForGreaterEqual | OpenQuote | StringLiteral | StringBody | NewLine => true,
            _ => false,
        }
    }
    pub const fn as_token_kind(&self) -> TokenKind {
        match self {
            LeftParentheses => TokenKind::LeftParentheses,
            RightParentheses => TokenKind::RightParentheses,
            LeftBrace => TokenKind::LeftBrace,
            RightBrace => TokenKind::RightBrace,
            Comma => TokenKind::Comma,
            Dot => TokenKind::Dot,
            Minus => TokenKind::Minus,
            Plus => TokenKind::Plus,
            Semicolon => TokenKind::Semicolon,
            Asterisk => TokenKind::Asterisk,
            Bang => TokenKind::Bang,
            BangEqual => TokenKind::BangEqual,
            Equal => TokenKind::Equal,
            EqualEqual => TokenKind::EqualEqual,
            Greater => TokenKind::Greater,
            GreaterEqual => TokenKind::GreaterEqual,
            Less => TokenKind::Less,
            LessEqual => TokenKind::LessEqual,
            StringLiteral => TokenKind::StringLiteral,
            _ => TokenKind::Unrecognized,
        }
    }
}

const fn transition(state: State, symbol: u8) -> State {
    match (state, symbol) {
        // single symbol tokens
        (Start, b'(') => LeftParentheses,
        (Start, b')') => RightParentheses,
        (Start, b'{') => LeftBrace,
        (Start, b'}') => RightBrace,
        (Start, b',') => Comma,
        (Start, b'.') => Dot,
        (Start, b'-') => Minus,
        (Start, b'+') => Plus,
        (Start, b';') => Semicolon,
        (Start, b'*') => Asterisk,
        (Start, b'\n') => NewLine,

        // two symbol tokens
        (Start, b'!') => CheckForBangEqual,
        (CheckForBangEqual, b'=') => BangEqual,
        (CheckForBangEqual, _) => Bang,
        (Start, b'=') => CheckForEqualEqual,
        (CheckForEqualEqual, b'=') => EqualEqual,
        (CheckForEqualEqual, _) => Equal,
        (Start, b'<') => CheckForLessEqual,
        (CheckForGreaterEqual, b'=') => GreaterEqual,
        (CheckForGreaterEqual, _) => Greater,
        (Start, b'>') => CheckForGreaterEqual,
        (CheckForLessEqual, b'=') => LessEqual,
        (CheckForLessEqual, _) => Less,

        // string literals
        (Start, b'"') => OpenQuote,
        (StringBody, b'"') | (OpenQuote, b'"') => StringLiteral,
        (StringBody, _) | (OpenQuote, _) => StringBody,

        _ => state,
    }
}

fn lex<'a>(source: &'a str) -> impl Iterator<Item = Token<'a>> + use<'a> {
    let mut lexeme_start = 0;
    let mut lexeme_end = 0;
    let mut line = 1;
    let mut column = 0;
    let mut state = Start;

    core::iter::from_fn(move || {
        loop {
            let &symbol = source.as_bytes().get(lexeme_end)?;

            let new_state = transition(state, symbol);

            if new_state.is_symbol_consumed() {
                lexeme_end += 1;
            }

            state = new_state;

            match state {
                NewLine => {
                    state = Start;
                    line += 1;
                    column = 0;
                    lexeme_start = lexeme_end;
                }
                _ if state.is_final() => {
                    let lexeme = &source[lexeme_start..lexeme_end];

                    let token = Token::new(state.as_token_kind(), lexeme, line, column);

                    state = Start;
                    column += lexeme.len();
                    lexeme_start = lexeme_end;
                    return Some(token);
                }
                _ => continue,
            }
        }
    })
}

#[test]
fn test_lex() {
    let source = "!(=)<{>\n}!=,==.<=->=+;*\"\"\"this is a second string literal!\"";
    for token in lex(source) {
        println!("{token}    {:?}", token.kind());
    }
}

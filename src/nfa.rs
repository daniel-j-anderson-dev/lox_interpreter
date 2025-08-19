use State::*;

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
    Star,
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
}
impl State {
    pub const fn is_final(&self) -> bool {
        match self {
            LeftParentheses | RightParentheses | LeftBrace | RightBrace | Comma | Dot | Minus
            | Plus | Semicolon | Star | Bang | Equal | Greater | Less | BangEqual | EqualEqual
            | GreaterEqual | LessEqual => true,
            _ => false,
        }
    }
    pub const fn is_symbol_consumed(&self) -> bool {
        match self {
            Start | LeftParentheses | RightParentheses | LeftBrace | RightBrace | Comma | Dot
            | Minus | Plus | Semicolon | Star | BangEqual | EqualEqual | GreaterEqual
            | LessEqual | CheckForBangEqual | CheckForEqualEqual | CheckForLessEqual
            | CheckForGreaterEqual => true,
            _ => false,
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
        (Start, b'*') => Star,

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

        _ => state,
    }
}

#[test]
fn perform_lexical_analysis_with_nfa() {
    let source = b"!(=)<{>}!=,==.<=->=+;*";
    let mut lexeme_start = 0;
    let mut lexeme_end = 0;
    let mut state = Start;

    loop {
        let symbol = match source.get(lexeme_end) {
            Some(byte) => *byte,
            None => break,
        };

        let new_state = transition(state, symbol);

        if new_state.is_symbol_consumed() {
            lexeme_end += 1;
        }

        println!(
            "({state:?}, {:?}) -> {new_state:?}\n{:?}\nsymbol {}consumed\n",
            symbol as char,
            &source[lexeme_start..lexeme_end],
            if new_state.is_symbol_consumed() {
                ""
            } else {
                "not "
            },
        );

        state = new_state;
        if state.is_final() {
            state = Start;
            lexeme_start = lexeme_end;
        }
    }
}

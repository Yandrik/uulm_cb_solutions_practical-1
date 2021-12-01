use std::str::FromStr;

use crate::lexer::tokens::{OpType, Token, TokenMeta};

#[allow(dead_code)]
const STATES: [i32; 6] = [1, 2, 3, 4, 5, 6];
const FINAL_STATES: [i32; 4] = [2, 3, 4, 5];
const ERROR_STATE: i32 = 6;

/// Transitions in a matrix in the form of this:
/// \CHR ( ) 0..=9 *|/|^|-|+ _ SPACE
/// ST
/// 1    2 3   4       5     6   1
/// 2    6 6   6       6     6   6
/// 3    6 6   6       6     6   6
/// 4    6 6   4       6     6   6
/// 5    6 6   6       6     6   6
/// 6    6 6   6       6     6   6
const TRANSITIONS: [[i32; 6]; 6] = [
    [2, 3, 4, 5, 6, 1],
    [6, 6, 6, 6, 6, 6],
    [6, 6, 6, 6, 6, 6],
    [6, 6, 4, 6, 6, 6],
    [6, 6, 6, 6, 6, 6],
    [6, 6, 6, 6, 6, 6],
];


#[derive(Debug, Copy, Clone)]
pub struct FSM {
    state: i32,
    last: i32,
}

#[allow(dead_code)]
impl FSM {
    #[inline]
    pub fn new() -> FSM {
        FSM { state: 1, last: 1 }
    }

    pub fn is_final(&self) -> bool {
        FINAL_STATES.contains(&self.state)
    }

    pub fn is_error(&self) -> bool {
        ERROR_STATE == self.state
    }

    /// revert to last state
    pub fn revert(&mut self) {
        self.state = self.last;
    }

    pub fn get_state(&self) -> i32 {
        self.state
    }

    pub fn transition(&mut self, c: char) {
        let new_state = self.get_transition(c);
        self.last = self.state;
        self.state = new_state;
    }

    fn get_transition(&self, c: char) -> i32 {
        let lut_col = match c {
            '(' => 0,
            ')' => 1,
            '0'..='9' => 2,
            '*' | '/' | '^' | '-' | '+' => 3,
            ' ' => 5,
            _ => 4,
        };
        TRANSITIONS[(&self.state - 1) as usize][lut_col]
    }
}

pub fn get_token(fsm: &FSM, cbuf: &Vec<char>, meta: TokenMeta) -> Option<Token> {
    match fsm.state {
        1 | 6 => None,
        2 => Some(Token::OBR(meta)),
        3 => Some(Token::CBR(meta)),
        4 => Some(Token::ID(meta, f64::from_str(&cbuf.iter().collect::<String>()).unwrap())),
        5 => Some(Token::OP(meta, OpType::from_char(*cbuf.last().unwrap()).unwrap())),
        _ => panic!("Invalid State {}!", fsm.state),
    }
}

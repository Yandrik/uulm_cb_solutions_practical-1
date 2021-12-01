use tokens::Token;

use crate::lexer::errors::LexerErrors;
use crate::lexer::fsm::FSM;
use crate::lexer::tokens::TokenMeta;

pub mod errors;
pub mod tokens;
mod fsm;

pub type Result<T> = std::result::Result<T, errors::LexerErrors>;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    read_pointer: usize,
}


impl Lexer {
    /// Create a new Lexer for the given String
    /// Example:
    /// ```
    /// use cb_calculator::lexer;
    /// let text = "some text";
    /// lexer::Lexer::new(String::from(text));
    /// ```
    #[inline]
    pub fn new(input: &str) -> Lexer {
        Lexer { input: input.chars().collect(), pos: 0, read_pointer: 0 }
    }

    // Get the next token
    pub fn next(&mut self) -> Result<Option<Token>> {
        if let Some((fsm, cbuf)) = self.longest_token_prefix() {
            if let Some(token) = fsm::get_token(&fsm, &cbuf, TokenMeta { pos: self.pos }) {
                // if a token could be derived
                self.pos = self.read_pointer;
                Ok(Some(token))
            } else {
                // If no token could be found
                if self.input.len() > self.read_pointer {
                    // If not at the end of the input
                    Err(LexerErrors::InvalidTokenError {
                        token: self.input.get(self.pos..).unwrap()
                            .iter()
                            .take(5)
                            .collect(),
                        pos: self.pos,
                    })
                } else {
                    // If at the end of the input
                    Ok(None)
                }
            }
        } else {
            // if no more tokens are there
            Ok(None)
        }
    }

    fn longest_token_prefix(&mut self) -> Option<(FSM, Vec<char>)> {
        let mut fsm = FSM::new();
        let mut chars = match self.input.get(self.pos..) {
            Some(slice) => slice.iter(),
            None => return None,
        };
        let mut cbuf: Vec<char> = vec![];

        while let Some(cur) = chars.next() {
            fsm.transition(*cur);
            if fsm.is_final() {
                cbuf.push(*cur);
            } else if fsm.is_error() {
                fsm.revert();
                break;
            }
            self.read_pointer += 1;
        }
        Some((fsm, cbuf))
    }
}

#[warn(dead_code)]
#[cfg(test)]
mod test {
    use tokens::OpType;

    use super::*;

    #[test]
    fn parser_test() {
        let mut lexer = Lexer::new("15/3^2+20-(5*60)");
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 0 }, 15.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OP(TokenMeta { pos: 2 }, OpType::DIV)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 3 }, 3.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OP(TokenMeta { pos: 4 }, OpType::POW)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 5 }, 2.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OP(TokenMeta { pos: 6 }, OpType::ADD)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 7 }, 20.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OP(TokenMeta { pos: 9 }, OpType::SUB)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OBR(TokenMeta { pos: 10 })))));
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 11 }, 5.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OP(TokenMeta { pos: 12 }, OpType::MUL)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 13 }, 60.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::CBR(TokenMeta { pos: 15 })))));
        assert!(matches!(lexer.next(), Ok(None)));
    }

    #[test]
    fn parser_error_test() {
        let mut lexer = Lexer::new("15+@");
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta{ pos: 0 }, 15.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OP(TokenMeta{ pos: 2 }, OpType::ADD)))));
        assert!(matches!(lexer.next(), Err(LexerErrors::InvalidTokenError { token: _, pos: _ })))
    }

    #[test]
    fn spaces_are_ignored() {
        let mut lexer = Lexer::new("  15 / 3  ^ 2 +     20 -(    5  * 60   )  ");
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 0 }, 15.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OP(TokenMeta { pos: 4 }, OpType::DIV)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 6 }, 3.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OP(TokenMeta { pos: 8 }, OpType::POW)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 11 }, 2.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OP(TokenMeta { pos: 13},  OpType::ADD)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 15 }, 20.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OP(TokenMeta { pos: 22 }, OpType::SUB)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OBR(TokenMeta { pos: 24 })))));
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 25 }, 5.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::OP(TokenMeta { pos: 30 }, OpType::MUL)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::ID(TokenMeta { pos: 33 }, 60.0)))));
        assert!(matches!(lexer.next(), Ok(Some(Token::CBR(TokenMeta { pos: 36 })))));
        assert!(matches!(lexer.next(), Ok(None)));
    }
}

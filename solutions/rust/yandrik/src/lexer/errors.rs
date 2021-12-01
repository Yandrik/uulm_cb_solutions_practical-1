use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerErrors {
    #[error("unexpected character {token} at position {pos}")]
    InvalidTokenError {
        token: String,
        pos: usize,
    },
    #[error("cannot lex an empty text sequence")]
    EmptyTextSequenceError,
}

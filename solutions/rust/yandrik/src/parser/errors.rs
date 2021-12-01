use thiserror::Error;

use crate::lexer::errors::LexerErrors;
use crate::lexer::tokens::Token;

#[derive(Debug, Error)]
pub enum ParserErrors {
    #[error("unexpected token: {0:?} in procedure {1}")]
    UnexpectedTokenError(Token, String),
    #[error("input ended prematurely")]
    MissingTokenError,
    #[error("lexer error")]
    LexerError(#[from] LexerErrors),
}

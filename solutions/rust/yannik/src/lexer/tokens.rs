/// # Token Metadata
/// Data contained is:
/// * File that the token was parsed in
/// * Line that the token was parsed in
/// * Position of the *first character making up the token* in said line
#[derive(Debug, Clone)]
pub struct TokenMeta {
    pub pos: usize,
}

#[derive(Debug, Clone)]
pub enum OpType {
    MUL,
    DIV,
    ADD,
    SUB,
    POW,
}

impl OpType {
    #[inline]
    pub fn from_char(c: char) -> Option<OpType> {
        match c {
            '*' => Some(OpType::MUL),
            '/' => Some(OpType::DIV),
            '+' => Some(OpType::ADD),
            '-' => Some(OpType::SUB),
            '^' => Some(OpType::POW),
            _ => None,
        }
    }
}

/// # Tokens
/// The tokens all contain [metadata](TokenMeta).
/// 1. `ID`: A number, parsed into 64 bit floating-point.
/// 1. `OBR`: An opening bracket (`(`).
/// 1. `CBR`: A closing bracket (`)`).
/// 1. `OP`: An operation. Containing an [Operation Type](OpType).
#[derive(Debug, Clone)]
pub enum Token {
    ID(TokenMeta, f64),
    OBR(TokenMeta),
    CBR(TokenMeta),
    OP(TokenMeta, OpType),
}

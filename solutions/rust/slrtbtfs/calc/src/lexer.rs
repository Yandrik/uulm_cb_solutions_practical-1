use crate::location::*;
use std::{iter::Peekable, str::Chars};

/**
 * The item kinds that can be possibly emitted by the lexer.
 * Errors are encoded in items and need to be brought to the user by the parser.
 */
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ItemKind {
    Number { value: u64 },
    LParen,
    RParen,
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    Eof,
    Error { reason: String },
}

impl ItemKind {
    /**
     * Description of items, for example for usage in error messages.
     */
    pub fn description(&self) -> String {
        match self {
            Number { value } => format!("Number ({})", value),
            LParen => "Opening Parenthesis '('".to_string(),
            RParen => "Closing Parenthesis ')'".to_string(),
            Add => "Operator '+'".to_string(),
            Sub => "Operator '-'".to_string(),
            Mul => "Operator '*'".to_string(),
            Div => "Operator '/'".to_string(),
            Pow => "Operator '^'".to_string(),
            Error { reason } => format!("lexer error: {}", reason),
            Eof => "end of input".to_string(),
        }
    }
}

/**
 * Items that are emitted by the lexer, with position and kind information.
 */
#[derive(Debug, PartialEq, Eq)]
pub struct Item {
    pub kind: ItemKind,
    pub location: Location,
}

/**
 * The Lexer takes its data from an iterator of chars and acts itself as an iterator of items.
 */
pub struct Lexer<'a> {
    position: Position,
    char_source: Peekable<Chars<'a>>,
}
use ItemKind::*;
impl Iterator for Lexer<'_> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.eat_whitespace();

        let start = self.position;

        let kind = match self.consume() {
            Some(first_digit @ '0'..='9') => {
                let mut value = first_digit as u64 - '0' as u64;
                while let Some(digit @ '0'..='9') = self.peek() {
                    value *= 10;
                    value += *digit as u64 - '0' as u64;
                    self.consume();
                }

                Number { value }
            }
            Some('(') => LParen,
            Some(')') => RParen,
            Some('+') => Add,
            Some('-') => Sub,
            Some('*') => Mul,
            Some('/') => Div,
            Some('^') => Pow,
            Some(unknown) => Error {
                reason: format!("Unknown Symbol: '{}'.", unknown),
            },
            None => Eof,
        };
        let end = self.position;
        Some(Item {
            kind,
            location: Location { start, end },
        })
    }
}

impl Lexer<'_> {
    /**
     * Create a lexer from a string.
     */
    pub fn lex(str: &str) -> Lexer<'_> {
        Lexer {
            position: Position { line: 1, column: 1 },
            char_source: str.chars().peekable(),
        }
    }

    /**
     * Consume all leading whitespace characters.
     */
    fn eat_whitespace(&mut self) {
        while let Some(' ' | '\n' | '\t') = self.peek() {
            self.consume();
        }
    }

    /**
     * Consumes one char and updates the position counter.
     * Returns None if the `char_source` is depleted.
     */
    fn consume(&mut self) -> Option<char> {
        let next = self.char_source.next()?;
        match next {
            '\n' => {
                self.position.line += 1;
                self.position.column = 1
            }
            _ => self.position.column += 1,
        }

        Some(next)
    }

    /**
     * Peeks the next char without consuming it or updating any counters.
     * Returns None if the `char_source` is depleted.
     */
    fn peek(&mut self) -> Option<&char> {
        self.char_source.peek()
    }
}

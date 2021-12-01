use crate::parser::Parser;
use crate::{location::Location, location::Position};

use crate::lexer::{Item, ItemKind::*, Lexer};

#[test]
fn test_eval() {
    let test_cases = vec![
        ("2 + 5 / 3 * 2", Ok(2.0 + 5.0 / 3.0 * 2.0)),
        ("(2 + 5) / 3", Ok((2.0 + 5.0) / 3.0)),
        ("((10 - 213) * 25) + 27", Ok(((10.0 - 213.0) * 25.0) + 27.0)),
        ("(7 - 3) * 43", Ok((7.0 - 3.0) * 43.0)),
        (
            "((1 + 1) * 1 - 2",
            Err("1:17: expected Closing Parenthesis ')', got end of input".to_string()),
        ),
        (
            "a + 1 * 3",
            Err("1:1: expected Number, got lexer error: Unknown Symbol: 'a'.".to_string()),
        ),
        ("++---+-(-+2)", Ok(-2.)),
        ("2^2^3", Ok(256.)),
        (
            "2 2",
            Err("1:3: expected end of input, got Number (2)".to_string()),
        ),
    ];

    for (expression, expectation) in test_cases.iter() {
        let reality = Parser::eval(*expression);
        assert_eq!(
            reality, *expectation,
            "Testing expression \"{}\"",
            expression
        );
    }
}

#[test]
fn test_lexer() {
    let input = "
)(?42+/* 1312 *
-^";
    let tokens: Vec<Item> = Lexer::lex(input)
        .take_while(|item| item.kind != Eof)
        .collect();

    let expected = vec![
        Item {
            kind: RParen,
            location: Location {
                start: Position { line: 2, column: 1 },
                end: Position { line: 2, column: 2 },
            },
        },
        Item {
            kind: LParen,
            location: Location {
                start: Position { line: 2, column: 2 },
                end: Position { line: 2, column: 3 },
            },
        },
        Item {
            kind: Error {
                reason: "Unknown Symbol: '?'.".to_string(),
            },
            location: Location {
                start: Position { line: 2, column: 3 },
                end: Position { line: 2, column: 4 },
            },
        },
        Item {
            kind: Number { value: 42 },
            location: Location {
                start: Position { line: 2, column: 4 },
                end: Position { line: 2, column: 6 },
            },
        },
        Item {
            kind: Add,
            location: Location {
                start: Position { line: 2, column: 6 },
                end: Position { line: 2, column: 7 },
            },
        },
        Item {
            kind: Div,
            location: Location {
                start: Position { line: 2, column: 7 },
                end: Position { line: 2, column: 8 },
            },
        },
        Item {
            kind: Mul,
            location: Location {
                start: Position { line: 2, column: 8 },
                end: Position { line: 2, column: 9 },
            },
        },
        Item {
            kind: Number { value: 1312 },
            location: Location {
                start: Position {
                    line: 2,
                    column: 10,
                },
                end: Position {
                    line: 2,
                    column: 14,
                },
            },
        },
        Item {
            kind: Mul,
            location: Location {
                start: Position {
                    line: 2,
                    column: 15,
                },
                end: Position {
                    line: 2,
                    column: 16,
                },
            },
        },
        Item {
            kind: Sub,
            location: Location {
                start: Position { line: 3, column: 1 },
                end: Position { line: 3, column: 2 },
            },
        },
        Item {
            kind: Pow,
            location: Location {
                start: Position { line: 3, column: 2 },
                end: Position { line: 3, column: 3 },
            },
        },
    ];
    assert_eq!(tokens.len(), expected.len());
    tokens
        .iter()
        .zip(expected.iter())
        .for_each(|(left, right)| assert_eq!(*left, *right));
}

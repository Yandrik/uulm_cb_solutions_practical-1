use std::iter::Peekable;

use crate::{
    lexer::{
        Item,
        ItemKind::{self, *},
        Lexer,
    },
    syntax_tree::Node,
    syntax_tree::NodeKind::*,
};

/**
 * Parser parses a expression of the following grammar:
 *
 * Regular expression syntax is used here, since we
 * can easily do that in a  hand rolled parser.
 *
 * ```text
 * S       -> Expr#
 * Expr    -> Sum
 * Sum     -> Mul (SumOp Mul)*
 * Mul     -> Unary (MulOp Unary)*
 * Unary   -> Power | SumOp Unary
 * Power   -> Atomic | Atomic PowerOp Power
 * Atomic  -> Number | Paren
 * Paren   -> '(' Expr ')'
 *
 * SumOp   -> '+' | '-'
 * MulOp   -> '*' | '/'
 * PowerOp -> '^'
 * ```
 */
pub struct Parser<'a> {
    lexer: Peekable<Lexer<'a>>,
}

impl Parser<'_> {
    /**
     * Parses a string.
     */
    pub fn parse(lexer: Lexer) -> Result<Box<Node>, String> {
        let mut parser = Parser {
            lexer: lexer.peekable(),
        };
        let expr = parser.parse_expr()?;
        // Check that all input has been consumed.
        parser.expect_next(Eof)?;

        Ok(expr)
    }
    /** Parses and evaluates a string. */
    pub fn eval(expression: &str) -> Result<f64, String> {
        let lexer = Lexer::lex(expression);

        let syntax_tree = Parser::parse(lexer)?;
        Ok(syntax_tree.eval())
    }

    /**
     * Parses rule
     *
     * ```
     * Expr    -> Sum  
     */
    fn parse_expr(&mut self) -> Result<Box<Node>, String> {
        self.parse_sum()
    }
    /**
     * Parses rule
     * ```
     * Sum     -> Mul (SumOp Mul)*
     * SumOp   -> '+' | '-'
     */
    fn parse_sum(&mut self) -> Result<Box<Node>, String> {
        let mut left = self.parse_mul()?;
        loop {
            if let operator @ (Add | Sub) = self.peek_kind() {
                let op = self.lexer.next().unwrap();
                let right = self.parse_mul()?;
                left = Box::new(Node {
                    location: op.location.join(&right.location),
                    kind: BinaryExpr {
                        left,
                        operator,
                        right,
                    },
                });
            } else {
                return Ok(left);
            }
        }
    }
    /**
     * Parses rule
     * ```
     * Mul     -> Unary (MulOp Unary)*
     * MulOp   -> '*' | '/'
     */
    fn parse_mul(&mut self) -> Result<Box<Node>, String> {
        let mut left = self.parse_unary()?;
        loop {
            if let operator @ (Mul | Div) = self.peek_kind() {
                let op = self.lexer.next().unwrap();
                let right = self.parse_unary()?;
                left = Box::new(Node {
                    location: op.location.join(&right.location),
                    kind: BinaryExpr {
                        left,
                        operator,
                        right,
                    },
                });
            } else {
                return Ok(left);
            }
        }
    }
    /**
     * Parses rule
     * ```
     * Unary   -> Power | SumOp Unary
     * SumOp   -> '+' | '-'
     */
    fn parse_unary(&mut self) -> Result<Box<Node>, String> {
        if let operator @ (Add | Sub) = self.peek_kind() {
            let op = self.lexer.next().unwrap();
            let inner = self.parse_unary()?;
            Ok(Box::new(Node {
                location: op.location.join(&inner.location),
                kind: UnaryExpr { operator, inner },
            }))
        } else {
            self.parse_power()
        }
    }
    /**
     * Parses rule
     * ```
     * Power   -> Atomic | Atomic PowerOp Power
     * PowerOp -> '^'
     */
    fn parse_power(&mut self) -> Result<Box<Node>, String> {
        let left = self.parse_atomic()?;
        if self.peek_kind() == Pow {
            self.expect_next(Pow)?;
            let right = self.parse_power()?;
            Ok(Box::new(Node {
                location: left.location.join(&right.location),
                kind: BinaryExpr {
                    left,
                    operator: Pow,
                    right,
                },
            }))
        } else {
            Ok(left)
        }
    }
    /**
     * Parses rule
     * ```
     * Atomic  -> Number | Paren
     */
    fn parse_atomic(&mut self) -> Result<Box<Node>, String> {
        if self.peek_kind() == LParen {
            self.parse_paren()
        } else {
            self.parse_number_literal()
        }
    }
    /**
     * Parses rule
     * ```
     * Paren   -> '(' Expr ')'
     */
    fn parse_paren(&mut self) -> Result<Box<Node>, String> {
        let left_paren = self.expect_next(LParen)?;
        let inner = self.parse_expr()?;
        let right_paren = self.expect_next(RParen)?;

        Ok(Box::new(Node {
            location: left_paren.location.join(&right_paren.location),
            kind: Paren { inner },
        }))
    }

    /**
     * Parses a single number literal.
     */
    fn parse_number_literal(&mut self) -> Result<Box<Node>, String> {
        let next = self.lexer.next().unwrap();
        if let Item {
            kind: Number { value },
            location,
        } = next
        {
            Ok(Box::new(Node {
                location,
                kind: NumberLiteral { value },
            }))
        } else {
            Parser::error("Number".to_string(), next)
        }
    }

    /**
     * This is called if the only possibly following item, assuming valid input is `expected`.
     * If different input is encountered from the lexer, an error is yielded, if the expectation matches reality the next lexer item is returned.
     */
    fn expect_next(&mut self, expected: ItemKind) -> Result<Item, String> {
        let next = self.lexer.next().unwrap();
        if next.kind == expected {
            Ok(next)
        } else {
            Parser::error(expected.description(), next)
        }
    }

    /**
     * Helper function for formatting error messages.
     */
    fn error<T>(expected: String, reality: Item) -> Result<T, String> {
        Err(format!(
            "{} expected {}, got {}",
            reality.location.to_string(),
            expected,
            reality.kind.description()
        ))
    }

    /**
     * Peek at the next item from the lexer.
     */
    fn peek_kind(&mut self) -> ItemKind {
        self.lexer.peek().unwrap().kind.clone()
    }
}

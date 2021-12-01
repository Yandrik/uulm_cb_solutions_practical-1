use crate::lexer::ItemKind;
use crate::location::Location;

/**
 * Nodes in the syntax tree consist of a location an node kinds specific information.
 */
pub struct Node {
    pub location: Location,
    pub kind: NodeKind,
}

/**
 * Possible node kinds.
 */
pub enum NodeKind {
    /** Any arithmetic operator which takes two arguments, e.g `left + right`. */
    BinaryExpr {
        left: Box<Node>,
        operator: ItemKind,
        right: Box<Node>,
    },
    /** Any arithmetic operator which takes a single argument, e.g. `-inner`. */
    UnaryExpr {
        operator: ItemKind,
        inner: Box<Node>,
    },
    /** A single number literal, without signs. */
    NumberLiteral { value: u64 },
    /** An expression wrapped in parentheses.  */
    Paren { inner: Box<Node> },
}

impl Node {
    /** Evaluates a syntax tree node arithmetically, using recursive pattern matching. */
    pub fn eval(&self) -> f64 {
        use ItemKind::*;
        match &self.kind {
            NodeKind::BinaryExpr {
                left,
                operator,
                right,
            } => {
                let a = left.eval();
                let b = right.eval();
                match operator {
                    Add => a + b,
                    Sub => a - b,
                    Mul => a * b,
                    Div => a / b,
                    Pow => a.powf(b),
                    invalid => panic!("Invalid binary operator: {}", invalid.description()),
                }
            }
            NodeKind::UnaryExpr { operator, inner } => {
                let a = inner.eval();
                match operator {
                    Add => a,
                    Sub => -a,
                    invalid => panic!("Invalid unary operator: {}", invalid.description()),
                }
            }
            NodeKind::NumberLiteral { value } => *value as f64,
            NodeKind::Paren { inner } => inner.eval(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/**
 * Describes a single position in the source code.
 * Both column and line numbers start at 1.
 */
pub struct Position {
    pub line: u32,
    pub column: u32,
}

/**
 * Describes a span between two positions in the code.
 */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Location {
    pub start: Position,
    pub end: Position,
}

/**
 * The location format is based on the one common in gcc, clang, et. al.
 */
impl ToString for Location {
    fn to_string(&self) -> String {
        format!("{}:{}:", self.start.line, self.start.column)
    }
}

impl Location {
    /** Joins the spans defined by two locations. */
    pub fn join(&self, right: &Location) -> Location {
        Location {
            start: self.start,
            end: right.end,
        }
    }
}

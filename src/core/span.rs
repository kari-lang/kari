#[derive(Clone, Copy, Debug)]
pub struct Span {
    /// The position of the first character in the span
    pub start: Position,

    /// The position of the last character in the span
    pub end: Position,
}

impl Span {
    pub fn merge(mut self, other: Self) -> Self {
        if self.start > other.start {
            self.start = other.start;
        }
        if self.end < other.end {
            self.end = other.end;
        }
        self
    }
}


#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Position {
    /// The line, starting at 0
    pub line: usize,

    /// The column, starting at 0
    pub column: usize,

    /// The byte index within the stream
    pub index: usize,
}

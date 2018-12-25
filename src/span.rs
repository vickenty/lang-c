//! Source text location tracking
use std::usize::MAX;
use std::{cmp, fmt};

/// Byte offset of a node start and end positions in the input stream
#[derive(Copy, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Span {
    /// Create a new span for a specific location
    pub fn span(start: usize, end: usize) -> Span {
        Span {
            start: start,
            end: end,
        }
    }

    /// Create a new undefined span that is equal to any other span
    pub fn none() -> Span {
        Span {
            start: MAX,
            end: MAX,
        }
    }

    /// Test if span is undefined
    pub fn is_none(&self) -> bool {
        self.start == MAX && self.end == MAX
    }
}

impl cmp::PartialEq for Span {
    fn eq(&self, other: &Self) -> bool {
        (self.start == other.start && self.end == other.end) || self.is_none() || other.is_none()
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        if !self.is_none() {
            write!(fmt, "{}…{}", self.start, self.end)
        } else {
            write!(fmt, "…")
        }
    }
}

/// Associate a span with an arbitrary type
#[derive(Debug, PartialEq, Clone)]
pub struct Node<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Node<T> {
    /// Create new node
    pub fn new(node: T, span: Span) -> Node<T> {
        Node {
            node: node,
            span: span,
        }
    }
}

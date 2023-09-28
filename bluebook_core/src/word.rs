use crate::{buffer::TextBuffer, codepoint::CharClassification};

/// A word boundary can be the start of a word, its end or both for punctuation
#[derive(PartialEq, Eq)]
pub enum WordBoundary {
    /// Denote that this is not a boundary
    Interior,
    /// A boundary indicating the end of a word
    Start,
    /// A boundary indicating the start of a word
    End,
    /// Both start and end boundaries (ex: punctuation characters)
    Both,
}

impl WordBoundary {
    pub fn is_start(&self) -> bool {
        *self == WordBoundary::Start || *self == WordBoundary::Both
    }

    pub fn is_end(&self) -> bool {
        *self == WordBoundary::End || *self == WordBoundary::Both
    }

    #[allow(unused)]
    pub fn is_boundary(&self) -> bool {
        *self != WordBoundary::Interior
    }

    pub fn new(prev: CharClassification, next: CharClassification) -> Self {
        use self::{CharClassification::*, WordBoundary::*};
        match (prev, next) {
            (Lf, Lf) => Start,
            (Lf, Space) => Interior,
            (Cr, Lf) => Interior,
            (Space, Lf) => Interior,
            (Space, Cr) => Interior,
            (Space, Space) => Interior,
            (_, Space) => End,
            (Space, _) => Start,
            (Lf, _) => Start,
            (_, Cr) => End,
            (_, Lf) => End,
            (Punctuation, Other) => Both,
            (Other, Punctuation) => Both,
            _ => Interior,
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum WordCursorError {
    #[error("Invalid word encountered")]
    InvalidCharacter,
    // Add more error variants as needed
}

/// A cursor providing utility function to navigate the rope
/// by word boundaries.
/// Boundaries can be the start of a word, its end, punctuation etc.
pub trait WordCursor<'buffer>: Iterator<Item = usize> + DoubleEndedIterator<Item = usize> {
    type Buffer: TextBuffer;
    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self;
    fn offset(&self) -> usize;

    /// Return the previous and end boundaries of the word under cursor.
    fn select_word(&mut self) -> (usize, usize);
}

use crate::{mode::Mode, text_buffer::TextBuffer};

/// Describe char classifications used to compose word boundaries
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CharClassification {
    /// Carriage Return (`r`)
    Cr,
    /// Line feed (`\n`)
    Lf,
    /// Whitespace character
    Space,
    /// Any punctuation character
    Punctuation,
    /// Includes letters and all of non-ascii unicode
    Other,
}

/// A word boundary can be the start of a word, its end or both for punctuation
#[derive(PartialEq, Eq)]
enum WordBoundary {
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
    fn is_start(&self) -> bool {
        *self == WordBoundary::Start || *self == WordBoundary::Both
    }

    fn is_end(&self) -> bool {
        *self == WordBoundary::End || *self == WordBoundary::Both
    }

    #[allow(unused)]
    fn is_boundary(&self) -> bool {
        *self != WordBoundary::Interior
    }
}

fn classify_boundary(prev: CharClassification, next: CharClassification) -> WordBoundary {
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

#[derive(thiserror::Error, Debug)]
pub enum WordCursorError {
    #[error("Invalid word encountered")]
    InvalidCharacter,
    // Add more error variants as needed
}

/// A cursor providing utility function to navigate the rope
/// by word boundaries.
/// Boundaries can be the start of a word, its end, punctuation etc.
pub trait WordCursor<'buffer> {
    type Buffer: TextBuffer;
    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self;
    /// Get the previous start boundary of a word, and set the cursor position to the boundary found.
    /// The behaviour diffs a bit on new line character with modal and non modal,
    /// while on modal, it will ignore the new line character and on non-modal,
    /// it will stop at the new line character
    fn prev_boundary(&mut self, mode: Mode) -> Option<usize>;
    /// Computes where the cursor position should be after backward deletion.
    fn prev_deletion_boundary(&mut self) -> Option<usize>;
    /// Get the position of the next non blank character in the rope
    fn next_non_blank_char(&mut self) -> usize;
    /// Get the next start boundary of a word, and set the cursor position to the boundary found.
    fn next_boundary(&mut self) -> Option<usize>;
    /// Get the next end boundary, and set the cursor position to the boundary found.
    fn end_boundary(&mut self) -> Option<usize>;
    /// Get the first matching [`CharClassification::Other`] backward and set the cursor position to this location .
    fn prev_code_boundary(&mut self) -> usize;
    /// Get the first matching [`CharClassification::Other`] forward and set the cursor position to this location .
    fn next_code_boundary(&mut self) -> usize;
    /// Return the previous and end boundaries of the word under cursor.
    fn select_word(&mut self) -> (usize, usize);
}

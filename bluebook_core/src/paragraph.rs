use crate::text_buffer::TextBuffer;

/// Describe char classifications used to compose word boundaries
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CharClassification {
    /// Carriage Return (`r`)
    Cr,
    /// Line feed (`\n`)
    Lf,
    /// Includes letters and all of non-ascii unicode
    Other,
}

/// A word boundary can be the start of a word, its end or both for punctuation
#[derive(PartialEq, Eq)]
enum ParagraphBoundary {
    /// Denote that this is not a boundary
    Interior,
    /// A boundary indicating the end of a new-line sequence
    Start,
    /// A boundary indicating the start of a new-line sequence
    End,
    /// Both start and end boundaries (when we have only one empty
    /// line)
    Both,
}

impl ParagraphBoundary {
    fn is_start(&self) -> bool {
        *self == ParagraphBoundary::Start || *self == ParagraphBoundary::Both
    }

    fn is_end(&self) -> bool {
        *self == ParagraphBoundary::End || *self == ParagraphBoundary::Both
    }

    #[allow(unused)]
    fn is_boundary(&self) -> bool {
        *self != ParagraphBoundary::Interior
    }
}

/// Return the [`CharClassification`] of the input character
pub fn get_char_property(codepoint: char) -> CharClassification {
    match codepoint {
        '\r' => CharClassification::Cr,
        '\n' => CharClassification::Lf,
        _ => CharClassification::Other,
    }
}

fn classify_boundary(
    before_prev: CharClassification,
    prev: CharClassification,
    next: CharClassification,
    after_next: CharClassification,
) -> ParagraphBoundary {
    use self::{CharClassification::*, ParagraphBoundary::*};

    match (before_prev, prev, next, after_next) {
        (Other, Lf, Lf, Other) => Both,
        (_, Lf, Lf, Other) => Start,
        (Lf, Cr, Lf, Other) => Start,
        (Other, Lf, Lf, _) => End,
        (Other, Cr, Lf, Cr) => End,
        _ => Interior,
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ParagraphCursorError {
    #[error("Invalid character encountered")]
    InvalidParagraph,
    // Add more error variants as needed
}

/// A cursor providing utility function to navigate the rope
/// by parahraphs boundaries.
/// Boundaries can be the start of a word, its end, punctuation etc.
pub trait ParagraphCursor<'buffer> {
    type Buffer: TextBuffer;
    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self;
    fn prev_boundary(&mut self) -> Option<usize>;
    fn next_boundary(&mut self) -> Option<usize>;
}

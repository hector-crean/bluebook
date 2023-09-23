

use crate::{
    buffer::peritext_buffer::cursor_impl::CursorRange, graphemes::UnicodeSegmentationError,
};

/// A cursor with convenience functions for moving through a TextBuffer.
///

/// Zero indexed cursor coordinates
/// Dinstinguish between 'document' and 'view' coordunates. i.e. the rendered position will not always be the same as the actual position
#[derive(Debug)]
pub struct CursorDocCoords {
    pub row: usize,
    pub col: usize,
}
impl CursorDocCoords {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    pub fn transform_to_view_coords(&self) -> CursorViewCoords {
        CursorViewCoords {
            row: self.row,
            col: self.col,
        }
    }
}

#[derive(Debug)]
pub struct CursorViewCoords {
    pub row: usize,
    pub col: usize,
}
impl CursorViewCoords {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

pub trait TextBufferCursor<'cursor> {
    // /// Set cursor position.
    fn set_anchor(self, byte_offset: usize) -> Self;

    fn set_head(self, byte_offset: usize) -> Self;

    fn set_point(self, byte_offset: usize) -> Self;

    // /// Get cursor position.
    fn anchor(&self) -> usize;

    fn head(&self) -> usize;

    fn range(&self) -> CursorRange;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn prev_grapheme_boundary(&self) -> Result<Option<usize>, UnicodeSegmentationError>;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn next_grapheme_boundary(&self) -> Result<Option<usize>, UnicodeSegmentationError>;

    // fn peek_prev_grapheme(&self) -> Option<&str>;

    // fn peek_next_grapheme(&self) -> Option<&str>;

    fn nth_next_grapheme_boundary(
        &self,
        n: usize,
    ) -> Result<Option<usize>, UnicodeSegmentationError>;

    fn nth_prev_grapheme_boundary(
        &self,
        n: usize,
    ) -> Result<Option<usize>, UnicodeSegmentationError>;

    /// Check if cursor position is at a codepoint boundary.
    fn is_grapheme_boundary(&self) -> Result<bool, UnicodeSegmentationError>;

    // fn move_head_horizontally(self, dir: Direction, count: usize, behaviour: Movement) -> Self;

    // /// Get the previous word offset from the given offset, if it exists.
    // fn prev_word_offset(&self, offset: usize) -> Option<usize>;

    // /// Get the next word offset from the given offset, if it exists.
    // fn next_word_offset(&self, offset: usize) -> Option<usize>;

    // /// Get the previous codepoint offset from the given offset, if it exists.
    // fn prev_codepoint_offset(&self, offset: usize) -> Option<usize>;

    // /// Get the next codepoint offset from the given offset, if it exists.
    // fn next_codepoint_offset(&self, offset: usize) -> Option<usize>;

    // /// Get the preceding line break offset from the given offset
    // fn preceding_line_break(&self, offset: usize) -> usize;

    // /// Get the next line break offset from the given offset
    // fn next_line_break(&self, offset: usize) -> usize;
}

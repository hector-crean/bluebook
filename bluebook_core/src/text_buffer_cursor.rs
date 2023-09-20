use crate::buffer::peritext_buffer::cursor_impl::CursorRange;

/// A cursor with convenience functions for moving through a TextBuffer.
///

#[derive(thiserror::Error, Debug)]
pub enum TextBufferCursorError {
    #[error("Could not find prev grapheme")]
    PrevGraphemeOffsetError,
    #[error("Could not find next grapheme")]
    NextGraphemeOffsetError,
    #[error("write error: {content:?}")]
    WriteError { content: String },
    #[error("byte offset {byte_offset:?} is not a codepoint boundary")]
    CodepointBoundaryError { byte_offset: usize },
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
    fn prev_grapheme_offset(&self) -> Option<usize>;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn next_grapheme_offset(&self) -> Option<usize>;

    fn nth_next_grapheme_boundary(&self, n: usize) -> Result<usize, TextBufferCursorError>;

    fn nth_prev_grapheme_boundary(&self, n: usize) -> Result<usize, TextBufferCursorError>;

    /// Check if cursor position is at a codepoint boundary.
    fn is_grapheme_boundary(&self) -> bool;

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

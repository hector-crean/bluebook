/// A cursor with convenience functions for moving through a TextBuffer.

pub trait TextBufferCursor<'cursor> {
    /// Set cursor position.
    fn set(self, byte_idx: usize) -> Self;

    /// Get cursor position.
    fn pos(&self) -> usize;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn prev_grapheme_offset(&self) -> Option<usize>;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn next_grapheme_offset(&self) -> Option<usize>;

    fn nth_next_grapheme_boundary(&self, n: usize) -> Option<usize>;

    fn nth_prev_grapheme_boundary(&self, n: usize) -> Option<usize>;

    /// Check if cursor position is at a codepoint boundary.
    fn is_grapheme_boundary(&self) -> bool;

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

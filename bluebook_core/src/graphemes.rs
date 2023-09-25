use unicode_segmentation::GraphemeIncomplete;

use crate::{cursor::CursorRange, text_buffer::TextBuffer};

#[derive(thiserror::Error, Debug)]
pub enum GraphemeClusterCursorError {
    #[error("Invalid character encountered")]
    InvalidCharacter,
    // Add more error variants as needed
}

struct GraphemeClusterItem<'s> {
    cluster: &'s str,
    byte_offset: usize,
    index: usize,
}

pub trait GraphemeClusterCursor<'buffer> {
    type Buffer: TextBuffer;
    fn new(text: &'buffer Self::Buffer, range: CursorRange) -> Self;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn prev_grapheme_cluster_boundary(&self) -> Result<Option<usize>, GraphemeClusterCursorError>;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn next_grapheme_cluster_boundary(&self) -> Result<Option<usize>, GraphemeClusterCursorError>;

    // fn peek_prev_grapheme(&self) -> Option<&str>;

    // fn peek_next_grapheme(&self) -> Option<&str>;

    fn nth_next_grapheme_cluster_boundary(
        &self,
        n: usize,
    ) -> Result<Option<usize>, GraphemeClusterCursorError>;

    fn nth_prev_grapheme_cluster_boundary(
        &self,
        n: usize,
    ) -> Result<Option<usize>, GraphemeClusterCursorError>;

    /// Check if cursor position is at a codepoint boundary.
    fn is_grapheme_boundary(&self) -> Result<bool, GraphemeClusterCursorError>;
}

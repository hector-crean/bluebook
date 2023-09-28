use xi_rope::{Cursor, RopeInfo};

use crate::{block::BlockCursor, word::WordCursor};

use super::buffer::RopeBuffer;

/// A cursor providing utility function to navigate the rope
/// by word boundaries.
/// Boundaries can be the start of a word, its end, punctuation etc.
pub struct RopeBlockCursor<'a> {
    pub(crate) cursor: Cursor<'a, RopeInfo>,
}

impl Iterator for RopeBlockCursor<'_> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl DoubleEndedIterator for RopeBlockCursor<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<'buffer> BlockCursor<'buffer> for RopeBlockCursor<'buffer> {
    type Buffer = RopeBuffer;

    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self {
        let cursor = Cursor::new(text, pos);
        RopeBlockCursor { cursor }
    }
    fn offset(&self) -> usize {
        self.cursor.pos()
    }
}

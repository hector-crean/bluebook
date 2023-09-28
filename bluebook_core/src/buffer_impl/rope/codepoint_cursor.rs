use crate::{codepoint::CodepointCursor, graphemes::GraphemeCursor};
use xi_rope::rope::BaseMetric;
use xi_rope::{Cursor, RopeInfo};

use super::buffer::RopeBuffer;

pub struct RopeCodepointCursor<'a> {
    pub(crate) cursor: Cursor<'a, RopeInfo>,
}

impl<'a> From<Cursor<'a, RopeInfo>> for RopeCodepointCursor<'a> {
    fn from(cursor: Cursor<'a, RopeInfo>) -> Self {
        RopeCodepointCursor { cursor }
    }
}

// Implement the DoubleEndedIterator trait for RopeGraphemeCursor

impl<'a> Iterator for RopeCodepointCursor<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.cursor.next::<BaseMetric>()
    }
}

impl<'a> DoubleEndedIterator for RopeCodepointCursor<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.cursor.prev::<BaseMetric>()
    }
}

impl<'buffer> CodepointCursor<'buffer> for RopeCodepointCursor<'buffer> {
    type Buffer = RopeBuffer;

    fn new(text: &'buffer Self::Buffer, offset: usize) -> Self {
        let cursor = Cursor::new(text, offset);
        RopeCodepointCursor { cursor }
    }
    fn offset(&self) -> usize {
        self.cursor.pos()
    }
}

use xi_rope::{Cursor, RopeInfo};

use crate::{paragraph::ParagraphCursor, word::WordCursor};

use super::buffer::RopeBuffer;

pub struct RopeParagraphCursor<'a> {
    pub(crate) cursor: Cursor<'a, RopeInfo>,
}

impl<'a> From<Cursor<'a, RopeInfo>> for RopeParagraphCursor<'a> {
    fn from(cursor: Cursor<'a, RopeInfo>) -> Self {
        Self { cursor }
    }
}

impl Iterator for RopeParagraphCursor<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl DoubleEndedIterator for RopeParagraphCursor<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<'buffer> ParagraphCursor<'buffer> for RopeParagraphCursor<'buffer> {
    type Buffer = RopeBuffer;

    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self {
        let cursor = Cursor::new(text, pos);
        RopeParagraphCursor { cursor }
    }
    fn offset(&self) -> usize {
        self.cursor.pos()
    }
}

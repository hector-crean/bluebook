use xi_rope::{Cursor, LinesMetric, RopeInfo};

use crate::line::LineCursor;

use super::buffer::RopeBuffer;

pub struct RopeLineCursor<'a> {
    pub(crate) cursor: Cursor<'a, RopeInfo>,
}

impl<'a> From<Cursor<'a, RopeInfo>> for RopeLineCursor<'a> {
    fn from(cursor: Cursor<'a, RopeInfo>) -> Self {
        Self { cursor }
    }
}

impl<'a> Iterator for RopeLineCursor<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let offset = self.cursor.next::<LinesMetric>();
        offset
    }
}

impl<'a> DoubleEndedIterator for RopeLineCursor<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let offset = self.cursor.prev::<LinesMetric>();
        offset
    }
}

impl<'buffer> LineCursor<'buffer> for RopeLineCursor<'buffer> {
    type Buffer = RopeBuffer;

    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self {
        let cursor = Cursor::new(text, pos);
        Self { cursor }
    }
    fn offset(&self) -> usize {
        self.cursor.pos()
    }
}

use xi_rope::{Cursor, Rope, RopeInfo};

use crate::{
    codepoint::CharClassification,
    mode::Mode,
    sentence::SentenceCursor,
    word::{WordBoundary, WordCursor},
};

use super::buffer::RopeBuffer;

/// A cursor providing utility function to navigate the rope
/// by word boundaries.
/// Boundaries can be the start of a word, its end, punctuation etc.
pub struct RopeSentenceCursor<'a> {
    pub(crate) cursor: Cursor<'a, RopeInfo>,
}

impl<'a> From<Cursor<'a, RopeInfo>> for RopeSentenceCursor<'a> {
    fn from(cursor: Cursor<'a, RopeInfo>) -> Self {
        Self { cursor }
    }
}

impl Iterator for RopeSentenceCursor<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl DoubleEndedIterator for RopeSentenceCursor<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        None
    }
}

impl<'buffer> SentenceCursor<'buffer> for RopeSentenceCursor<'buffer> {
    type Buffer = RopeBuffer;

    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self {
        let cursor = Cursor::new(text, pos);
        RopeSentenceCursor { cursor }
    }
    fn offset(&self) -> usize {
        self.cursor.pos()
    }
}

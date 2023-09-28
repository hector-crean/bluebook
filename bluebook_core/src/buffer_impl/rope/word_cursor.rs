use xi_rope::{Cursor, Rope, RopeInfo};

use crate::{
    codepoint::CharClassification,
    mode::Mode,
    word::{WordBoundary, WordCursor},
};

use super::buffer::RopeBuffer;

/// A cursor providing utility function to navigate the rope
/// by word boundaries.
/// Boundaries can be the start of a word, its end, punctuation etc.
pub struct RopeWordCursor<'a> {
    pub(crate) cursor: Cursor<'a, RopeInfo>,
}

impl<'a> From<Cursor<'a, RopeInfo>> for RopeWordCursor<'a> {
    fn from(cursor: Cursor<'a, RopeInfo>) -> Self {
        Self { cursor }
    }
}

impl Iterator for RopeWordCursor<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.cursor.next_codepoint() {
            let mut prop = CharClassification::new(ch);
            let mut candidate = self.cursor.pos();
            while let Some(next) = self.cursor.next_codepoint() {
                let prop_next = CharClassification::new(next);
                if WordBoundary::new(prop, prop_next).is_start() {
                    break;
                }
                prop = prop_next;
                candidate = self.cursor.pos();
            }
            self.cursor.set(candidate);
            return Some(candidate);
        }
        None
    }
}

impl DoubleEndedIterator for RopeWordCursor<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let Some(ch) = self.cursor.prev_codepoint() {
            let mut prop = CharClassification::new(ch);
            let mut candidate = self.cursor.pos();
            while let Some(prev) = self.cursor.prev_codepoint() {
                let prop_prev = CharClassification::new(prev);
                if WordBoundary::new(prop_prev, prop).is_start() {
                    break;
                }

                prop = prop_prev;
                candidate = self.cursor.pos();
            }
            self.cursor.set(candidate);
            return Some(candidate);
        }
        None
    }
}

impl<'buffer> WordCursor<'buffer> for RopeWordCursor<'buffer> {
    type Buffer = RopeBuffer;

    fn new(buffer: &'buffer Self::Buffer, pos: usize) -> Self {
        let cursor = Cursor::new(&buffer.text, pos);
        RopeWordCursor { cursor }
    }
    fn offset(&self) -> usize {
        self.cursor.pos()
    }

    fn select_word(&mut self) -> (usize, usize) {
        let initial = self.cursor.pos();

        let start = match self.next_back() {
            Some(position) => position,
            None => initial, // Set to initial if there's no previous word
        };

        self.cursor.set(initial);

        let end = match self.next() {
            Some(position) => position,
            None => initial, // Set to initial if there's no next word
        };

        (start, end)
    }
}

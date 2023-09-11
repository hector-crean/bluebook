use std::borrow::Cow;

use super::grapheme::Graphemes;
use crate::{text_buffer::TextBuffer, text_buffer_cursor::TextBufferCursor};
use peritext::rich_text::cursor::Cursor;

#[derive(Debug)]
pub struct PeritextCursor<'a> {
    pub slice: Cow<'a, str>,
    pub position: usize,
}

// #[derive(Debug)]
// pub struct StringCursor<'a> {
//     text: &'a mut String,
//     position: usize,
// }

impl<'a> PeritextCursor<'a> {
    /// Create a new cursor.
    pub fn new(slice: Cow<'a, str>) -> Self {
        Self { slice, position: 0 }
    }
}

impl<'cursor> TextBufferCursor<'cursor> for PeritextCursor<'cursor> {
    fn set(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    fn pos(&self) -> usize {
        self.position
    }

    fn is_grapheme_boundary(&self) -> bool {
        let mut graphemes = Graphemes::new(&self.slice, false);

        graphemes.is_grapheme_boundary(self.position)
    }

    fn next_grapheme_offset(&self) -> Option<usize> {
        let mut graphemes = Graphemes::new(&self.slice, false);

        graphemes.next().map(|item| item.byte_offset)
    }

    fn prev_grapheme_offset(&self) -> Option<usize> {
        let mut graphemes = Graphemes::new(&self.slice, false);

        graphemes.next_back().map(|item| item.byte_offset)
    }

    fn nth_next_grapheme_boundary(&self, n: usize) -> Option<usize> {
        let mut graphemes = Graphemes::new(&self.slice, false);

        graphemes.nth(n).map(|item| item.byte_offset)
    }
    fn nth_prev_grapheme_boundary(&self, n: usize) -> Option<usize> {
        let mut graphemes = Graphemes::new(&self.slice, false);

        graphemes.nth_back(n).map(|item| item.byte_offset)
    }
}

pub fn len_utf8_from_first_byte(b: u8) -> usize {
    match b {
        b if b < 0x80 => 1,
        b if b < 0xe0 => 2,
        b if b < 0xf0 => 3,
        _ => 4,
    }
}

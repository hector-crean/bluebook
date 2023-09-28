use crate::buffer::TextBuffer;
use crate::graphemes::GraphemeCursor;
use xi_rope::{Cursor, RopeInfo};

use super::buffer::RopeBuffer;

pub struct RopeGraphemeCursor<'a> {
    pub(crate) cursor: Cursor<'a, RopeInfo>,
}

impl<'a> From<Cursor<'a, RopeInfo>> for RopeGraphemeCursor<'a> {
    fn from(cursor: Cursor<'a, RopeInfo>) -> Self {
        Self { cursor }
    }
}

// Implement the DoubleEndedIterator trait for RopeGraphemeCursor

impl<'a> Iterator for RopeGraphemeCursor<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        // Delegate the implementation to the existing next_grapheme_cluster_boundary method
        match self.cursor.next_grapheme() {
            Some(offset) => {
                self.cursor.set(offset);

                Some(offset)
            }
            _ => None,
        }
    }
}

impl<'a> DoubleEndedIterator for RopeGraphemeCursor<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // Delegate the implementation to the existing prev_grapheme_cluster_boundary method
        match self.cursor.prev_grapheme() {
            Some(offset) => {
                self.cursor.set(offset);

                Some(offset)
            }
            _ => None,
        }
    }
}

impl<'buffer> GraphemeCursor<'buffer> for RopeGraphemeCursor<'buffer> {
    type Buffer = RopeBuffer;

    fn new(text: &'buffer Self::Buffer, offset: usize) -> Self {
        let cursor = Cursor::new(text, offset);
        RopeGraphemeCursor { cursor }
    }
    fn offset(&self) -> usize {
        self.cursor.pos()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    Hello there!  How're you doing?  It's a fine day, \
    isn't it?  Aren't you glad we're alive?\r\n\
    こんにちは！元気ですか？日はいいですね。\
    私たちが生きだって嬉しいではないか？\r\n\
    ";

    #[test]
    fn nth_grapheme() {
        let buffer = RopeBuffer::from_str(TEXT);

        let mut gc = RopeGraphemeCursor::new(&buffer, 0);

        let offset = gc.nth(5);
    }
}

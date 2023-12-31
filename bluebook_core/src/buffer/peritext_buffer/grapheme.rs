//! This example shows how to implement a grapeheme iterator over the contents
//! of a `Rope` or `RopeSlice`.  This also serves as a good starting point for
//! iterators for other kinds of segementation, such as word boundaries.

#![allow(clippy::redundant_field_names)]
#![allow(dead_code)]

// use std::str::pattern::Pattern;
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete};

#[derive(Clone, Debug)]
pub struct Graphemes<'a> {
    slice: &'a str,
    gc: GraphemeCursor,
}

impl<'a> Graphemes<'a> {
    pub fn new(s: &str, is_extended: bool) -> Graphemes<'_> {
        let len = s.len();
        Graphemes {
            slice: s,
            gc: GraphemeCursor::new(0, len, is_extended),
        }
    }
    pub fn set_cursor_offet(mut self, byte_offset: usize) -> Self {
        self.gc.set_cursor(byte_offset);
        self
    }
    pub fn is_grapheme_boundary(mut self, byte_offset: usize) -> bool {
        loop {
            match self.gc.is_boundary(self.slice, byte_offset) {
                Ok(n) => return n,
                Err(GraphemeIncomplete::PreContext(_n)) => {
                    // let (ctx_chunk, ctx_byte_start, _, _) = self.slice.chunk_at_byte(n - 1);
                    // self.gc.provide_context(ctx_chunk, ctx_byte_start);
                }
                Err(_) => unreachable!(),
            }
        }
    }
}

pub struct GraphemeIterItem {
    pub byte_offset: usize,
}
impl GraphemeIterItem {
    fn new(byte_offset: usize) -> Self {
        Self { byte_offset }
    }
}
impl<'a> Iterator for Graphemes<'a> {
    type Item = GraphemeIterItem;

    fn next(&mut self) -> Option<Self::Item> {
        let next_idx = self.gc.next_boundary(self.slice, 0).unwrap();

        next_idx.map(GraphemeIterItem::new)
    }
}

impl<'a> DoubleEndedIterator for Graphemes<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let prev_idx = self.gc.prev_boundary(self.slice, 0).unwrap();

        prev_idx.map(GraphemeIterItem::new)
    }
}

pub fn nth_next_grapheme_boundary(
    slice: &str,
    byte_offset: usize,
    n: usize,
) -> Option<GraphemeIterItem> {
    let mut graphemes = Graphemes::new(slice, false).set_cursor_offet(byte_offset);

    graphemes.nth(n)
}

pub fn nth_prev_grapheme_boundary(
    slice: &str,
    byte_offset: usize,
    n: usize,
) -> Option<GraphemeIterItem> {
    let mut graphemes = Graphemes::new(slice, false).set_cursor_offet(byte_offset);
    graphemes.nth_back(n)
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
        let mut graphemes = Graphemes::new(TEXT, false).set_cursor_offet(TEXT.len());

        assert_eq!(graphemes.nth_back(1).unwrap().byte_offset, TEXT.len() - 2);
    }
}

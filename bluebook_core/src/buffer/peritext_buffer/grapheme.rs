//! This example shows how to implement a grapeheme iterator over the contents
//! of a `Rope` or `RopeSlice`.  This also serves as a good starting point for
//! iterators for other kinds of segementation, such as word boundaries.

#![allow(clippy::redundant_field_names)]
#![allow(dead_code)]

// use std::str::pattern::Pattern;
use core::cmp;
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
    pub fn set_cursor_offet(mut self, byte_idx: usize) -> Self {
        self.gc.set_cursor(byte_idx);
        self
    }
    pub fn is_grapheme_boundary(mut self, byte_idx: usize) -> bool {
        loop {
            match self.gc.is_boundary(self.slice, byte_idx) {
                Ok(n) => return n,
                Err(GraphemeIncomplete::PreContext(n)) => {
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
        let next_idx = self.gc.next_boundary(self.slice, 0).unwrap().unwrap();
        Some(GraphemeIterItem::new(next_idx))
    }
}

impl<'a> DoubleEndedIterator for Graphemes<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let prev_idx = self.gc.prev_boundary(self.slice, 0).unwrap().unwrap();
        Some(GraphemeIterItem::new(prev_idx))
    }
}

pub fn nth_next_grapheme_boundary<'a>(
    slice: &'a str,
    byte_idx: usize,
    n: usize,
) -> Option<GraphemeIterItem> {
    let mut graphemes = Graphemes::new(slice, false).set_cursor_offet(byte_idx);
    graphemes.nth(n)
}

pub fn nth_prev_grapheme_boundary<'a>(
    slice: &'a str,
    byte_idx: usize,
    n: usize,
) -> Option<GraphemeIterItem> {
    let mut graphemes = Graphemes::new(slice, false).set_cursor_offet(byte_idx);
    graphemes.nth_back(n)
}

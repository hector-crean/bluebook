use super::{
    drain::RopeDrain,
    grapheme::{self, RopeGraphemes},
};
use crate::text_buffer::TextBuffer;

use ropey::{
    iter::{Bytes, Chars, Chunks, Lines},
    Rope, RopeSlice,
};
use std::{
    borrow::Cow,
    fs::File,
    io,
    ops::{Deref, DerefMut, Range},
};

use super::cursor_impl::RopeCursor;

//https://developer.wordpress.org/block-editor/reference-guides/packages/packages-rich-text/

// impl TextBuffer for Rope {
//     type Cursor<'cursor> = RopeCursor<'cursor>;
//     type DrainItem = char;

//     fn cursor<'cursor>(&'cursor self, position: usize) -> Option<Self::Cursor<'cursor>> {
//         if self.is_char_boundary(position) {
//             Some(RopeCursor {
//                 rope: self,
//                 cursor: position,
//             })
//         } else {
//             None
//         }
//     }

//     fn write(&self, offset: usize, s: &str) -> Result<(), crate::text_buffer::TextBufferError> {
//         todo!()
//     }

//     fn drain<R>(&mut self, range: R) -> Self::Drain
//     where
//         R: std::ops::RangeBounds<usize>,
//     {
//         todo!()
//     }

//     fn replace_range<R>(&mut self, range: R, replace_with: &str)
//     where
//         R: std::ops::RangeBounds<usize>,
//     {
//         todo!()
//     }

//     fn slice(&self, range: Range<usize>) -> Option<Cow<'_, str>> {
//         Some(self.slice(range).into())
//     }
//     fn len(&self) -> usize {
//         self.len()
//     }

//     fn from_str(s: &str) -> Self {
//         Rope::from_str(s)
//     }
//     fn prev_grapheme_offset(&self, offset: usize) -> Option<usize> {
//         grapheme::prev_grapheme_boundary(&self.slice(..), offset)
//     }
//     fn next_grapheme_offset(&self, offset: usize) -> Option<usize> {
//         grapheme::next_grapheme_boundary(&self.slice(..), offset)
//     }
//     fn prev_codepoint_offset(&self, offset: usize) -> Option<usize> {
//         todo!()
//     }
//     fn next_codepoint_offset(&self, offset: usize) -> Option<usize> {
//         todo!()
//     }
//     fn prev_word_offset(&self, offset: usize) -> Option<usize> {
//         todo!()
//     }
//     fn next_word_offset(&self, offset: usize) -> Option<usize> {
//         todo!()
//     }

//     fn preceding_line_break(&self, offset: usize) -> usize {
//         todo!()
//     }
//     fn next_line_break(&self, offset: usize) -> usize {
//         todo!()
//     }
//     fn is_empty(&self) -> bool {
//         self.to_string().is_empty()
//     }
// }

use super::text_buffer_cursor::TextBufferCursor;

use std::{
    borrow::Cow,
    default::Default,
    error::Error,
    fs::File,
    io,
    ops::{Deref, DerefMut, Range, RangeBounds},
    slice::Iter,
};

#[derive(thiserror::Error, Debug)]
pub enum TextBufferError {
    #[error("buffer could not be flushed")]
    FlushError,
    #[error("write error: {content:?}")]
    WriteError { content: String },
}

/**
 *
 * The Drain struct holds a mutable reference to the TextBuffer, ensuring that the text buffer cannot be directly accessed or modified while the Drain instance exists.
The drain function on TextBuffer returns a Drain instance.
The lifetime 'a in the Drain struct guarantees that the TextBuffer's data remains valid for the duration of the drain.
This design effectively locks the TextBuffer during the draining process. Once the Drain iterator is dropped or goes out of scope, the TextBuffer is accessible again. This approach provides safety while allowing for more flexibility in how the drain iterator can be used.
 */

// pub struct Drain<'a, T: TextBuffer + ?Sized> {
//     pub text_buffer: &'a mut T,
//     pub range: Range<usize>,
// }

// impl<'a, T: TextBuffer> Iterator for Drain<'a, T> {
//     type Item = T::DrainItem;

//     fn next(&mut self) -> Option<Self::Item> {

//         // Logic to drain from text_buffer within the range.
//         // This would adjust the underlying TextBuffer's data.
//     }
// }

pub trait TextBuffer: Default {
    //The where Self: 'cursor clause is crucial. It ensures that any reference held by the Cursor type must outlive the 'cursor lifetime.
    //Indicates that the Cursor cannot outlive the TextBuffer it is derived from.
    type Cursor<'cursor>: TextBufferCursor<'cursor>
    where
        Self: 'cursor;

    /// Create a cursor with a reference to the text and a offset position.
    ///
    /// Returns None if the position isn't a codepoint boundary.
    fn cursor<'cursor>(&'cursor self, position: usize) -> Option<Self::Cursor<'cursor>>;
    // ^ should I specify cursors?

    fn write(&mut self, offset: usize, s: &str) -> Result<(), TextBufferError>;

    fn replace_range<R>(&mut self, range: R, replace_with: &str)
    where
        R: RangeBounds<usize>;

    // fn flush(&mut self) -> Result<(), TextBufferError>;

    fn take(&self) -> Cow<str>;

    /// Get slice of text at range.
    fn slice(&self, range: Range<usize>) -> Option<Cow<str>>;

    /// Get length of text (in bytes).
    fn len(&self) -> usize;

    /// Get the previous word offset from the given offset, if it exists.
    fn prev_word_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next word offset from the given offset, if it exists.
    fn next_word_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn prev_grapheme_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next grapheme offset from the given offset, if it exists.
    fn next_grapheme_offset(&self, offset: usize) -> Option<usize>;

    /// Get the previous codepoint offset from the given offset, if it exists.
    fn prev_codepoint_offset(&self, offset: usize) -> Option<usize>;

    /// Get the next codepoint offset from the given offset, if it exists.
    fn next_codepoint_offset(&self, offset: usize) -> Option<usize>;

    /// Get the preceding line break offset from the given offset
    fn preceding_line_break(&self, offset: usize) -> usize;

    /// Get the next line break offset from the given offset
    fn next_line_break(&self, offset: usize) -> usize;

    /// Returns `true` if this text has 0 length.
    fn is_empty(&self) -> bool;

    /// Construct an instance of this type from a `&str`.
    fn from_str(s: &str) -> Self;
}

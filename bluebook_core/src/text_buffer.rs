use unicode_segmentation::GraphemeIncomplete;

use crate::{
    buffer::peritext_buffer::cursor_impl::CursorRange, error::TextBufferWithCursorError,
    graphemes::UnicodeSegmentationError, span::Span, text_buffer_cursor::CursorDocCoords,
};

use super::text_buffer_cursor::TextBufferCursor;

use std::{
    borrow::Cow,
    fmt::Debug,
    ops::{Range, RangeBounds},
};

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

pub trait TextBuffer {
    //The where Self: 'cursor clause is crucial. It ensures that any reference held by the Cursor type must outlive the 'cursor lifetime.
    //Indicates that the Cursor cannot outlive the TextBuffer it is derived from.
    type Cursor<'cursor>: TextBufferCursor<'cursor>
    where
        Self: 'cursor;

    type SpanItem: Into<Span>;
    type SpanIter<'spans>: Iterator<Item = Self::SpanItem>
    where
        Self: 'spans;

    fn span_iter<'spans, 'buffer: 'spans>(&'buffer self) -> Self::SpanIter<'spans>;

    fn annotate<R>(&mut self, range: R, annotation: peritext::Style)
    where
        R: RangeBounds<usize>;

    /// Create a cursor with a reference to the text and a offset position.
    ///
    /// Returns None if the position isn't a codepoint boundary.
    fn cursor(&mut self, range: CursorRange)
        -> Result<Self::Cursor<'_>, TextBufferWithCursorError>;
    // ^ should I specify cursors?

    fn cursor_coords(
        &mut self,
        cursor_range: CursorRange,
    ) -> Result<CursorDocCoords, TextBufferWithCursorError>;

    fn write(&mut self, offset: usize, s: &str) -> Result<usize, TextBufferWithCursorError>;

    fn drain<R>(&mut self, range: R) -> Result<Cow<str>, TextBufferWithCursorError>
    where
        R: RangeBounds<usize>;

    fn replace_range<R>(
        &mut self,
        range: R,
        replace_with: &str,
    ) -> Result<Range<usize>, TextBufferWithCursorError>
    where
        R: RangeBounds<usize>;

    // fn flush(&mut self) -> Result<(), TextBufferError>;

    fn take(&self) -> Cow<str>;

    /// Get slice of text at range.
    fn slice(&self, range: Range<usize>) -> Result<Cow<str>, TextBufferWithCursorError>;

    /// Get length of text (in bytes).
    fn len(&self) -> usize;

    /// Returns `true` if this text has 0 length.
    fn is_empty(&self) -> bool;

    /// Construct an instance of this type from a `&str`.
    fn from_str(s: &str) -> Self;
}

// fn drain(&mut self, range: R) -> Result<impl Iterator<Item = E> + '_, TextBufferCursorError>
// where
//     R: RangeBounds<usize>;

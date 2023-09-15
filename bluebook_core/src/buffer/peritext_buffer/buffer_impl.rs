use super::cursor_impl::{CursorRange, PeritextCursor};
use crate::text_buffer_cursor::{TextBufferCursor, TextBufferCursorError};
use crate::{
    span::Span,
    text_buffer::{TextBuffer, TextBufferError},
};
use std::{
    borrow::Cow,
    ops::{Range, RangeBounds},
};

use peritext::rich_text::{self, IndexType, RichText as RichTextInner};

impl From<peritext::rich_text::Span> for Span {
    fn from(val: peritext::rich_text::Span) -> Self {
        Span {
            insert: val.insert,
            attributes: val.attributes,
        }
    }
}

pub struct Peritext {
    inner: RichTextInner,
}

impl Peritext {
    pub fn new(client_id: u64) -> Self {
        Self {
            inner: RichTextInner::new(client_id),
        }
    }
}

impl TextBuffer for Peritext {
    type Cursor<'cursor> = PeritextCursor<'cursor> where Self:'cursor;
    type SpanItem = rich_text::Span;
    type SpanIter<'spans> = rich_text::iter::Iter<'spans> where Self: 'spans;

    fn cursor(&self, cursor_range: CursorRange) -> Result<Self::Cursor<'_>, TextBufferCursorError> {
        let new_cursor = PeritextCursor {
            buffer: self.inner.to_string().into(),
            cursor_range,
        };

        if new_cursor.is_grapheme_boundary() {
            Ok(new_cursor)
        } else {
            Err(TextBufferCursorError::CodepointBoundaryError {
                byte_offset: cursor_range.head,
            })
        }
    }

    // pub fn iter(&self) -> impl Iterator<Item = Span> + '_ {
    fn annotate<R>(&mut self, range: R, annotation: peritext::Style)
    where
        R: RangeBounds<usize>,
    {
        self.inner.annotate(range, annotation)
    }

    fn span_iter<'spans, 'buffer: 'spans>(&'buffer self) -> Self::SpanIter<'spans> {
        rich_text::iter::Iter::new(&self.inner)
    }

    fn write<'a>(&mut self, offset: usize, s: &'a str) -> Result<usize, TextBufferError> {
        self.inner.insert(offset, s);

        Ok(offset + s.len())
    }

    fn drain<R>(&mut self, range: R) -> Result<&str, TextBufferError>
    where
        R: RangeBounds<usize>,
    {
        todo!()
    }

    fn replace_range<R>(
        &mut self,
        range: R,
        replace_with: &str,
    ) -> Result<Range<usize>, TextBufferError>
    where
        R: RangeBounds<usize>,
    {
        let start = match range.start_bound() {
            std::ops::Bound::Included(&start) => start,
            std::ops::Bound::Excluded(&start) => start + 1,
            std::ops::Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            std::ops::Bound::Included(&end) => end + 1,
            std::ops::Bound::Excluded(&end) => end,
            std::ops::Bound::Unbounded => self.inner.len(), // Assuming inner is a collection with a len() method
        };

        self.inner.delete(start..end);
        self.inner.insert(start, replace_with);

        Ok(Range {
            start,
            end: start + replace_with.len(),
        })
    }
    // fn edit(&mut self, range: Range<usize>, new: impl Into<String>) {
    //     self.replace_range(range, &new.into());
    // }

    fn take(&self) -> Cow<str> {
        self.inner.to_string().into()
    }

    fn slice(&self, range: Range<usize>) -> Option<Cow<str>> {
        let str = self.inner.slice_str(range, IndexType::Utf16);

        Some(str.into())
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn from_str(_s: &str) -> Self {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn replace() {
        let mut buf = String::from("hello world");

        buf.replace_range(1..9, "era");

        assert_eq!("herald", buf);
    }
}

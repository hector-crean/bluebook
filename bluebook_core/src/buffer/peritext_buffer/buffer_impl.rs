use super::cursor_impl::{CursorRange, PeritextCursor};
use crate::error::TextBufferWithCursorError;
use crate::text_buffer_cursor::{CursorDocCoords, TextBufferCursor};
use crate::{span::Span, text_buffer::TextBuffer};
use std::ops::Bound;
use std::{
    borrow::Cow,
    ops::{Range, RangeBounds},
};

use peritext::rich_text::{self, IndexType, RichText as RichTextInner};

use unicode_segmentation::UnicodeSegmentation;

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

    fn convert_range<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
        let start = match range.start_bound() {
            Bound::Included(&start) => start,
            Bound::Excluded(&start) => start + 1,
            Bound::Unbounded => 0,
        };

        let end = match range.end_bound() {
            Bound::Included(&end) => end + 1,
            Bound::Excluded(&end) => end,
            Bound::Unbounded => self.inner.len(),
        };

        (start, end)
    }
}

impl TextBuffer for Peritext {
    type Cursor<'cursor> = PeritextCursor<'cursor> where Self:'cursor;
    type SpanItem = rich_text::Span;
    type SpanIter<'spans> = rich_text::iter::Iter<'spans> where Self: 'spans;

    fn cursor(
        &mut self,
        cursor_range: CursorRange,
    ) -> Result<Self::Cursor<'_>, TextBufferWithCursorError> {
        let new_cursor = PeritextCursor {
            buffer: self.inner.to_string().into(),
            cursor_range,
        };

        let is_boundary = new_cursor.is_grapheme_boundary()?;
        Ok(new_cursor)
    }

    /**
     * Our cursor
     */

    fn cursor_coords(
        &mut self,
        cursor_range: CursorRange,
    ) -> Result<CursorDocCoords, TextBufferWithCursorError> {
        let buf = &self.inner.slice_str(0..cursor_range.head, IndexType::Utf8);

        let mut row = buf.chars().filter(|&c| c == '\n').count();
        let mut col: usize = 0;

        fn find_last_newline_position(s: &str) -> Option<usize> {
            for (i, c) in s.char_indices().rev() {
                if c == '\n' {
                    return Some(i);
                }
            }
            None
        }

        let newline_idx = find_last_newline_position(&buf);

        match newline_idx {
            Some(newline_idx) => {
                let g = UnicodeSegmentation::graphemes(&buf[newline_idx..cursor_range.head], true);

                col = g.collect::<Vec<&str>>().len();
            }
            None => {}
        }

        tracing::info!("{:?}", buf);

        // // If the cursor's position exceeds the buffer, return an error.
        // if cursor_range.head > buf.len() {
        //     return Err(TextBufferWithCursorError::OutOfBounds); // Assuming you have such an error variant
        // }

        // let mut line_start_byte_offset: usize = 0;
        // let mut row: usize = 0;
        // let mut col: usize = 0;

        // for (line_idx, line) in buf.lines().enumerate() {
        //     let newline_char = '\n'.to_string();
        //     tracing::info!(
        //         "{:?} <  {:?} < {:?} ",
        //         line_start_byte_offset,
        //         cursor_range.head,
        //         line_start_byte_offset + line.len() + line_idx * &newline_char.len()
        //     );

        //     if line_start_byte_offset <= cursor_range.head
        //         && cursor_range.head
        //             <= line_start_byte_offset + line.len() + line_idx * &newline_char.len()
        //     {
        //         let g = UnicodeSegmentation::graphemes(
        //             &buf[line_start_byte_offset..cursor_range.head],
        //             true,
        //         );

        //         col = g.collect::<Vec<&str>>().len();
        //     }
        //     row = line_idx;
        //     line_start_byte_offset += line.len();
        // }

        if buf.ends_with("\n") {
            row += 1;
        }

        let coords = CursorDocCoords::new(row, col);
        tracing::info!("coords: {:?}", coords);

        Ok(coords)
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

    fn write(&mut self, offset: usize, s: &str) -> Result<usize, TextBufferWithCursorError> {
        self.inner.insert(offset, s);

        Ok(offset + s.len())
    }

    fn drain<R>(&mut self, range: R) -> Result<Cow<str>, TextBufferWithCursorError>
    where
        R: RangeBounds<usize>,
    {
        let (start, end) = self.convert_range(range);

        self.inner.delete(start..end);

        let slice = self.slice(start..end);

        slice
    }

    fn replace_range<R>(
        &mut self,
        range: R,
        replace_with: &str,
    ) -> Result<Range<usize>, TextBufferWithCursorError>
    where
        R: RangeBounds<usize>,
    {
        let (start, end) = self.convert_range(range);

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

    fn slice(&self, range: Range<usize>) -> Result<Cow<str>, TextBufferWithCursorError> {
        let str = self.inner.slice_str(range, IndexType::Utf16);

        Ok(str.into())
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

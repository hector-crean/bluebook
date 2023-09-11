use super::cursor_impl::PeritextCursor;
use crate::text_buffer_cursor::TextBufferCursor;
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

    fn cursor(&self, _position: usize) -> Option<Self::Cursor<'_>> {
        let new_cursor = PeritextCursor {
            text: self.inner.to_string().into(),
            position: _position,
        };

        if new_cursor.is_boundary() {
            Some(new_cursor)
        } else {
            None
        }
    }

    fn write(&mut self, offset: usize, s: &str) -> Result<(), TextBufferError> {
        self.inner.insert(offset, s);

        Ok(())
    }

    fn replace_range<R>(&mut self, range: R, replace_with: &str)
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

    fn prev_grapheme_offset(&self, _from: usize) -> Option<usize> {
        todo!()
    }

    fn next_grapheme_offset(&self, _from: usize) -> Option<usize> {
        todo!()
    }

    fn prev_codepoint_offset(&self, _from: usize) -> Option<usize> {
        todo!()
    }

    fn next_codepoint_offset(&self, _from: usize) -> Option<usize> {
        todo!()
    }

    fn prev_word_offset(&self, _from: usize) -> Option<usize> {
        todo!()
    }

    fn next_word_offset(&self, _from: usize) -> Option<usize> {
        todo!()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn from_str(_s: &str) -> Self {
        todo!()
    }

    fn preceding_line_break(&self, _from: usize) -> usize {
        todo!()
    }

    fn next_line_break(&self, _from: usize) -> usize {
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

    #[test]
    fn prev_codepoint_offset() {
        let a = String::from("a\u{00A1}\u{4E00}\u{1F4A9}");
        assert_eq!(Some(6), a.prev_codepoint_offset(10));
        assert_eq!(Some(3), a.prev_codepoint_offset(6));
        assert_eq!(Some(1), a.prev_codepoint_offset(3));
        assert_eq!(Some(0), a.prev_codepoint_offset(1));
        assert_eq!(None, a.prev_codepoint_offset(0));
        let b = a.slice(1..10).unwrap().to_string();
        assert_eq!(Some(5), b.prev_codepoint_offset(9));
        assert_eq!(Some(2), b.prev_codepoint_offset(5));
        assert_eq!(Some(0), b.prev_codepoint_offset(2));
        assert_eq!(None, b.prev_codepoint_offset(0));
    }

    #[test]
    fn next_codepoint_offset() {
        let a = String::from("a\u{00A1}\u{4E00}\u{1F4A9}");
        assert_eq!(Some(10), a.next_codepoint_offset(6));
        assert_eq!(Some(6), a.next_codepoint_offset(3));
        assert_eq!(Some(3), a.next_codepoint_offset(1));
        assert_eq!(Some(1), a.next_codepoint_offset(0));
        assert_eq!(None, a.next_codepoint_offset(10));
        let b = a.slice(1..10).unwrap().to_string();
        assert_eq!(Some(9), b.next_codepoint_offset(5));
        assert_eq!(Some(5), b.next_codepoint_offset(2));
        assert_eq!(Some(2), b.next_codepoint_offset(0));
        assert_eq!(None, b.next_codepoint_offset(9));
    }

    #[test]
    fn prev_next() {
        let buf = String::from("abc");
        let mut cursor = buf.cursor(0).unwrap();

        assert_eq!(cursor.next(), Some(0));
        assert_eq!(cursor.next(), Some(1));
        assert_eq!(cursor.prev(), Some(1));
        assert_eq!(cursor.next(), Some(1));
        assert_eq!(cursor.next(), Some(2));
    }

    #[test]
    fn peek_next_codepoint() {
        let inp = String::from("$¢€£💶");
        let mut cursor = inp.cursor(0).unwrap();
        assert_eq!(cursor.peek_next_codepoint(), Some('$'));
        assert_eq!(cursor.peek_next_codepoint(), Some('$'));
        assert_eq!(cursor.next_codepoint(), Some('$'));
        assert_eq!(cursor.peek_next_codepoint(), Some('¢'));
        assert_eq!(cursor.prev_codepoint(), Some('$'));
        assert_eq!(cursor.peek_next_codepoint(), Some('$'));
        assert_eq!(cursor.next_codepoint(), Some('$'));
        assert_eq!(cursor.next_codepoint(), Some('¢'));
        assert_eq!(cursor.peek_next_codepoint(), Some('€'));
        assert_eq!(cursor.next_codepoint(), Some('€'));
        assert_eq!(cursor.peek_next_codepoint(), Some('£'));
        assert_eq!(cursor.next_codepoint(), Some('£'));
        assert_eq!(cursor.peek_next_codepoint(), Some('💶'));
        assert_eq!(cursor.next_codepoint(), Some('💶'));
        assert_eq!(cursor.peek_next_codepoint(), None);
        assert_eq!(cursor.next_codepoint(), None);
        assert_eq!(cursor.peek_next_codepoint(), None);
    }

    #[test]
    fn prev_grapheme_offset() {
        // A with ring, hangul, regional indicator "US"
        let a = String::from("A\u{030a}\u{110b}\u{1161}\u{1f1fa}\u{1f1f8}");
        assert_eq!(Some(9), a.prev_grapheme_offset(17));
        assert_eq!(Some(3), a.prev_grapheme_offset(9));
        assert_eq!(Some(0), a.prev_grapheme_offset(3));
        assert_eq!(None, a.prev_grapheme_offset(0));
    }

    #[test]
    fn next_grapheme_offset() {
        // A with ring, hangul, regional indicator "US"
        let a = String::from("A\u{030a}\u{110b}\u{1161}\u{1f1fa}\u{1f1f8}");
        assert_eq!(Some(3), a.next_grapheme_offset(0));
        assert_eq!(Some(9), a.next_grapheme_offset(3));
        assert_eq!(Some(17), a.next_grapheme_offset(9));
        assert_eq!(None, a.next_grapheme_offset(17));
    }

    #[test]
    fn prev_word_offset() {
        let a = String::from("Technically a word: ৬藏A\u{030a}\u{110b}\u{1161}");
        assert_eq!(Some(20), a.prev_word_offset(35));
        assert_eq!(Some(20), a.prev_word_offset(27));
        assert_eq!(Some(20), a.prev_word_offset(23));
        assert_eq!(Some(14), a.prev_word_offset(20));
        assert_eq!(Some(14), a.prev_word_offset(19));
        assert_eq!(Some(12), a.prev_word_offset(13));
        assert_eq!(None, a.prev_word_offset(12));
        assert_eq!(None, a.prev_word_offset(11));
        assert_eq!(None, a.prev_word_offset(0));
    }

    #[test]
    fn next_word_offset() {
        let a = String::from("Technically a word: ৬藏A\u{030a}\u{110b}\u{1161}");
        assert_eq!(Some(11), a.next_word_offset(0));
        assert_eq!(Some(11), a.next_word_offset(7));
        assert_eq!(Some(13), a.next_word_offset(11));
        assert_eq!(Some(18), a.next_word_offset(14));
        assert_eq!(Some(35), a.next_word_offset(18));
        assert_eq!(Some(35), a.next_word_offset(19));
        assert_eq!(Some(35), a.next_word_offset(20));
        assert_eq!(Some(35), a.next_word_offset(26));
        assert_eq!(Some(35), a.next_word_offset(35));
    }

    #[test]
    fn preceding_line_break() {
        let a = String::from("Technically\na word:\n ৬藏A\u{030a}\n\u{110b}\u{1161}");
        assert_eq!(0, a.preceding_line_break(0));
        assert_eq!(0, a.preceding_line_break(11));
        assert_eq!(12, a.preceding_line_break(12));
        assert_eq!(12, a.preceding_line_break(13));
        assert_eq!(20, a.preceding_line_break(21));
        assert_eq!(31, a.preceding_line_break(31));
        assert_eq!(31, a.preceding_line_break(34));

        let b = String::from("Technically a word: ৬藏A\u{030a}\u{110b}\u{1161}");
        assert_eq!(0, b.preceding_line_break(0));
        assert_eq!(0, b.preceding_line_break(11));
        assert_eq!(0, b.preceding_line_break(13));
        assert_eq!(0, b.preceding_line_break(21));
    }

    #[test]
    fn next_line_break() {
        let a = String::from("Technically\na word:\n ৬藏A\u{030a}\n\u{110b}\u{1161}");
        assert_eq!(11, a.next_line_break(0));
        assert_eq!(11, a.next_line_break(11));
        assert_eq!(19, a.next_line_break(13));
        assert_eq!(30, a.next_line_break(21));
        assert_eq!(a.len(), a.next_line_break(31));

        let b = String::from("Technically a word: ৬藏A\u{030a}\u{110b}\u{1161}");
        assert_eq!(b.len(), b.next_line_break(0));
        assert_eq!(b.len(), b.next_line_break(11));
        assert_eq!(b.len(), b.next_line_break(13));
        assert_eq!(b.len(), b.next_line_break(19));
    }
}

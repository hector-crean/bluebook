use std::ops::Range;
use std::ops::{Deref, DerefMut};

use xi_rope::{
    spans::{Span, SpanIter},
    Interval, Rope,
};

use crate::{
    block::BlockCursor, buffer::TextBuffer, codepoint::CodepointCursor, graphemes::GraphemeCursor,
    line::LineCursor, paragraph::ParagraphCursor, sentence::SentenceCursor, span::SpanData,
    word::WordCursor,
};

use super::{
    block_cursor::RopeBlockCursor, codepoint_cursor::RopeCodepointCursor,
    grapheme_cursor::RopeGraphemeCursor, line_cursor::RopeLineCursor,
    paragraph_cursor::RopeParagraphCursor, sentence_cursor::RopeSentenceCursor,
    word_cursor::RopeWordCursor,
};

pub struct RopeBuffer {
    pub inner: Rope,
}

impl RopeBuffer {
    pub fn new(s: &str) -> Self {
        Self {
            inner: Rope::from(s),
        }
    }
}

impl Deref for RopeBuffer {
    type Target = Rope;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl DerefMut for RopeBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl ToString for RopeBuffer {
    fn to_string(&self) -> String {
        self.inner.to_string()
    }
}

impl<'a, T: Clone> Into<crate::span::Span<T>> for (xi_rope::Interval, &'a T) {
    fn into(self) -> crate::span::Span<T> {
        let start = self.0.start();
        let end = self.0.end();

        crate::span::Span {
            range: Range { start, end },
            data: self.1.clone(),
        }
    }
}

impl TextBuffer for RopeBuffer {
    type CodepointCursor<'cursor> = RopeCodepointCursor<'cursor>;
    type GraphemeCursor<'cursor> = RopeGraphemeCursor<'cursor>;
    type WordCursor<'cursor> = RopeWordCursor<'cursor>;
    type SentenceCursor<'cursor> = RopeSentenceCursor<'cursor>;
    type ParagraphCursor<'cursor> = RopeParagraphCursor<'cursor>;
    type LineCursor<'cursor> = RopeLineCursor<'cursor>;
    type BlockCursor<'cursor> = RopeBlockCursor<'cursor>;

    type SpanIterItem<'span> = (xi_rope::Interval, &'span SpanData);
    type SpanIter<'span> = xi_rope::spans::SpanIter<'span, SpanData>;

    fn from_str(s: &str) -> Self {
        RopeBuffer {
            inner: Rope::from(s),
        }
    }

    fn slice(
        &self,
        range: std::ops::Range<usize>,
    ) -> Result<std::borrow::Cow<str>, crate::buffer::ConversionError> {
        todo!()
    }

    fn codepoint_cursor(
        &mut self,
        offset: usize,
    ) -> Result<Self::CodepointCursor<'_>, crate::codepoint::CodepointCursorError> {
        let cursor = Self::CodepointCursor::new(self, offset);
        Ok(cursor)
    }

    fn grapheme_cursor(
        &mut self,
        offset: usize,
    ) -> Result<Self::GraphemeCursor<'_>, crate::graphemes::GraphemeCursorError> {
        let cursor = Self::GraphemeCursor::new(self, offset);
        Ok(cursor)
    }

    fn word_cursor(
        &mut self,
        offset: usize,
    ) -> Result<Self::WordCursor<'_>, crate::word::WordCursorError> {
        let cursor = Self::WordCursor::new(self, offset);
        Ok(cursor)
    }

    fn sentence_cursor(
        &mut self,
        offset: usize,
    ) -> Result<Self::SentenceCursor<'_>, crate::sentence::SentenceCursorError> {
        let cursor = Self::SentenceCursor::new(self, offset);
        Ok(cursor)
    }

    fn paragraph_cursor(
        &mut self,
        offset: usize,
    ) -> Result<Self::ParagraphCursor<'_>, crate::paragraph::ParagraphCursorError> {
        let cursor = Self::ParagraphCursor::new(self, offset);
        Ok(cursor)
    }

    fn line_cursor(
        &mut self,
        offset: usize,
    ) -> Result<Self::LineCursor<'_>, crate::line::LineCursorError> {
        let cursor = Self::LineCursor::new(self, offset);
        Ok(cursor)
    }

    fn block_cursor(
        &mut self,
        offset: usize,
    ) -> Result<Self::BlockCursor<'_>, crate::block::BlockCursorError> {
        let cursor = Self::BlockCursor::new(self, offset);
        Ok(cursor)
    }

    fn write(
        &mut self,
        offset: usize,
        s: &str,
    ) -> Result<usize, crate::graphemes::GraphemeCursorError> {
        self.inner.edit(offset..offset, s);
        Ok(offset)
    }

    fn drain(&mut self, range: std::ops::Range<usize>) -> std::vec::Drain<&str> {
        todo!()
    }

    fn replace_range(
        &mut self,
        range: Range<usize>,
        replace_with: &str,
    ) -> Result<Range<usize>, crate::graphemes::GraphemeCursorError> {
        let iv = Interval::new(range.start, range.end);

        self.inner.edit(iv, replace_with);
        Ok(range)
    }

    fn span_iter<'spans, 'buffer: 'spans>(&'buffer self) -> Self::SpanIter<'spans> {
        todo!()
    }

    fn annotate(&mut self, range: Range<usize>, data: SpanData) {
        todo!()
    }

    fn take(&self) -> std::borrow::Cow<str> {
        self.inner.slice_to_cow(..)
    }

    fn len(&self) -> usize {
        self.inner.len()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn replace() {}
}

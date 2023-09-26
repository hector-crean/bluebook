use std::borrow::Cow;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use std::ops::{Range, RangeBounds};

use xi_rope::interval::IntervalBounds;
use xi_rope::rope::ChunkIter;
use xi_rope::LinesMetric;
use xi_rope::{
    spans::{Span, SpanIter},
    Interval, Rope,
};

use crate::codepoint::CharIndicesJoin;
use crate::encoding;
use crate::position::Position;
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

    fn char_indices_iter<'a, T: IntervalBounds>(
        &'a self,
        range: T,
    ) -> CharIndicesJoin<
        std::str::CharIndices<'a>,
        std::iter::Map<ChunkIter<'a>, fn(&str) -> std::str::CharIndices<'_>>,
    > {
        let iter: ChunkIter<'a> = self.iter_chunks(range);
        let iter: std::iter::Map<ChunkIter<'a>, fn(&str) -> std::str::CharIndices<'_>> =
            iter.map(str::char_indices);
        CharIndicesJoin::new(iter)
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

    fn slice(&self, range: std::ops::Range<usize>) -> Cow<str> {
        self.inner.slice_to_cow(range)
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
        self.inner.edit(offset..offset + s.len(), s);

        Ok(offset + s.len())
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

    /// Return the line number corresponding to the byte index `offset`.
    ///
    /// The line number is 0-based, thus this is equivalent to the count of newlines
    /// in the slice up to `offset`.
    ///
    /// Time complexity: O(log n)
    ///
    /// # Panics
    ///
    /// This function will panic if `offset > self.len()`. Callers are expected to
    /// validate their input.
    fn line_of_offset(&self, offset: usize) -> usize {
        self.count::<LinesMetric>(offset)
    }

    /// Return the byte offset corresponding to the line number `line`.
    /// If `line` is equal to one plus the current number of lines,
    /// this returns the offset of the end of the rope. Arguments higher
    /// than this will panic.
    ///
    /// The line number is 0-based.
    ///
    /// Time complexity: O(log n)
    ///
    /// # Panics
    ///
    /// This function will panic if `line > self.measure::<LinesMetric>() + 1`.
    /// Callers are expected to validate their input.
    fn offset_of_line(&self, line: usize) -> usize {
        let max_line = self.measure::<LinesMetric>() + 1;
        match line.cmp(&max_line) {
            Ordering::Greater => {
                panic!("line number {} beyond last line {}", line, max_line);
            }
            Ordering::Equal => {
                return self.len();
            }
            Ordering::Less => self.count_base_units::<LinesMetric>(line),
        }
    }

    /// Converts a UTF8 offset to a UTF16 LSP position
    /// Returns None if it is not a valid UTF16 offset
    fn offset_to_position(&self, offset: usize) -> Position {
        let (line, col) = self.offset_to_line_col(offset);
        let line_offset = self.offset_of_line(line);

        let utf16_col = encoding::offset_utf8_to_utf16(self.char_indices_iter(line_offset..), col);

        Position {
            line: line as u32,
            character: utf16_col as u32,
        }
    }

    fn offset_of_position(&self, pos: &Position) -> usize {
        let (line, column) = self.position_to_line_col(pos);

        self.offset_of_line_col(line, column)
    }

    fn position_to_line_col(&self, pos: &Position) -> (usize, usize) {
        let line = pos.line as usize;
        let line_offset = self.offset_of_line(line);

        let column = encoding::offset_utf16_to_utf8(
            self.char_indices_iter(line_offset..),
            pos.character as usize,
        );

        (line, column)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn replace() {}
}

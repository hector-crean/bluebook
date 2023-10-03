use std::borrow::Cow;
use std::cmp::Ordering;
use std::ops::Range;
use std::ops::{Deref, DerefMut};

use xi_rope::{
    delta::Delta, interval::IntervalBounds, rope::ChunkIter, spans::SpansBuilder, tree::Node,
    DeltaBuilder, Interval, LinesMetric, Rope, RopeDelta, RopeInfo,
};

use crate::codepoint::CharIndicesJoin;
use crate::cursor::CursorRange;
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

#[derive(Clone)]
pub struct InlayHint {}

pub struct RopeBuffer {
    pub text: Rope,
    tombstones: Rope,
}

impl RopeBuffer {
    pub fn new(s: &str) -> Self {
        Self {
            text: Rope::from(s),
            tombstones: Rope::default(),
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

    fn replace_delta(&mut self, iv: Interval, replace_with: &str) -> RopeDelta {
        let mut builder = DeltaBuilder::new(self.len());

        builder.replace(iv, Rope::from(replace_with));

        let delta = builder.build();

        delta
    }
    fn delete_delta(&mut self, iv: Interval) -> RopeDelta {
        let mut builder = DeltaBuilder::new(self.len());

        builder.delete(iv);

        let delta = builder.build();

        delta
    }
}

impl Deref for RopeBuffer {
    type Target = Rope;

    fn deref(&self) -> &Self::Target {
        &self.text
    }
}

impl DerefMut for RopeBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.text
    }
}

impl ToString for RopeBuffer {
    fn to_string(&self) -> String {
        self.text.to_string()
    }
}

impl<'a, T: Clone> From<(xi_rope::Interval, &'a T)> for crate::span::Span<T> {
    fn from(val: (xi_rope::Interval, &'a T)) -> Self {
        let start = val.0.start();
        let end = val.0.end();

        crate::span::Span {
            range: Range { start, end },
            data: val.1.clone(),
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
            text: Rope::from(s),
            tombstones: Rope::default(),
        }
    }

    fn slice(&self, range: std::ops::Range<usize>) -> Cow<str> {
        self.text.slice_to_cow(range)
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
        self.text.edit(offset..offset, s);

        Ok(offset + s.len())
    }

    fn drain(&mut self, _range: std::ops::Range<usize>) -> std::vec::Drain<&str> {
        todo!()
    }

    fn replace_range(
        &mut self,
        range: Range<usize>,
        replace_with: &str,
    ) -> Result<Range<usize>, crate::graphemes::GraphemeCursorError> {
        let iv = Interval::new(range.start, range.end);

        self.text.edit(iv, replace_with);

        Ok(range)
    }

    fn span_iter<'spans, 'buffer: 'spans>(&'buffer self) -> Self::SpanIter<'spans> {
        todo!()
    }

    fn annotate(&mut self, range: Range<usize>, data: SpanData) {
        let mut sb = SpansBuilder::new(self.len());
        sb.add_span(range, data);
        let _spans = sb.build();
    }

    fn take(&self) -> std::borrow::Cow<str> {
        self.text.slice_to_cow(..)
    }

    fn len(&self) -> usize {
        self.text.len()
    }

    fn is_empty(&self) -> bool {
        self.text.is_empty()
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
            Ordering::Equal => self.len(),
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
            line,
            character: utf16_col,
        }
    }

    fn offset_of_position(&self, pos: &Position) -> usize {
        let (line, column) = self.position_to_line_col(pos);

        self.offset_of_line_col(line, column)
    }

    fn position_to_line_col(&self, pos: &Position) -> (usize, usize) {
        let line = pos.line;
        let line_offset = self.offset_of_line(line);

        let column =
            encoding::offset_utf16_to_utf8(self.char_indices_iter(line_offset..), pos.character);

        (line, column)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn replace() {}
}

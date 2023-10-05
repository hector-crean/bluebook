use crate::{
    block::{BlockCursor, BlockCursorError},
    codepoint::{CodepointCursor, CodepointCursorError},
    graphemes::{GraphemeCursor, GraphemeCursorError},
    line::{LineCursor, LineCursorError},
    paragraph::{ParagraphCursor, ParagraphCursorError},
    position::Position,
    sentence::{SentenceCursor, SentenceCursorError},
    span::SpanData,
    word::{WordCursor, WordCursorError},
};

use std::{borrow::Cow, ops::Range};

#[derive(thiserror::Error, Debug)]
pub enum ConversionError {
    #[error("Failed to convert text representation")]
    InvalidRange,
}

pub trait TextBuffer {
    //The where Self: 'cursor clause is crucial. It ensures that any reference held by the Cursor type must outlive the 'cursor lifetime.
    //Indicates that the Cursor cannot outlive the TextBuffer it is derived from.
    type CodepointCursor<'cursor>: CodepointCursor<'cursor>
    where
        Self: 'cursor;
    type GraphemeCursor<'cursor>: GraphemeCursor<'cursor>
    where
        Self: 'cursor;
    type WordCursor<'cursor>: WordCursor<'cursor>
    where
        Self: 'cursor;

    type SentenceCursor<'cursor>: SentenceCursor<'cursor>
    where
        Self: 'cursor;

    type ParagraphCursor<'cursor>: ParagraphCursor<'cursor>
    where
        Self: 'cursor;

    type LineCursor<'cursor>: LineCursor<'cursor>
    where
        Self: 'cursor;

    type BlockCursor<'cursor>: BlockCursor<'cursor>
    where
        Self: 'cursor;

    type SpanIterItem<'span_iter>: Into<crate::span::Span<SpanData>>
    where
        Self: 'span_iter;

    type SpanIter<'span_iter>: Iterator<Item = Self::SpanIterItem<'span_iter>>
    where
        Self: 'span_iter;

    type Delta;

    //Curors take a snapshot of the underlying text buffer, and then navigate around it, holding the state of their
    //offset position internal to them. As soon as the underlying buffer changes, the cursor is invalidated, and we
    //must create a new cursor at a specified offset position. Rinse repeat.

    ///Character Cursor:
    ///Unit of Movement:
    /// A Character Cursor moves through the text one Unicode scalar value (character) at a time. Unicode scalar
    /// values are code points that represent individual characters in the Unicode standard. These code points
    /// can be one or more bytes in length.
    ///Basic Use Case:
    /// Useful for iterating through text when you don't need to account for complex characters or grapheme
    /// clusters, such as when processing text at a byte level or when character-level navigation suffices.
    fn codepoint_cursor(
        &mut self,
        offset: usize,
    ) -> Result<Self::CodepointCursor<'_>, CodepointCursorError>;
    ////Grapheme Cluster Cursor:
    /// Unit of Movement:
    /// A Grapheme Cluster Cursor moves through the text one grapheme cluster at a time. A grapheme cluster is
    /// the smallest unit of a text that a user would perceive as a single character. It can consist of a single
    /// Unicode code point or a sequence of code points.
    /// Basic Use Case:
    /// Essential when working with languages or scripts that involve complex character compositions, such as
    ///  diacritics, ligatures, or emoji sequences. It ensures that you navigate text in a way that respects
    /// the user's perception of characters.
    fn grapheme_cursor(
        &mut self,
        offset: usize,
    ) -> Result<Self::GraphemeCursor<'_>, GraphemeCursorError>;
    ///Word Cursor: Jumps between word boundaries, allowing for efficient word-based navigation.
    fn word_cursor(&mut self, offset: usize) -> Result<Self::WordCursor<'_>, WordCursorError>;
    ///Sentence Cursor: Moves between sentences or sentence-like structures in the text, based on punctuation and context.
    fn sentence_cursor(
        &mut self,
        offset: usize,
    ) -> Result<Self::SentenceCursor<'_>, SentenceCursorError>;
    ///Paragraph Cursor: Navigates between paragraphs, which are typically separated by line breaks or indentation.
    fn paragraph_cursor(
        &mut self,
        offset: usize,
    ) -> Result<Self::ParagraphCursor<'_>, ParagraphCursorError>;
    ///Line Cursor: Advances through lines or rows of text. Useful for quick navigation within a paragraph or block of text.
    fn line_cursor(&mut self, offset: usize) -> Result<Self::LineCursor<'_>, LineCursorError>;
    ///Block Cursor: Allows you to jump between blocks of text, which could be defined by headers, section breaks, or other structural elements.
    fn block_cursor(&mut self, offset: usize) -> Result<Self::BlockCursor<'_>, BlockCursorError>;

    //Quite crucial: we will implemenet default implementation to most of the traits for string slices.
    fn from_str(s: &str) -> Self;
    /// Construct an instance of this type from a `&str`.
    fn slice(&self, range: Range<usize>) -> Cow<str>;

    fn write(&mut self, offset: usize, s: &str) -> Result<Self::Delta, GraphemeCursorError>;

    fn drain(&mut self, range: Range<usize>) -> std::vec::Drain<&str>;

    fn replace_range(
        &mut self,
        range: Range<usize>,
        replace_with: &str,
    ) -> Result<Self::Delta, GraphemeCursorError>;

    // fn flush(&mut self) -> Result<(), TextBufferError>;

    fn span_iter<'spans, 'buffer: 'spans>(&'buffer self) -> Self::SpanIter<'spans>;

    fn annotate(&mut self, range: Range<usize>, data: SpanData);

    fn take(&self) -> Cow<str>;

    /// Get length of text (in bytes).
    fn len(&self) -> usize;

    /// Returns `true` if this text has 0 length.
    fn is_empty(&self) -> bool;

    fn line_of_offset(&self, offset: usize) -> usize;
    fn offset_of_line(&self, line: usize) -> usize;
    fn offset_to_position(&self, offset: usize) -> Position;
    fn offset_of_position(&self, pos: &Position) -> usize;
    fn position_to_line_col(&self, pos: &Position) -> (usize, usize);

    fn line_content(&self, line: usize) -> Cow<'_, str> {
        self.slice(self.offset_of_line(line)..self.offset_of_line(line + 1))
    }

    /// The last line of the held rope
    fn last_line(&self) -> usize {
        self.line_of_offset(self.len())
    }

    fn offset_line_end(
        &mut self,
        offset: usize,
        caret: bool,
    ) -> Result<usize, GraphemeCursorError> {
        let line = self.line_of_offset(offset);
        self.line_end_offset(line, caret)
    }

    fn offset_to_line_col(&self, offset: usize) -> (usize, usize) {
        let offset: usize = offset.min(self.len());
        let line = self.line_of_offset(offset);
        let line_start = self.offset_of_line(line);
        if offset == line_start {
            return (line, 0);
        }

        let col = offset - line_start;
        (line, col)
    }

    fn offset_of_line_col(&self, line: usize, col: usize) -> usize {
        let mut pos = 0;
        let mut offset = self.offset_of_line(line);
        for c in self.slice(offset..self.offset_of_line(line + 1)).chars() {
            if c == '\n' {
                return offset;
            }

            let char_len = c.len_utf8();
            if pos + char_len > col {
                return offset;
            }
            pos += char_len;
            offset += char_len;
        }
        offset
    }

    fn line_end_col(&mut self, line: usize, caret: bool) -> Result<usize, GraphemeCursorError> {
        let line_start = self.offset_of_line(line);
        let offset = self.line_end_offset(line, caret)?;
        Ok(offset - line_start)
    }

    fn line_end_offset(&mut self, line: usize, caret: bool) -> Result<usize, GraphemeCursorError> {
        let mut offset = self.offset_of_line(line + 1);
        let mut line_content: &str = &self.line_content(line);
        if line_content.ends_with("\r\n") {
            offset -= 2;
            line_content = &line_content[..line_content.len() - 2];
        } else if line_content.ends_with('\n') {
            offset -= 1;
            line_content = &line_content[..line_content.len() - 1];
        }
        if !caret && !line_content.is_empty() {
            let mut gc = self.grapheme_cursor(offset)?;
            if let Some(o) = gc.next_back() {
                offset = o;
            }
        }
        Ok(offset)
    }
}

#[cfg(test)]
mod test {
    use super::TextBuffer;
}

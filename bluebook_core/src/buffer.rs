use crate::{
    block::{BlockCursor, BlockCursorError},
    codepoint::{CodepointCursor, CodepointCursorError},
    coordinates::{ColPosition, RowPosition},
    graphemes::{GraphemeCursor, GraphemeCursorError},
    line::{LineCursor, LineCursorError},
    paragraph::{ParagraphCursor, ParagraphCursorError},
    sentence::{SentenceCursor, SentenceCursorError},
    span::{Span, SpanData},
    word::{WordCursor, WordCursorError},
};

use std::{
    borrow::Cow,
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
    fn slice(&self, range: Range<usize>) -> Result<Cow<str>, ConversionError>;

    fn write(&mut self, offset: usize, s: &str) -> Result<usize, GraphemeCursorError>;

    fn drain(&mut self, range: Range<usize>) -> std::vec::Drain<&str>;

    fn replace_range(
        &mut self,
        range: Range<usize>,
        replace_with: &str,
    ) -> Result<Range<usize>, GraphemeCursorError>;

    // fn flush(&mut self) -> Result<(), TextBufferError>;

    fn span_iter<'spans, 'buffer: 'spans>(&'buffer self) -> Self::SpanIter<'spans>;

    fn annotate(&mut self, range: Range<usize>, data: SpanData);

    fn take(&self) -> Cow<str>;

    /// Get length of text (in bytes).
    fn len(&self) -> usize;

    /// Returns `true` if this text has 0 length.
    fn is_empty(&self) -> bool;
}

#[cfg(test)]
mod test {
    use super::TextBuffer;
}

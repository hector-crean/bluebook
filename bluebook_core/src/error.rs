use crate::buffer;
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

#[derive(thiserror::Error, Debug)]
pub enum BluebookCoreError {
    #[error(transparent)]
    CharCursor(#[from] CodepointCursorError),
    #[error(transparent)]
    GraphemeCursor(#[from] GraphemeCursorError),
    #[error(transparent)]
    WordCursor(#[from] WordCursorError),
    #[error(transparent)]
    SentenceCursor(#[from] SentenceCursorError),
    #[error(transparent)]
    ParagraphCursor(#[from] ParagraphCursorError),
    #[error(transparent)]
    LineCursor(#[from] LineCursorError),
    #[error(transparent)]
    BlockCursor(#[from] BlockCursorError),
    #[error(transparent)]
    ConversionError(#[from] buffer::ConversionError),
}

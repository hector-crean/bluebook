use crate::text_buffer::TextBuffer;

#[derive(thiserror::Error, Debug)]
pub enum SentenceCursorError {
    #[error("Invalid sentence encountered")]
    InvalidSentence,
    // Add more error variants as needed
}

pub trait SentenceCursor<'buffer> {
    type Buffer: TextBuffer;
    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self;
}

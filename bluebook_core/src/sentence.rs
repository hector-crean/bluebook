use crate::buffer::TextBuffer;

#[derive(thiserror::Error, Debug)]
pub enum SentenceCursorError {
    #[error("Invalid sentence encountered")]
    InvalidSentence,
    // Add more error variants as needed
}

pub trait SentenceCursor<'buffer>:
    Iterator<Item = usize> + DoubleEndedIterator<Item = usize>
{
    type Buffer: TextBuffer;
    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self;
    fn offset(&self) -> usize;
}

use crate::buffer::TextBuffer;

#[derive(thiserror::Error, Debug)]
pub enum GraphemeCursorError {
    #[error("Invalid character encountered")]
    InvalidCharacter,
    // Add more error variants as needed
}

pub trait GraphemeCursor<'buffer>:
    Iterator<Item = usize> + DoubleEndedIterator<Item = usize>
{
    type Buffer: TextBuffer;
    fn new(text: &'buffer Self::Buffer, offset: usize) -> Self;
    fn offset(&self) -> usize;
}

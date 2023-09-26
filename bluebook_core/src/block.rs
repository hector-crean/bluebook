use crate::buffer::TextBuffer;

#[derive(thiserror::Error, Debug)]
pub enum BlockCursorError {
    #[error("Invalid character encountered")]
    InvalidCharacter,
    // Add more error variants as needed
}

pub trait BlockCursor<'buffer> {
    type Buffer: TextBuffer;
    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self;
    fn offset(&self) -> usize;
}

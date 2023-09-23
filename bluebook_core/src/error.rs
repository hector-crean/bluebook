use crate::graphemes::UnicodeSegmentationError;

#[derive(thiserror::Error, Debug)]
pub enum TextBufferWithCursorError {
    #[error(transparent)]
    UnicodeSegmentationError(#[from] UnicodeSegmentationError),
    #[error("Out of bounds")]
    OutOfBounds,
}

#[derive(thiserror::Error, Debug)]
pub enum TextEditorError {
    #[error(transparent)]
    TextBufferError(#[from] crate::text_buffer::TextBufferError),
    #[error(transparent)]
    TextBufferCursorError(#[from] crate::text_buffer_cursor::TextBufferCursorError),
}

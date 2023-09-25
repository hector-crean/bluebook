use crate::text_buffer;

#[derive(thiserror::Error, Debug)]
pub enum BluebookCoreError {
    #[error(transparent)]
    CursorError(#[from] text_buffer::CursorError),
    #[error(transparent)]
    ConversionError(#[from] text_buffer::ConversionError),
}

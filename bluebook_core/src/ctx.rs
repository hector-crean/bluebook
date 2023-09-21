use tracing::info;

use crate::text_buffer_cursor::{TextBufferCursor, TextBufferCursorError};
use crate::{
    buffer::peritext_buffer::cursor_impl::CursorRange, command::Transaction,
    error::TextEditorError, text_buffer::TextBuffer,
};

pub struct TextEditorContext<Buffer>
where
    Buffer: TextBuffer,
{
    pub text_buffer: Buffer,
    pub cursor_range: CursorRange,
    // cursor_mode: CursorMode,
    // motion_mode: MotionMode,
}

impl<'ctx, Buffer> TextEditorContext<Buffer>
where
    Buffer: TextBuffer,
{
    pub fn new(text_buffer: Buffer, cursor_range: CursorRange) -> Self {
        Self {
            text_buffer: text_buffer.into(),
            cursor_range: cursor_range.into(),
        }
    }

    pub fn consume_transaction<B: TextBuffer>(
        &mut self,
        transaction: Transaction,
    ) -> Result<bool, TextEditorError> {
        let success = match transaction {
            Transaction::DeleteSelection => match self.cursor_range.is_empty() {
                true => Ok(false),
                false => {
                    let CursorRange { anchor, head } = self.cursor_range;
                    let _ = self.text_buffer.drain(anchor..head)?;
                    self.cursor_range.set_point(anchor);

                    Ok(true)
                }
            },
            Transaction::DeleteBackward => match self.cursor_range.is_empty() {
                true => {
                    let CursorRange { anchor, head } = self.cursor_range;

                    let cursor = self.text_buffer.cursor(self.cursor_range)?;

                    let offset = cursor.nth_prev_grapheme_boundary(1)?;

                    drop(cursor);

                    let _ = self.text_buffer.drain(offset..head)?;

                    self.cursor_range.set_point(offset);

                    Ok(true)
                }
                false => {
                    // let CursorRange { anchor, head } = self.cursor_range;
                    // let _ = self.text_buffer.drain(anchor..head)?;
                    Ok(true)
                }
            },
            Transaction::InsertAtCursorHead { value: s } | Transaction::Paste { clipboard: s } => {
                let CursorRange { head, .. } = self.cursor_range;
                let byte_idx = self.text_buffer.write(head, &s)?;
                self.cursor_range.set_point(byte_idx);
                Ok(true)
            }

            Transaction::MoveCursorHeadTo { offset } => {
                let r = self.cursor_range.set_head(offset);

                let cursor = self.text_buffer.cursor(*r)?;

                self.cursor_range.set(cursor.range());

                Ok(true)
            }
            Transaction::MoveCursorLeft { grapheme_count } => {
                let cursor = self.text_buffer.cursor(self.cursor_range)?;

                let offset = cursor.nth_prev_grapheme_boundary(grapheme_count);

                let transaction_suceeded = match offset {
                    Ok(offset) => {
                        self.cursor_range.set_point(offset);
                        true
                    }
                    Err(err) => match err {
                        TextBufferCursorError::PrevGraphemeOffsetError => {
                            // self.cursor_range.set_point(0);
                            true
                        }
                        _ => false,
                    },
                };

                Ok(transaction_suceeded)
            }
            Transaction::MoveCursorRight { grapheme_count } => {
                let cursor = self.text_buffer.cursor(self.cursor_range)?;

                let offset = cursor.nth_next_grapheme_boundary(grapheme_count)?;

                self.cursor_range.set_point(offset);

                Ok(true)
            }
            _ => Ok(false),
        };

        tracing::info!("Cursor Range {:?}", self.cursor_range);

        success
    }
}

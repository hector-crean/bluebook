use crate::{
    command::Transaction, cursor::CursorRange, error::BluebookCoreError, text_buffer::TextBuffer,
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
            text_buffer,
            cursor_range,
        }
    }

    pub fn consume_transaction<B: TextBuffer>(
        &mut self,
        transaction: Transaction,
    ) -> Result<bool, BluebookCoreError> {
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
                    let CursorRange { anchor: _, head } = self.cursor_range;

                    let cursor = self.text_buffer.cursor(self.cursor_range)?;

                    let offset = if let Some(offset) = cursor.nth_prev_grapheme_boundary(1)? {
                        offset
                    } else {
                        return Ok(false);
                    };

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

            Transaction::InsertNewLine => {
                let newline = &"\n".to_string();
                let CursorRange { head, .. } = self.cursor_range;
                let byte_idx = self.text_buffer.write(head, newline)?;
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

                let transaction_suceeded =
                    if let Some(offset) = cursor.nth_prev_grapheme_boundary(grapheme_count)? {
                        self.cursor_range.set_point(offset);
                        true
                    } else {
                        return Ok(false);
                    };

                Ok(transaction_suceeded)
            }
            Transaction::MoveCursorRight { grapheme_count } => {
                let cursor = self.text_buffer.cursor(self.cursor_range)?;

                let transaction_suceeded =
                    if let Some(offset) = cursor.nth_next_grapheme_boundary(grapheme_count)? {
                        self.cursor_range.set_point(offset);
                        true
                    } else {
                        return Ok(false);
                    };

                Ok(transaction_suceeded)
            }
            _ => Ok(false),
        };

        success
    }
}

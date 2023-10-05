use crate::graphemes::GraphemeCursor;
use crate::{
    buffer::TextBuffer, command::Transaction, cursor::CursorRange, error::BluebookCoreError,
    span::Spanslike,
};
pub struct TextEditorContext<Buffer, Spans, BufferDelta>
where
    Buffer: TextBuffer,
    Spans: Spanslike<Delta = BufferDelta>,
{
    pub text_buffer: Buffer,
    pub cursor_range: CursorRange,
    pub spans: Spans,
}

impl<'ctx, Buffer, Spans, BufferDelta> TextEditorContext<Buffer, Spans, BufferDelta>
where
    Buffer: TextBuffer<Delta = BufferDelta>,
    Spans: Spanslike<Delta = BufferDelta>,
{
    pub fn new(text_buffer: Buffer, cursor_range: CursorRange, spans: Spans) -> Self {
        Self {
            text_buffer,
            cursor_range,
            spans,
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
                    let _ = self.text_buffer.replace_range(anchor..head, "")?;
                    self.cursor_range.set_point(anchor);

                    Ok(true)
                }
            },
            Transaction::DeleteBackward => match self.cursor_range.is_empty() {
                true => {
                    let CursorRange { anchor: _, head } = self.cursor_range;

                    let mut gc = self.text_buffer.grapheme_cursor(self.cursor_range.head)?;

                    let offset = if let Some(offset) = gc.nth_back(0) {
                        offset
                    } else {
                        0
                    };

                    drop(gc);

                    let _ = self.text_buffer.replace_range(offset..head, "")?;

                    self.cursor_range.set_point(offset);

                    Ok(true)
                }
                false => {
                    // let CursorRange { anchor, head } = self.cursor_range;
                    // let _ = self.text_buffer.drain(anchor..head)?;
                    Ok(true)
                }
            },
            Transaction::InsertAtCursorHead { value: s } => {
                let CursorRange { head, .. } = self.cursor_range;
                let delta = self.text_buffer.write(head, &s)?;
                self.spans.update(&delta);

                self.cursor_range.set_point(head + s.len());

                tracing::info!("{:?}", self.text_buffer.slice(0..self.text_buffer.len()));

                Ok(true)
            }
            Transaction::Paste { clipboard: s } => {
                let CursorRange { head, .. } = self.cursor_range;
                let delta = self.text_buffer.write(head, &s)?;
                self.spans.update(&delta);

                self.cursor_range.set_point(head + s.len());

                tracing::info!("{:?}", self.text_buffer.slice(0..self.text_buffer.len()));

                Ok(true)
            }
            Transaction::InsertNewLine => {
                let CursorRange { head, .. } = self.cursor_range;

                let newline_char = '\n'.to_string();
                let delta = self.text_buffer.write(head, &newline_char)?;
                self.spans.update(&delta);

                self.cursor_range.set_point(head + newline_char.len());

                tracing::info!("{:?}", self.text_buffer.slice(0..self.text_buffer.len()));

                Ok(true)
            }

            Transaction::MoveCursorLeft { grapheme_count } => {
                let mut gc = self.text_buffer.grapheme_cursor(self.cursor_range.head)?;

                let transaction_suceeded = if let Some(offset) = gc.nth_back(grapheme_count) {
                    self.cursor_range.set_point(offset);
                    true
                } else {
                    return Ok(false);
                };

                Ok(transaction_suceeded)
            }
            Transaction::MoveCursorRight { grapheme_count } => {
                let mut gc = self.text_buffer.grapheme_cursor(self.cursor_range.head)?;

                let transaction_suceeded = if let Some(offset) = gc.nth(grapheme_count) {
                    self.cursor_range.set_point(offset);
                    true
                } else {
                    return Ok(false);
                };

                Ok(transaction_suceeded)
            }
            Transaction::MoveCursorHeadTo { offset } => {
                self.cursor_range.set_point(offset);

                Ok(true)
            }
            _ => Ok(false),
        };

        success
    }
}

use crate::text_buffer_cursor::TextBufferCursor;
use crate::{
    buffer::peritext_buffer::cursor_impl::CursorRange,
    command::Transaction,
    context::{FromContext, Handler},
    error::TextEditorError,
    text_buffer::TextBuffer,
};
use serde::{Deserialize, Serialize};
mod msg {

    use crate::command::Transaction;

    use super::*;

    pub fn edit<'ctx, B: TextBuffer>(mut buffer: B) -> Result<Transaction, TextEditorError> {
        // let offset = buffer.write(0, "hello, my name is Hector")?;

        let _cursor = buffer.cursor(CursorRange { anchor: 0, head: 0 })?;

        Ok(Transaction::Append)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionMode {
    Delete { count: usize },
    Yank { count: usize },
    Indent,
    Outdent,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CursorMode {
    Normal(usize),
    // Insert(Selection),
}

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
        match transaction {
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
                println!("{:?}", byte_idx);
                Ok(true)
            }
            Transaction::MoveCursorHeadTo { offset } => {
                let r = CursorRange::new(self.cursor_range.anchor, offset);

                let cursor = self.text_buffer.cursor(r)?;

                self.cursor_range.set(cursor.range());

                Ok(true)
            }
            Transaction::MoveCursorLeft { grapheme_count } => {
                let cursor = self.text_buffer.cursor(self.cursor_range)?;

                let offset = cursor.nth_prev_grapheme_boundary(grapheme_count)?;

                self.cursor_range.set_point(offset);

                Ok(true)
            }
            Transaction::MoveCursorRight { grapheme_count } => {
                let cursor = self.text_buffer.cursor(self.cursor_range)?;

                let offset = cursor.nth_next_grapheme_boundary(grapheme_count)?;

                self.cursor_range.set_point(offset);

                Ok(true)
            }
            _ => Ok(false),
        }
    }

    // pub fn send_events(&'ctx mut self) -> Result<Transaction, TextEditorError> {

    // }

    pub fn send<T, H, R>(&'ctx self, handler: H) -> R
    where
        H: Handler<'ctx, Self, T, R>,
        Self: 'ctx,
    {
        handler.call(self)
    }

    // fn commands(&'ctx self) -> Result<Transaction, TextEditorError> {
    //     self.send(msg::edit)
    // }

    fn event_reader(self, edit_command: Transaction) -> Result<bool, TextEditorError> {
        use Transaction::*;
        match edit_command {
            MoveLineDown => Ok(true),
            _ => Ok(true),
        }
    }
}

// impl<'ctx, Buffer: TextBuffer + 'ctx> FromContext<'ctx> for Buffer {
//     type Context = TextEditorContext<Buffer>;
//     fn from_context(context: &'ctx Self::Context) -> Self {
//         context.text_buffer
//     }
// }

// impl<'ctx, Buffer: TextBuffer> FromContext<'ctx> for &'ctx CursorRange {
//     type Context = TextEditorContext<'ctx, Buffer>;
//     type Marker = PhantomData<Buffer>;

//     fn from_context(context: &Self::Context) -> Self {
//         context.cursor_range
//     }
// }

#[cfg(test)]
mod tests {
    use crate::buffer::peritext_buffer::buffer_impl::Peritext;

    use super::*;

    #[test]
    fn magic_params_editor() {
        let mut buf = Peritext::new(1);
        let mut cursor_range = CursorRange::default();

        let _ctx = TextEditorContext::<Peritext>::new(buf, cursor_range);

        // let handler = |cursor_range: CursorRange| -> TextEditorContext<Peritext> { todo!() };
    }
}

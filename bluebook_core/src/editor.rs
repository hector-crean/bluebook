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

    pub fn edit<'ctx, B: TextBuffer>(buffer: &B) -> Result<Transaction, TextEditorError> {
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

pub struct TextEditorContext<'ctx, Buffer>
where
    Buffer: TextBuffer,
{
    pub text_buffer: &'ctx mut Buffer,
    pub cursor_range: &'ctx mut CursorRange,
    // cursor_mode: CursorMode,
    // motion_mode: MotionMode,
}

impl<'ctx, Buffer> TextEditorContext<'ctx, Buffer>
where
    Buffer: TextBuffer,
{
    pub fn new(
        text_buffer: impl Into<&'ctx mut Buffer>,
        cursor_range: impl Into<&'ctx mut CursorRange>,
    ) -> Self {
        Self {
            text_buffer: text_buffer.into(),
            cursor_range: cursor_range.into(),
        }
    }

    pub fn consume_transaction<B: TextBuffer>(
        self,
        transaction: Transaction,
    ) -> Result<bool, TextEditorError> {
        let Self {
            text_buffer,
            cursor_range,
        } = self;
        match transaction {
            Transaction::DeleteSelection => match cursor_range.is_empty() {
                true => Ok(false),
                false => {
                    let CursorRange { anchor, head } = *cursor_range;
                    let _ = text_buffer.drain(anchor..head)?;
                    Ok(true)
                }
            },
            Transaction::InsertAtCursorHead { value: s } | Transaction::Paste { clipboard: s } => {
                let CursorRange { head, .. } = *cursor_range;
                text_buffer.write(head, &s)?;
                Ok(true)
            }
            Transaction::MoveCursorHeadTo { offset } => {
                let r = CursorRange::new(cursor_range.anchor, offset);

                let cursor = text_buffer.cursor(r)?;

                cursor_range.set(cursor.range());

                Ok(true)
            }
            Transaction::MoveCursorLeft { grapheme_count } => {
                let cursor = text_buffer.cursor(*cursor_range)?;

                let offset = cursor.nth_prev_grapheme_boundary(grapheme_count)?;

                cursor_range.set_head(offset);

                Ok(true)
            }
            Transaction::MoveCursorRight { grapheme_count } => {
                let cursor = text_buffer.cursor(*cursor_range)?;

                let offset = cursor.nth_next_grapheme_boundary(grapheme_count)?;

                cursor_range.set_head(offset);

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

    fn commands(&'ctx self) -> Result<Transaction, TextEditorError> {
        self.send(msg::edit)
    }

    fn event_reader(self, edit_command: Transaction) -> Result<bool, TextEditorError> {
        use Transaction::*;
        match edit_command {
            MoveLineDown => Ok(true),
            _ => Ok(true),
        }
    }
}

impl<'ctx, Buffer: TextBuffer> FromContext<'ctx> for &'ctx Buffer {
    type Context = TextEditorContext<'ctx, Buffer>;
    fn from_context(context: &'ctx Self::Context) -> Self {
        context.text_buffer
    }
}

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

        let _ctx = TextEditorContext::<Peritext>::new(&mut buf, &mut cursor_range);

        // let handler = |cursor_range: CursorRange| -> TextEditorContext<Peritext> { todo!() };
    }
}

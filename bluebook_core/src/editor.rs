use std::{any::Any, marker::PhantomData};

use crate::{
    buffer::peritext_buffer::cursor_impl::CursorRange,
    command::EditCommand,
    context::{FromContext, Handler},
    error::TextEditorError,
    text_buffer::{TextBuffer, TextBufferError},
};
use serde::{Deserialize, Serialize};

mod msg {

    use super::*;

    pub fn edit<'ctx, B: TextBuffer<'ctx>>(buffer: &B) -> Result<EditCommand, TextEditorError> {
        // let offset = buffer.write(0, "hello, my name is Hector")?;

        let cursor = buffer.cursor(CursorRange { anchor: 0, head: 0 })?;

        Ok(EditCommand::Append)
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
    Buffer: TextBuffer<'ctx>,
{
    text_buffer: &'ctx mut Buffer,
    cursor_range: &'ctx mut CursorRange,
    // cursor_mode: CursorMode,
    // motion_mode: MotionMode,
}

impl<'ctx, Buffer> TextEditorContext<'ctx, Buffer>
where
    Buffer: TextBuffer<'ctx>,
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

    pub fn send<T, H, R>(&'ctx self, handler: H) -> R
    where
        H: Handler<'ctx, Self, T, R>,
        Self: 'ctx,
    {
        handler.call(self)
    }

    fn commands(&'ctx self) -> Result<EditCommand, TextEditorError> {
        self.send(msg::edit)
    }

    fn event_reader(mut self, edit_command: EditCommand) -> Result<bool, TextEditorError> {
        use EditCommand::*;
        match edit_command {
            MoveLineDown => Ok(true),
            _ => Ok(true),
        }
    }
}

impl<'ctx, Buffer: TextBuffer<'ctx>> FromContext<'ctx> for &'ctx Buffer {
    type Context = TextEditorContext<'ctx, Buffer>;
    fn from_context(context: &'ctx Self::Context) -> Self {
        &context.text_buffer
    }
}

// impl<'ctx, Buffer: TextBuffer<'ctx>> FromContext<'ctx> for &'ctx CursorRange {
//     type Context = TextEditorContext<'ctx, Buffer>;
//     type Marker = PhantomData<Buffer>;

//     fn from_context(context: &Self::Context) -> Self {
//         context.cursor_range
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{
        buffer::peritext_buffer::buffer_impl::Peritext,
        error::TextEditorError,
        text_buffer::{self, TextBufferError},
    };

    use super::*;

    #[test]
    fn magic_params_editor() -> () {
        let mut buf = Peritext::new(1);
        let mut cursor_range = CursorRange::default();

        let ctx = TextEditorContext::<Peritext>::new(&mut buf, &mut cursor_range);

        // let handler = |cursor_range: CursorRange| -> TextEditorContext<Peritext> { todo!() };
    }
}

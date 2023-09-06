use std::cell::RefMut;

use crate::{command::EditCommand, text_buffer::TextBuffer, text_buffer_cursor::TextBufferCursor};
use serde::{Deserialize, Serialize};

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

pub struct TextEditor<'buffer, Buffer>
where
    Buffer: TextBuffer,
{
    text_buffer: &'buffer mut Buffer,
    // cursor_mode: CursorMode,
    // motion_mode: MotionMode,
}

impl<'buffer, Buffer> TextEditor<'buffer, Buffer>
where
    Buffer: TextBuffer,
{
    pub fn new(text_buffer: impl Into<&'buffer mut Buffer>) -> Self {
        Self {
            text_buffer: text_buffer.into(),
        }
    }

    fn consume_edit_command(self, edit_command: EditCommand) -> () {
        use EditCommand::*;
        match edit_command {
            MoveLineDown => self.text_buffer.replace_range(.., ""),
            MoveLineDown => {}
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn editor() {
        let editor = TextEditor {
            text_buffer: &mut String::from(""),
        };

        editor.consume_edit_command(EditCommand::DeleteWordBackward);
    }
}

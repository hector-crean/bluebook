use crate::text_buffer_cursor::TextBufferCursor;
use crate::{
    buffer::peritext_buffer::grapheme::nth_next_grapheme_boundary, selection::CursorRange,
    text_buffer::TextBuffer,
};

use std::{cmp::Reverse, iter};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Movement {
    Extend,
    Move,
}

impl CursorRange {
    pub fn move_horizontally<B: TextBuffer>(
        self,
        buffer: B,
        dir: Direction,
        count: usize,
        behaviour: Movement,
    ) -> CursorRange {
        let head_cursor = buffer.cursor(self.head);

        let new_pos = match head_cursor {
            Some(head_cursor) => match dir {
                Direction::Forward => head_cursor.nth_next_grapheme_boundary(count),
                Direction::Backward => head_cursor.nth_prev_grapheme_boundary(count),
            },
            None => None,
        };

        let cursor_range = self.put_block_cursor(
            &buffer,
            new_pos.unwrap_or(self.head),
            behaviour == Movement::Extend,
        );

        cursor_range
    }
}

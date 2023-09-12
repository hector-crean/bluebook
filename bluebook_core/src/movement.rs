use crate::text_buffer_cursor::TextBufferCursor;
use crate::{selection::CursorRange, text_buffer::TextBuffer};

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
        buffer: &B,
        dir: Direction,
        count: usize,
        behaviour: Movement,
    ) -> CursorRange {
        let anchor_cursor = buffer.cursor(self.anchor);

        let new_pos = match anchor_cursor {
            Some(cursor) => match dir {
                Direction::Forward => cursor.nth_next_grapheme_boundary(count),
                Direction::Backward => cursor.nth_prev_grapheme_boundary(count),
            },
            None => None,
        };

        println!("{:?}", &new_pos);

        let cursor_range = self.put_block_cursor(
            buffer,
            new_pos.unwrap_or(self.anchor),
            behaviour == Movement::Extend,
        );

        cursor_range
    }
}

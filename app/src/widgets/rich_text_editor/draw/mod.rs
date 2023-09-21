use bluebook_core::buffer::peritext_buffer::cursor_impl::CursorRange;
use egui::{Galley, Painter, Pos2, Rect};

pub struct Drawer {
    p: Painter,
}

impl Drawer {
    pub fn new(p: Painter) -> Self {
        Drawer { p }
    }

    pub fn draw(self) {}
}

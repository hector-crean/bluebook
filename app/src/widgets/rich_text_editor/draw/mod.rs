use std::{sync::Arc};


use egui::{Color32, Galley, Painter, Pos2, Rect};

pub struct Draw<'painter> {
    painter: &'painter Painter,
}

impl<'painter> Draw<'painter> {
    pub fn new(painter: &'painter Painter) -> Self {
        Draw { painter }
    }

    pub fn draw_cursor(&self, rect: Rect) {
        let top = rect.center_top();
        let bottom = rect.center_bottom();

        self.painter.line_segment([top, bottom], (1., Color32::RED));
    }
    pub fn draw_text(&self, pos: Pos2, galley: Arc<Galley>) {
        self.painter.galley(pos, galley);
    }
}

use egui::Painter;

pub struct Drawer {
    p: Painter,
}

impl Drawer {
    pub fn new(p: Painter) -> Self {
        Drawer { p }
    }

    pub fn draw(self) {}
}

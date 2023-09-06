use app::{
    easy_mark_editor::{self, EasyMarkEditor},
    widgets::rich_text_editor::editor::TextEditor,
};
use eframe::{self, egui};
use egui::{ScrollArea, Vec2};

// #[derive(serde::Deserialize, serde::Serialize)]
#[derive(Default)]
struct TextEditApp {}

impl TextEditApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let editor = TextEditor::<String>::default();

        ScrollArea::vertical()
            .id_source("source")
            .show(ui, |ui| ui.add(editor));
    }
}

impl eframe::App for TextEditApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    {
        // Silence wgpu log spam (https://github.com/gfx-rs/wgpu/issues/3206)
        let mut rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_owned());
        for loud_crate in ["naga", "wgpu_core", "wgpu_hal"] {
            if !rust_log.contains(&format!("{loud_crate}=")) {
                rust_log += &format!(",{loud_crate}=warn");
            }
        }
        std::env::set_var("RUST_LOG", rust_log);
    }

    let options = eframe::NativeOptions {
        drag_and_drop_support: true,
        initial_window_size: Some([1280.0, 1024.0].into()),
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|cc| Box::new(TextEditApp::new(cc))),
    )?;

    Ok(())
}

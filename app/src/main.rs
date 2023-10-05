use bluebook_app::widgets::rich_text_editor::view::{
    editor_ui, egui_transact_fn, EguiTextEditor, ViewCtx, ViewSettings,
};

use bluebook_core::{
    buffer_impl::rope::{
        buffer::RopeBuffer,
        span::{Delta, RopeInfo, RopeSpans, Spans},
    },
    ctx::TextEditorContext,
    cursor::CursorRange,
    editor::TextEditor,
};
use eframe::{self, egui};
use egui::{Align2, Id, ScrollArea, Vec2, Widget};

use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};

use tracing::Level;
use tracing_subscriber::prelude::*;

// #[derive(serde::Deserialize, serde::Serialize)]
struct TextEditApp {
    editor: EguiTextEditor<RopeBuffer, RopeSpans, Delta<RopeInfo>>,
}

impl TextEditApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let buf = RopeBuffer::new("");
        let cursor_range = CursorRange::default();
        let spans = RopeSpans::new(&buf);

        // println!("{:?}, {:?}", &self.cursor_range, &self.buf.take());
        let edit_ctx = TextEditorContext::new(buf, cursor_range, spans);
        let view_settings =
            ViewSettings::new(Id::new("text_editor"), Vec2::ZERO, Align2::CENTER_CENTER);

        let editor = TextEditor::<
            RopeBuffer,
            RopeSpans,
            Delta<RopeInfo>,
            egui::Event,
            ViewSettings,
            ViewCtx,
        >::new(edit_ctx, egui_transact_fn, view_settings);

        Self {
            editor: EguiTextEditor(editor),
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        let TextEditApp { editor } = self;

        ScrollArea::vertical().id_source("source").show(ui, |ui| {
            ui.add(editor_ui::<RopeBuffer, RopeSpans, Delta<RopeInfo>>(editor))
        });
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
    let filter = filter::Targets::new()
        .with_target("bluebook_core", Level::INFO)
        .with_target("bluebook_app", Level::INFO);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

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
        "bluebook",
        options,
        Box::new(|cc| Box::new(TextEditApp::new(cc))),
    )?;

    Ok(())
}

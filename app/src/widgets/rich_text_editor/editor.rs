use std::{collections::HashMap, ops::Range, sync::Arc};

use bluebook_core::{
    annotation::Span, command::EditCommand, text_buffer::TextBuffer,
    text_buffer_cursor::TextBufferCursor,
};
use egui::{
    text::{LayoutJob, LayoutSection},
    vec2, Align, Align2, Color32, FontSelection, Galley, Margin, NumExt, Response, ScrollArea,
    Sense, TextFormat, Ui, Vec2,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use string_cache::Atom;

use crate::formatting::Formatting;

pub struct TextEditor<Buffer>
where
    Buffer: TextBuffer,
{
    text_buffer: Buffer,
    spans: Vec<Span>,
    font_selection: FontSelection,
    margin: Vec2,
    align: Align2,
}

impl Default for TextEditor<String> {
    fn default() -> Self {
        let span = Span {
            range: Range { start: 2, end: 4 },
            attributes: HashMap::from_iter([(
                Formatting::Italic.atom(),
                json!({
                    "code": 200,
                    "success": true,
                    "payload": {
                        "features": [
                            "serde",
                            "json"
                        ]
                    }
                }),
            )]),
        };

        Self {
            text_buffer: String::from("Title: My Day\n\nToday was a good day.\nI woke up early, went for a run, and then had a hearty breakfast.\nNote: Buy more eggs."),
            spans: vec![span],
            font_selection: FontSelection::Default,
            margin: Vec2::ZERO,
            align: Align2::CENTER_CENTER,
        }
    }
}

impl<'buffer, Buffer> TextEditor<Buffer>
where
    Buffer: TextBuffer,
{
    pub fn new(
        text_buffer: impl Into<Buffer>,
        spans: Vec<Span>,
        font_selection: FontSelection,
        margin: Vec2,
        align: Align2,
    ) -> Self {
        Self {
            text_buffer: text_buffer.into(),
            spans,
            font_selection,
            margin,
            align,
        }
    }

    // fn consume_edit_command(self, edit_command: EditCommand) -> () {
    //     use EditCommand::*;
    //     match edit_command {
    //         MoveLineDown => self.text_buffer.replace_range(.., ""),
    //         MoveLineDown => {}
    //         _ => {}
    //     }
    // }

    fn layout_job(&self) -> LayoutJob {
        let buffer = self.text_buffer.take();

        let mut job = LayoutJob {
            sections: vec![LayoutSection {
                leading_space: 0.0,
                byte_range: 0..self.text_buffer.len(),
                format: TextFormat {
                    font_id: egui::FontId::monospace(12.0),
                    color: Color32::DARK_RED,
                    ..Default::default()
                },
            }],
            text: buffer.into(),
            break_on_newline: true,
            ..Default::default()
        };

        for Span { range, attributes } in self.spans.iter() {
            for attribute in attributes.iter() {
                let formatting: Formatting = attribute.into();

                job.sections.push(LayoutSection {
                    leading_space: 0.,
                    byte_range: range.clone(),
                    format: formatting.into(),
                })
            }
        }

        job
    }
    fn layouter(&self, ui: &Ui) -> Arc<Galley> {
        let job = self.layout_job();
        let galley = ui.fonts(|rdr| rdr.layout_job(job));
        galley
    }
}

impl<Buffer: TextBuffer + Default> egui::Widget for TextEditor<Buffer> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let galley = self.layouter(ui);

        let font_id = self.font_selection.resolve(ui.style());

        let available_width = ui.available_width().at_least(24.0);
        let desired_width = ui.spacing().text_edit_width;
        let wrap_width = if ui.layout().horizontal_justify() {
            available_width
        } else {
            desired_width.min(available_width)
        } - self.margin.x * 2.0;
        let desired_width = galley.size().x.max(wrap_width);

        let row_height = ui.fonts(|f| f.row_height(&font_id));

        let desired_height = 4. * row_height;
        let desired_size = vec2(desired_width, galley.size().y.max(desired_height))
            .at_least(Vec2::ZERO - self.margin * 2.0);

        let (auto_id, rect) = ui.allocate_space(desired_size);

        let sense = Sense::click_and_drag();

        let response = ui.interact(rect, auto_id, sense);

        let painter = ui.painter_at(rect.expand(1.0)); // expand to avoid clipping cursor

        let text_draw_pos = self
            .align
            .align_size_within_rect(galley.size(), response.rect)
            .intersect(response.rect) // limit pos to the response rect area
            .min;

        painter.galley(text_draw_pos, galley.clone());

        response
    }
}

// use std::io::{Seek, SeekFrom};

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

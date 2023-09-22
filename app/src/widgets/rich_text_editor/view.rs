use std::{
    collections::HashMap,
    ops::{Deref, DerefMut, Range},
    sync::Arc,
};

use bluebook_core::{
    buffer::peritext_buffer::{buffer_impl::Peritext, cursor_impl::CursorRange},
    command::Transaction,
    ctx::TextEditorContext,
    editor::TextEditor,
    movement::{Direction, Movement},
    span::Span,
    text_buffer::TextBuffer,
    text_buffer_cursor::{CursorDocCoords, TextBufferCursor},
};
use egui::{
    epaint::text::{cursor::Cursor, Row, TextWrapping},
    text::{CCursor, LayoutJob, LayoutSection},
    vec2, Align, Align2, Color32, Context, Event, FontId, FontSelection, Galley, Id, Key, Margin,
    NumExt, Painter, Pos2, Rect, Response, ScrollArea, Sense, TextFormat, Ui, Vec2,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use string_cache::Atom;
use tracing::info;

use crate::formatting::{Formatting, TextFormatBuilder};

use super::draw::Draw;

#[derive(thiserror::Error, Debug)]
pub enum TextEditorError {
    #[error(transparent)]
    TextBufferCursorError(#[from] bluebook_core::error::TextBufferWithCursorError),
}

pub struct EguiViewCtx {
    id: Id,
    margin: Vec2,
    align: Align2,
}
impl EguiViewCtx {
    pub fn new(id: Id, margin: Vec2, align: Align2) -> Self {
        Self { id, margin, align }
    }
}

pub struct EguiTextEditor<Buf: TextBuffer>(pub TextEditor<Buf, egui::Event, EguiViewCtx>);

impl<Buf: TextBuffer> Deref for EguiTextEditor<Buf> {
    type Target = TextEditor<Buf, egui::Event, EguiViewCtx>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Buf: TextBuffer> DerefMut for EguiTextEditor<Buf> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub fn editor_ui<'ctx, Buffer: TextBuffer + 'ctx>(
    text_edtitor: &'ctx mut EguiTextEditor<Buffer>,
) -> impl egui::Widget + 'ctx {
    move |ui: &mut egui::Ui| text_edtitor.editor_ui(ui)
}

pub fn egui_transact_fn<'ctx, Buf: TextBuffer>(
    ctx: &'ctx TextEditorContext<Buf>,
    event: &Event,
) -> Option<Transaction> {
    let transaction = match event {
        Event::Copy => None,
        Event::CompositionEnd(c) => None,
        Event::CompositionUpdate(c) => None,
        Event::CompositionStart => None,
        Event::Cut => None,
        Event::Key {
            key,
            pressed,
            repeat,
            modifiers,
        } => match (key, pressed) {
            (Key::Backspace, true) => Some(Transaction::DeleteBackward),
            (Key::Enter, true) => Some(Transaction::InsertNewLine),
            (Key::ArrowLeft, true) => Some(Transaction::MoveCursorLeft { grapheme_count: 1 }),
            (Key::ArrowRight, true) => Some(Transaction::MoveCursorRight { grapheme_count: 1 }),

            _ => None,
        },
        Event::MouseWheel {
            unit,
            delta,
            modifiers,
        } => None,
        Event::Paste(s) => Some(Transaction::Paste {
            clipboard: s.clone(),
        }),
        Event::PointerButton {
            pos,
            button,
            pressed,
            modifiers,
        } => None,
        Event::PointerGone => None,
        Event::PointerMoved(c) => None,
        Event::Text(s) => Some(Transaction::InsertAtCursorHead { value: s.into() }),

        _ => None,
    };

    transaction
}

impl<'ctx, Buffer> EguiTextEditor<Buffer>
where
    Buffer: TextBuffer,
{
    fn editor_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let font_id = FontSelection::default().resolve(ui.style());

        // Calculate widget size and allocate space in one step:
        let max_rect = ui
            .available_rect_before_wrap()
            .shrink2(self.view_ctx.margin);

        let galley = self.rich_text_layouter(ui, max_rect.width());

        let (auto_id, rect) = {
            let desired_size = self.size(ui, &galley.size(), &font_id);
            ui.allocate_space(desired_size)
        };

        // Interact and handle events:
        let mut response = ui.interact(rect, auto_id, Sense::click_and_drag());
        let events = ui.input(|i| i.events.clone());

        let requires_change = events.iter().any(|event| {
            self.emit_transcation(event).map_or(false, |t| {
                self.edit_ctx()
                    .consume_transaction::<Buffer>(t)
                    .unwrap_or(true)
            })
        });

        if requires_change {
            response.mark_changed();
        }

        // Paint if visible:
        if ui.is_rect_visible(rect) {
            let draw_position = self.draw_position(galley.size(), rect);

            let cursor_rect = self.cursor_rect(ui, &font_id, &galley, draw_position);

            let painter = ui.painter_at(rect.expand(1.0));
            let draw = Draw::new(&painter);

            if let Ok(cursor_rect) = cursor_rect {
                draw.draw_cursor(cursor_rect);
            }
            draw.draw_text(draw_position, galley);
        }

        response
    }

    fn rich_text_layouter(&self, ui: &Ui, max_width: f32) -> Arc<Galley> {
        let buffer = self.0.edit_ctx.text_buffer.take();

        let mut job = LayoutJob {
            text: buffer.into(),
            break_on_newline: true,
            wrap: TextWrapping {
                max_width: f32::INFINITY,
                ..Default::default()
            },
            ..Default::default()
        };

        for span in self.0.edit_ctx.text_buffer.span_iter() {
            let Span { insert, attributes } = span.into();

            let mut bldr = TextFormatBuilder::new();

            for attribute in attributes.iter() {
                let formatting: Formatting = attribute.into();
                match formatting {
                    Formatting::Italic => {
                        bldr = bldr.italics(true);
                    }
                    Formatting::Bold => bldr = bldr.color(Color32::DARK_RED),
                    Formatting::Comment(_) => {
                        bldr = bldr.background(Color32::YELLOW);
                    }

                    _ => {}
                }
            }
            job.append(&insert, 0., bldr.build())
        }

        let galley = ui.fonts(|rdr| rdr.layout_job(job));
        galley
    }

    fn row_height(ui: &Ui, font_id: &FontId) -> f32 {
        ui.fonts(|f| f.row_height(&font_id))
    }

    fn size(&self, ui: &Ui, galley_size: &Vec2, font_id: &FontId) -> Vec2 {
        let available_width = ui.available_width().at_least(24.0);
        let horizontal_justify = ui.layout().horizontal_justify();
        let text_edit_width = ui.spacing().text_edit_width;

        let wrap_width = if horizontal_justify {
            available_width
        } else {
            text_edit_width.min(available_width)
        } - self.0.view_ctx.margin.x * 2.0;

        let desired_width = galley_size.x.max(wrap_width);
        let row_height = ui.fonts(|f| f.row_height(&font_id));
        let desired_height = 4.0 * row_height;

        let desired_size = Vec2::new(desired_width, galley_size.y.max(desired_height))
            .at_least(Vec2::ZERO - self.0.view_ctx.margin * 2.0);

        desired_size
    }

    fn draw_position(&self, size: Vec2, frame: Rect) -> Pos2 {
        let text_draw_pos = self
            .0
            .view_ctx
            .align
            .align_size_within_rect(size, frame)
            .intersect(frame) // limit pos to the response rect area
            .min;

        text_draw_pos
    }

    pub fn cursor_rect(
        &mut self,
        ui: &Ui,
        font_id: &FontId,
        galley: &Galley,
        draw_position: Pos2,
    ) -> Result<Rect, TextEditorError> {
        let CursorDocCoords { row, col } = self
            .0
            .edit_ctx
            .text_buffer
            .cursor_coords(self.0.edit_ctx.cursor_range)?;

        fn is_row_wrapped(row: &Row) -> bool {
            !row.ends_with_newline
        }

        let prev_wrapped_row_count = {
            galley
                .rows
                .iter()
                .take(row)
                .filter(|row| is_row_wrapped(row))
                .count()
        };

        tracing::info!("{:?}", prev_wrapped_row_count);

        let galley_row = &galley.rows[row + prev_wrapped_row_count];

        let screen_x = galley_row.x_offset(col);

        let row_height = ui.fonts(|f| f.row_height(&font_id));

        let cursor_rect = Rect::from_min_max(
            draw_position + vec2(screen_x, galley_row.min_y()),
            draw_position
                + vec2(
                    screen_x,
                    galley_row.max_y().at_least(galley_row.min_y() + row_height),
                ),
        )
        .expand(1.5);

        Ok(cursor_rect)
    }
}

// impl<'ctx, Buffer: TextBuffer> egui::Widget for EguiTextEditor<Buffer> {
//     fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
//         let font_id = FontSelection::default().resolve(ui.style());

//         let galley = self.layouter(ui, ui.available_width());

//         let desired_size = self.size(ui, galley.size(), &font_id);

//         let (auto_id, rect) = ui.allocate_space(desired_size);

//         let id = ui.make_persistent_id(self.view_ctx().id);

//         let sense = Sense::click_and_drag();

//         let mut response = ui.interact(rect, auto_id, sense);

//         let painter = ui.painter_at(rect.expand(1.0)); // expand to avoid clipping cursor

//         let text_draw_pos = self.draw_position(galley.size(), response.rect);

//         self.paint_cursor(ui, &font_id, &painter, &galley);

//         painter.galley(text_draw_pos, galley);

//         let events = ui.input(|i| i.events.clone());

//         // let transactions = events.iter().map(|e| interpret_event(&self.ctx, e));

//         for event in &events {
//             let transaction = self.emit_transcation(event);

//             match transaction {
//                 Some(t) => {
//                     let success = self.edit_ctx().consume_transaction::<Buffer>(t);
//                 }
//                 _ => {}
//             }
//         }

//         response
//     }
// }

#[derive(Clone, Default)]
pub struct TextEditorState {
    cursor_range: CursorRange,
}

impl TextEditorState {
    pub fn load(ctx: &Context, id: Id) -> Option<Self> {
        ctx.data_mut(|d| d.get_persisted(id))
    }

    pub fn store(self, ctx: &Context, id: Id) {
        ctx.data_mut(|d| d.insert_persisted(id, self));
    }

    pub fn set_cursor_range(&mut self, cursor_range: CursorRange) {
        self.cursor_range = cursor_range;
    }
}

impl<'ctx, Buffer: TextBuffer> egui::WidgetWithState for EguiTextEditor<Buffer> {
    type State = TextEditorState;
}

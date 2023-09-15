use std::{collections::HashMap, ops::Range, sync::Arc};

use bluebook_core::{
    buffer::peritext_buffer::{buffer_impl::Peritext, cursor_impl::CursorRange},
    command::Transaction,
    editor::TextEditorContext,
    movement::{Direction, Movement},
    span::Span,
    text_buffer::TextBuffer,
    text_buffer_cursor::TextBufferCursor,
};
use egui::{
    epaint::text::{cursor::Cursor, TextWrapping},
    text::{CCursor, LayoutJob, LayoutSection},
    vec2, Align, Align2, Color32, Context, Event, FontId, FontSelection, Galley, Id, Key, Margin,
    NumExt, Painter, Pos2, Rect, Response, ScrollArea, Sense, TextFormat, Ui, Vec2,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use string_cache::Atom;

use crate::formatting::{Formatting, TextFormatBuilder};

#[derive(thiserror::Error, Debug)]
pub enum TextEditorError {
    #[error(transparent)]
    TextBufferError(#[from] bluebook_core::text_buffer::TextBufferError),
    #[error(transparent)]
    TextBufferCursorError(#[from] bluebook_core::text_buffer_cursor::TextBufferCursorError),
}

pub struct TextEditor<'ctx, Buffer>
where
    Buffer: TextBuffer,
{
    ctx: TextEditorContext<'ctx, Buffer>,
    id: Id,
    margin: Vec2,
    align: Align2,
}

impl<'ctx, Buffer> TextEditor<'ctx, Buffer>
where
    Buffer: TextBuffer,
{
    pub fn new(
        id: Id,
        text_buffer: &'ctx mut Buffer,
        cursor_range: &'ctx mut CursorRange,
        margin: Vec2,
        align: Align2,
    ) -> Self {
        let ctx = TextEditorContext::new(text_buffer, cursor_range);
        Self {
            ctx,
            id,
            margin,
            align,
        }
    }

    fn layouter(&self, ui: &Ui, max_width: f32) -> Arc<Galley> {
        let buffer = self.ctx.text_buffer.take();

        let mut job = LayoutJob {
            text: buffer.into(),
            break_on_newline: true,
            wrap: TextWrapping {
                max_width,
                ..Default::default()
            },
            ..Default::default()
        };

        for span in self.ctx.text_buffer.span_iter() {
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

    fn size(&self, ui: &Ui, galley_size: Vec2, font_id: &FontId) -> Vec2 {
        let available_width = ui.available_width().at_least(24.0);

        let wrap_width = if ui.layout().horizontal_justify() {
            available_width
        } else {
            ui.spacing().text_edit_width.min(available_width)
        } - self.margin.x * 2.0;

        let desired_width = galley_size.x.max(wrap_width);

        let row_height = ui.fonts(|f| f.row_height(&font_id));

        let desired_height = 4. * row_height;

        let desired_size = vec2(desired_width, galley_size.y.max(desired_height))
            .at_least(Vec2::ZERO - self.margin * 2.0);

        desired_size
    }

    fn draw_position(&self, size: Vec2, frame: Rect) -> Pos2 {
        let text_draw_pos = self
            .align
            .align_size_within_rect(size, frame)
            .intersect(frame) // limit pos to the response rect area
            .min;

        text_draw_pos
    }

    pub fn send_command(
        &mut self,
        event: &Event,
        ui: &Ui,
        galley: &Galley,
        id: Id,
    ) -> Result<Option<Transaction>, TextEditorError> {
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
                (Key::Backspace | Key::Delete, true) => None,
                (Key::Enter, true) => None,
                (Key::ArrowLeft, true) => None,
                (Key::ArrowRight, true) => None,
                _ => None,
            },
            Event::MouseWheel {
                unit,
                delta,
                modifiers,
            } => None,
            Event::Paste(s) => None,
            Event::PointerButton {
                pos,
                button,
                pressed,
                modifiers,
            } => None,
            Event::PointerGone => None,
            Event::PointerMoved(c) => None,
            Event::Text(s) => None,

            _ => None,
        };

        Ok(transaction)
    }

    fn commands_iter(
        &'ctx mut self,
        ui: &'ctx Ui,
        galley: &'ctx Galley,
        id: Id,
    ) -> impl Iterator<Item = Result<Option<Transaction>, TextEditorError>> + 'ctx {
        let iter = ui
            .input(|i| i.events.clone())
            .into_iter()
            .map(move |event| self.send_command(&event, ui, galley, id));

        iter
    }

    fn consume_transaction(
        self,
        transaction: impl Iterator<Item = Result<Option<Transaction>, TextEditorError>> + 'ctx,
    ) -> Result<(), TextEditorError> {
        for txtn in transaction.into_iter() {
            match txtn {
                Ok(Some(txt)) => {
                    self.ctx.consume_transaction::<Buffer>(txt);
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn paint_cursor(
        &self,
        ui: &Ui,
        font_id: &FontId,
        painter: &Painter,
        pos: Pos2,
        galley: &Galley,
        cursor: &Cursor,
    ) -> Rect {
        let row_height = ui.fonts(|f| f.row_height(&font_id));

        let mut cursor_rect = galley.pos_from_cursor(cursor).translate(pos.to_vec2());

        cursor_rect.max.y = cursor_rect.max.y.at_least(cursor_rect.min.y + row_height); // Handle completely empty galleys
        cursor_rect = cursor_rect.expand(1.5); // slightly above/below row

        let top = cursor_rect.center_top();
        let bottom = cursor_rect.center_bottom();

        painter.line_segment([top, bottom], (1., Color32::RED));

        cursor_rect
    }
}

impl<'ctx, Buffer: TextBuffer> egui::Widget for TextEditor<'ctx, Buffer> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let font_id = FontSelection::default().resolve(ui.style());

        let galley = self.layouter(ui, ui.available_width());

        let desired_size = self.size(ui, galley.size(), &font_id);

        let (auto_id, rect) = ui.allocate_space(desired_size);

        let id = ui.make_persistent_id(self.id);

        let signal = match self.events(ui, &galley, id) {
            Ok(signal) => signal,
            Err(_) => false,
        };

        let sense = Sense::click_and_drag();

        let mut response = ui.interact(rect, auto_id, sense);

        let painter = ui.painter_at(rect.expand(1.0)); // expand to avoid clipping cursor

        let text_draw_pos = self.draw_position(galley.size(), response.rect);

        if signal {
            response.mark_changed()
        }

        // match &state.cursor_range {
        //     Some(cursor_range) => {
        //         self.paint_cursor(
        //             ui,
        //             &font_id,
        //             &painter,
        //             response.rect.min,
        //             &galley,
        //             &cursor_range.primary,
        //         );
        //     }
        //     None => {}
        // }

        painter.galley(text_draw_pos, galley);

        response
    }
}

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

impl<'ctx, Buffer: TextBuffer> egui::WidgetWithState for TextEditor<'ctx, Buffer> {
    type State = TextEditorState;
}

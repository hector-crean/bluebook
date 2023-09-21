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
            (Key::Enter, true) => None,
            (Key::ArrowLeft, true) => Some(Transaction::MoveCursorLeft { grapheme_count: 1 }),
            (Key::ArrowRight, true) => Some(Transaction::MoveCursorRight { grapheme_count: 1 }),

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

        let galley = self.layouter(ui, ui.available_width());

        let desired_size = self.size(ui, galley.size(), &font_id);

        let (auto_id, rect) = ui.allocate_space(desired_size);

        let id = ui.make_persistent_id(self.view_ctx().id);

        let sense = Sense::click_and_drag();

        let mut response = ui.interact(rect, auto_id, sense);

        let painter = ui.painter_at(rect.expand(1.0)); // expand to avoid clipping cursor

        let text_draw_pos = self.draw_position(galley.size(), response.rect);

        painter.galley(text_draw_pos, galley);

        let events = ui.input(|i| i.events.clone());

        // let transactions = events.iter().map(|e| interpret_event(&self.ctx, e));

        for event in &events {
            let edit_ctx = &(self.edit_ctx);
            let transaction = &self.0.emit_transcation(event);

            match transaction {
                Some(t) => {
                    let success = self.edit_ctx().consume_transaction::<Buffer>(t.clone());
                }
                _ => {}
            }
        }

        response
    }

    // pub fn toggle(self, on: &'ctx mut bool) -> impl egui::Widget + 'ctx {
    //     move |ui: &mut egui::Ui| self.text_editor_ui(ui, on)
    // }

    pub fn toggle_ui(&mut self, ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
        //Widget code can be broken up in four steps:
        //  1. Decide a size for the widget
        //  2. Allocate space for it
        //  3. Handle interactions with the widget (if any)
        //  4. Paint the widget

        // 1. Deciding widget size:
        // You can query the `ui` how much space is available
        let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

        // 2. Allocating space:
        // This is where we get a region of the screen assigned.
        // We also tell the Ui to sense clicks in the allocated region.
        let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

        // 3. Interact: Time to check for clicks!
        if response.clicked() {
            *on = !*on;
            response.mark_changed(); // report back that the value changed
        }
        // Attach some meta-data to the response which can be used by screen readers:
        response.widget_info(|| egui::WidgetInfo::selected(egui::WidgetType::Checkbox, *on, ""));

        // 4. Paint!
        // Make sure we need to paint:
        if ui.is_rect_visible(rect) {
            // Let's ask for a simple animation from egui.
            // egui keeps track of changes in the boolean associated with the id and
            // returns an animated value in the 0-1 range for how much "on" we are.
            let how_on = ui.ctx().animate_bool(response.id, *on);
            // We will follow the current style by asking
            // "how should something that is being interacted with be painted?".
            // This will, for instance, give us different colors when the widget is hovered or clicked.
            let visuals = ui.style().interact_selectable(&response, *on);
            // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
            let rect = rect.expand(visuals.expansion);
            let radius = 0.5 * rect.height();
            ui.painter()
                .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
            // Paint the circle, animating it from left to right with `how_on`:
            let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
            let center = egui::pos2(circle_x, rect.center().y);
            ui.painter()
                .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
        }

        // All done! Return the interaction response so the user can check what happened
        // (hovered, clicked, ...) and maybe show a tooltip:
        response
    }

    fn layouter(&self, ui: &Ui, max_width: f32) -> Arc<Galley> {
        let buffer = self.0.edit_ctx.text_buffer.take();

        let mut job = LayoutJob {
            text: buffer.into(),
            break_on_newline: true,
            wrap: TextWrapping {
                max_width,
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

    fn size(&self, ui: &Ui, galley_size: Vec2, font_id: &FontId) -> Vec2 {
        let available_width = ui.available_width().at_least(24.0);

        let wrap_width = if ui.layout().horizontal_justify() {
            available_width
        } else {
            ui.spacing().text_edit_width.min(available_width)
        } - self.0.view_ctx.margin.x * 2.0;

        let desired_width = galley_size.x.max(wrap_width);

        let row_height = ui.fonts(|f| f.row_height(&font_id));

        let desired_height = 4. * row_height;

        let desired_size = vec2(desired_width, galley_size.y.max(desired_height))
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

impl<'ctx, Buffer: TextBuffer> egui::Widget for EguiTextEditor<Buffer> {
    fn ui(mut self, ui: &mut egui::Ui) -> egui::Response {
        let font_id = FontSelection::default().resolve(ui.style());

        let galley = self.layouter(ui, ui.available_width());

        let desired_size = self.size(ui, galley.size(), &font_id);

        let (auto_id, rect) = ui.allocate_space(desired_size);

        let id = ui.make_persistent_id(self.view_ctx().id);

        let sense = Sense::click_and_drag();

        let mut response = ui.interact(rect, auto_id, sense);

        let painter = ui.painter_at(rect.expand(1.0)); // expand to avoid clipping cursor

        let text_draw_pos = self.draw_position(galley.size(), response.rect);

        painter.galley(text_draw_pos, galley);

        let events = ui.input(|i| i.events.clone());

        // let transactions = events.iter().map(|e| interpret_event(&self.ctx, e));

        for event in &events {
            let transaction = self.emit_transcation(event);

            match transaction {
                Some(t) => {
                    let success = self.edit_ctx().consume_transaction::<Buffer>(t);
                }
                _ => {}
            }
        }

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

impl<'ctx, Buffer: TextBuffer> egui::WidgetWithState for EguiTextEditor<Buffer> {
    type State = TextEditorState;
}

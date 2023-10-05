use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use bluebook_core::{
    buffer::TextBuffer,
    buffer_impl::rope::word_cursor,
    command::Transaction,
    ctx::TextEditorContext,
    cursor::CursorRange,
    editor::TextEditor,
    position::Position,
    span::{Span, SpanData, Spanslike},
};
use egui::{
    epaint::text::{Row, TextWrapping},
    text::LayoutJob,
    vec2, Align2, Color32, Context, Event, FontId, FontSelection, Galley, Id, InputState, Key,
    Modifiers, NumExt, PointerButton, Pos2, Rect, Response, Sense, TextFormat, Ui, Vec2,
};

use crate::formatting::{Formatting, TextFormatBuilder};

use super::{cursor::cursor_from_visual_position, draw::Draw};

#[derive(thiserror::Error, Debug)]
pub enum TextEditorError {
    #[error(transparent)]
    BluebookCoreError(#[from] bluebook_core::error::BluebookCoreError),
}

pub struct ViewSettings {
    id: Id,
    margin: Vec2,
    align: Align2,
}

impl ViewSettings {
    pub fn new(id: Id, margin: Vec2, align: Align2) -> Self {
        Self { id, margin, align }
    }
}

pub struct ViewCtx {
    pub response: Response,
    pub galley: Arc<Galley>,
    // pub text_draw_pos: Pos2,
    // pub text_clip_rect: Rect,
}
impl ViewCtx {
    fn new(response: Response, galley: Arc<Galley>, text_draw_pos: Pos2, clip_rect: Rect) -> Self {
        Self { response, galley }
    }
}

pub struct EguiTextEditor<Buf: TextBuffer, Spans: Spanslike<Delta = BufferDelta>, BufferDelta>(
    pub TextEditor<Buf, Spans, BufferDelta, egui::Event, ViewSettings, ViewCtx>,
);

// impl<Buf: TextBuffer> Deref for EguiTextEditor<Buf> {
//     type Target = TextEditor<Buf, egui::Event, ViewSettings, ViewCtx>;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl<Buf: TextBuffer> DerefMut for EguiTextEditor<Buf> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

pub fn editor_ui<
    'ctx,
    Buffer: TextBuffer<Delta = BufferDelta> + 'ctx,
    Spans: Spanslike<Delta = BufferDelta>,
    BufferDelta,
>(
    text_edtitor: &'ctx mut EguiTextEditor<Buffer, Spans, BufferDelta>,
) -> impl egui::Widget + 'ctx {
    move |ui: &mut egui::Ui| text_edtitor.editor_ui(ui)
}

pub fn egui_transact_fn<Buf: TextBuffer, Spans: Spanslike<Delta = BufferDelta>, BufferDelta>(
    ctx: &TextEditorContext<Buf, Spans, BufferDelta>,
    (event, view_ctx): (&Event, &ViewCtx),
) -> Option<Transaction> {
    match event {
        Event::Copy => None,
        Event::CompositionEnd(_c) => None,
        Event::CompositionUpdate(_c) => None,
        Event::CompositionStart => None,
        Event::Cut => None,
        Event::Key {
            key,
            pressed,
            repeat: _,
            modifiers: _,
        } => match (key, pressed) {
            (Key::Backspace, true) => Some(Transaction::DeleteBackward),
            (Key::Enter, true) => Some(Transaction::InsertNewLine),
            (Key::ArrowLeft, true) => Some(Transaction::MoveCursorLeft { grapheme_count: 0 }),
            (Key::ArrowRight, true) => Some(Transaction::MoveCursorRight { grapheme_count: 0 }),

            _ => None,
        },
        Event::MouseWheel {
            unit: _,
            delta: _,
            modifiers: _,
        } => None,
        Event::Paste(s) => Some(Transaction::Paste {
            clipboard: s.clone(),
        }),

        Event::PointerButton {
            pos,
            button,
            pressed,
            modifiers,
        } => match (pos, button, pressed, modifiers) {
            (_, PointerButton::Primary, true, &Modifiers::NONE) => {
                let cursor_doc_pos =
                    cursor_from_visual_position(&view_ctx.galley, vec2(pos.x, pos.y));

                let offset_of_position = ctx.text_buffer.offset_of_position(&cursor_doc_pos);

                Some(Transaction::MoveCursorHeadTo {
                    offset: offset_of_position,
                })
            }
            _ => None,
        },
        Event::PointerGone => None,
        Event::PointerMoved(_c) => None,
        Event::Text(s) => Some(Transaction::InsertAtCursorHead { value: s.into() }),

        _ => None,
    }
}

impl<'ctx, Buffer, Spans, BufferDelta> EguiTextEditor<Buffer, Spans, BufferDelta>
where
    Buffer: TextBuffer<Delta = BufferDelta>,
    Spans: Spanslike<Delta = BufferDelta>,
{
    fn editor_ui(&mut self, ui: &mut egui::Ui) -> egui::Response {
        let font_id = FontSelection::default().resolve(ui.style());

        // Calculate widget size and allocate space in one step:
        let max_rect = ui
            .available_rect_before_wrap()
            .shrink2(self.0.settings.margin);

        let galley = self.rich_text_layouter(ui, max_rect.width());

        let (auto_id, rect) = {
            let desired_size = self.size(ui, &galley.size(), &font_id);
            ui.allocate_space(desired_size)
        };

        // Interact and handle events:
        let response = ui.interact(rect, auto_id, Sense::click_and_drag());

        let events = ui.input(|input| {
            let egui::InputState { events, .. } = input;

            events.clone()
        });

        let size = &galley.size();
        let draw_position = self.draw_position(&size, &rect);
        let cursor_rect = self.cursor_rect(ui, &font_id, &galley, draw_position);
        let painter = ui.painter_at(rect.expand(1.0));
        let draw = Draw::new(&painter);

        let view_ctx = ViewCtx::new(response, galley, draw_position, rect);

        let requires_change = events.iter().any(|event| {
            self.0
                .emit_transaction(event, &view_ctx)
                .map_or(false, |t| {
                    self.0
                        .edit_ctx()
                        .consume_transaction::<Buffer>(t)
                        .unwrap_or(true)
                })
        });

        if requires_change {
            // response.mark_changed();
        }

        // Paint if visible:
        if ui.is_rect_visible(rect) {
            if let Ok(cursor_rect) = cursor_rect {
                draw.draw_cursor(cursor_rect);
            }
            draw.draw_text(draw_position, view_ctx.galley);
        }

        view_ctx.response
    }

    fn rich_text_layouter(&mut self, ui: &Ui, _max_width: f32) -> Arc<Galley> {
        let len = self.0.edit_ctx.text_buffer.len();
        let s = self.0.edit_ctx.text_buffer.slice(0..len).to_string();

        let mut job = LayoutJob {
            break_on_newline: true,
            wrap: TextWrapping {
                max_width: f32::INFINITY,
                ..Default::default()
            },
            ..Default::default()
        };

        let bldr = TextFormatBuilder::new();

        job.append(&s, 0., bldr.build());

        // for span in self.0.edit_ctx.text_buffer.span_iter().into_iter() {
        //     let Span::<SpanData> { range, data } = span.into();

        //     // for attribute in attributes.iter() {
        //     //     let formatting: Formatting = attribute.into();
        //     //     match formatting {
        //     //         Formatting::Italic => {
        //     //             bldr = bldr.italics(true);
        //     //         }
        //     //         Formatting::Bold => bldr = bldr.color(Color32::DARK_RED),
        //     //         Formatting::Comment(_) => {
        //     //             bldr = bldr.background(Color32::YELLOW);
        //     //         }

        //     //         _ => {}
        //     //     }
        //     // }
        //     job.append(&insert, 0., bldr.build())
        // }

        ui.fonts(|rdr| rdr.layout_job(job))
    }

    fn row_height(ui: &Ui, font_id: &FontId) -> f32 {
        ui.fonts(|f| f.row_height(font_id))
    }

    fn size(&self, ui: &Ui, galley_size: &Vec2, font_id: &FontId) -> Vec2 {
        let available_width = ui.available_width().at_least(24.0);
        let horizontal_justify = ui.layout().horizontal_justify();
        let text_edit_width = ui.spacing().text_edit_width;

        let wrap_width = if horizontal_justify {
            available_width
        } else {
            text_edit_width.min(available_width)
        } - self.0.settings.margin.x * 2.0;

        let desired_width = galley_size.x.max(wrap_width);
        let row_height = ui.fonts(|f| f.row_height(font_id));
        let desired_height = 4.0 * row_height;

        Vec2::new(desired_width, galley_size.y.max(desired_height))
            .at_least(Vec2::ZERO - self.0.settings.margin * 2.0)
    }

    fn draw_position(&self, size: &Vec2, frame: &Rect) -> Pos2 {
        self.0
            .settings
            .align
            .align_size_within_rect(size.clone(), frame.clone())
            .intersect(frame.clone()) // limit pos to the response rect area
            .min
    }

    pub fn cursor_rect(
        &mut self,
        ui: &Ui,
        font_id: &FontId,
        galley: &Galley,
        draw_position: Pos2,
    ) -> Result<Rect, TextEditorError> {
        let TextEditorContext {
            text_buffer,
            cursor_range,
            spans,
        } = &self.0.edit_ctx;

        let Position {
            line: row,
            character: col,
        } = text_buffer.offset_to_position(cursor_range.head);

        let galley_row = match &galley.rows.get(row as usize) {
            Some(row) => row,
            None => match galley.rows.last() {
                Some(last) => last,
                None => &galley.rows[0],
            },
        };

        let screen_x = galley_row.x_offset(col as usize);

        let row_height = ui.fonts(|f| f.row_height(font_id));

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

impl<'ctx, Buffer: TextBuffer, Spans: Spanslike<Delta = BufferDelta>, BufferDelta>
    egui::WidgetWithState for EguiTextEditor<Buffer, Spans, BufferDelta>
{
    type State = TextEditorState;
}

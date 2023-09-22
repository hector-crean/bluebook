use crate::ctx::TextEditorContext;
use crate::expr::Expr;
use crate::text_buffer_cursor::{CursorDocCoords, TextBufferCursor};
use crate::{
    buffer::peritext_buffer::cursor_impl::CursorRange, command::Transaction,
    error::TextBufferWithCursorError, text_buffer::TextBuffer,
};
use serde::{Deserialize, Serialize};

pub struct TextEditor<Buf: TextBuffer, EventRep, ViewCtx> {
    pub edit_ctx: TextEditorContext<Buf>,
    pub transact_fn: fn(&TextEditorContext<Buf>, &EventRep) -> Option<Transaction>,
    // pub cursor_view_transform_fn: fn(&CursorDocCoords, CursorFnArgs) -> CursorDocCoords,
    pub view_ctx: ViewCtx,
}

impl<Buf: TextBuffer, EventRep, ViewCtx> Expr for TextEditor<Buf, EventRep, ViewCtx> {
    type Repr<T> = T;
    type Ctx = TextEditorContext<Buf>;

    fn app<F: Fn(&Self::Ctx, A) -> B, A, B>(
        ctx: &Self::Ctx,
        f: Self::Repr<F>,
        arg: Self::Repr<A>,
    ) -> Self::Repr<B> {
        f(ctx, arg)
    }
}

impl<Buf: TextBuffer, EventRep, ViewCtx> TextEditor<Buf, EventRep, ViewCtx> {
    pub fn new(
        edit_ctx: TextEditorContext<Buf>,
        transact_fn: fn(&TextEditorContext<Buf>, &EventRep) -> Option<Transaction>,
        // cursor_view_transform_fn: fn(&CursorDocCoords, CursorFnArgs) -> CursorDocCoords,
        view_ctx: ViewCtx,
    ) -> Self {
        Self {
            edit_ctx,
            transact_fn,
            view_ctx,
        }
    }
    pub fn edit_ctx(&mut self) -> &mut TextEditorContext<Buf> {
        &mut self.edit_ctx
    }
    pub fn view_ctx(&mut self) -> &mut ViewCtx {
        &mut self.view_ctx
    }
    pub fn emit_transcation(&self, event: &EventRep) -> Option<Transaction> {
        Self::app(&(self.edit_ctx), self.transact_fn, event)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn magic_params_editor() {}
}

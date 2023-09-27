use crate::ctx::TextEditorContext;
use crate::expr::Expr;

use crate::{buffer::TextBuffer, command::Transaction};

pub struct TextEditor<Buf: TextBuffer, EventRep, ViewSettings, ViewCtx> {
    pub edit_ctx: TextEditorContext<Buf>,
    pub transact_fn: fn(&TextEditorContext<Buf>, (&EventRep, &ViewCtx)) -> Option<Transaction>,
    pub settings: ViewSettings,
}

impl<Buf: TextBuffer, EventRep, ViewSettings, ViewCtx> Expr
    for TextEditor<Buf, EventRep, ViewSettings, ViewCtx>
{
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

impl<Buf: TextBuffer, EventRep, ViewSettings, ViewCtx>
    TextEditor<Buf, EventRep, ViewSettings, ViewCtx>
{
    pub fn new(
        edit_ctx: TextEditorContext<Buf>,
        transact_fn: fn(&TextEditorContext<Buf>, (&EventRep, &ViewCtx)) -> Option<Transaction>,
        // cursor_view_transform_fn: fn(&CursorDocCoords, CursorFnArgs) -> CursorDocCoords,
        settings: ViewSettings,
    ) -> Self {
        Self {
            edit_ctx,
            transact_fn,
            settings,
        }
    }
    pub fn edit_ctx(&mut self) -> &mut TextEditorContext<Buf> {
        &mut self.edit_ctx
    }
    pub fn view_settings(&mut self) -> &mut ViewSettings {
        &mut self.settings
    }
    pub fn emit_transcation(&self, event: &EventRep, view_ctx: &ViewCtx) -> Option<Transaction> {
        Self::app(&(self.edit_ctx), self.transact_fn, (event, view_ctx))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn magic_params_editor() {}
}

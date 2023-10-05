use std::marker::PhantomData;

use crate::ctx::TextEditorContext;
use crate::expr::Expr;

use crate::span::Spanslike;
use crate::{buffer::TextBuffer, command::Transaction};

/// Represents a text editor with customizable behavior.
pub struct TextEditor<
    Buf: TextBuffer,
    Spans: Spanslike<Delta = BufferDelta>,
    BufferDelta,
    EventRep,
    ViewSettings,
    ViewCtx,
> {
    _delta: PhantomData<BufferDelta>,
    pub edit_ctx: TextEditorContext<Buf, Spans, BufferDelta>,
    pub transact_fn: fn(
        &TextEditorContext<Buf, Spans, BufferDelta>,
        (&EventRep, &ViewCtx),
    ) -> Option<Transaction>,
    pub settings: ViewSettings,
}

impl<
        Buf: TextBuffer,
        Spans: Spanslike<Delta = BufferDelta>,
        BufferDelta,
        EventRep,
        ViewSettings,
        ViewCtx,
    > Expr for TextEditor<Buf, Spans, BufferDelta, EventRep, ViewSettings, ViewCtx>
{
    type Repr<T> = T;
    type Ctx = TextEditorContext<Buf, Spans, BufferDelta>;

    /// Applies a function to the context and an argument.
    fn app<F: Fn(&Self::Ctx, A) -> B, A, B>(
        ctx: &Self::Ctx,
        f: Self::Repr<F>,
        arg: Self::Repr<A>,
    ) -> Self::Repr<B> {
        f(ctx, arg)
    }
}

impl<
        Buf: TextBuffer,
        Spans: Spanslike<Delta = BufferDelta>,
        BufferDelta,
        EventRep,
        ViewSettings,
        ViewCtx,
    > TextEditor<Buf, Spans, BufferDelta, EventRep, ViewSettings, ViewCtx>
{
    /// Constructs a new TextEditor instance.
    pub fn new(
        edit_ctx: TextEditorContext<Buf, Spans, BufferDelta>,
        transact_fn: fn(
            &TextEditorContext<Buf, Spans, BufferDelta>,
            (&EventRep, &ViewCtx),
        ) -> Option<Transaction>,
        settings: ViewSettings,
    ) -> Self {
        Self {
            _delta: PhantomData,
            edit_ctx,
            transact_fn,
            settings,
        }
    }

    /// Provides mutable access to the editor context.
    pub fn edit_ctx(&mut self) -> &mut TextEditorContext<Buf, Spans, BufferDelta> {
        &mut self.edit_ctx
    }

    /// Provides mutable access to the view settings.
    pub fn view_settings(&mut self) -> &mut ViewSettings {
        &mut self.settings
    }

    /// Emits a transaction based on an event and view context.
    pub fn emit_transaction(&self, event: &EventRep, view_ctx: &ViewCtx) -> Option<Transaction> {
        Self::app(&(self.edit_ctx), self.transact_fn, (event, view_ctx))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn magic_params_editor() {}
}

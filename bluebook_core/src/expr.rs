pub trait Expr {
    type Repr<T>;
    type Ctx;

    fn app<F: Fn(&Self::Ctx, A) -> B, A, B>(
        ctx: &Self::Ctx,
        f: Self::Repr<F>,
        arg: Self::Repr<A>,
    ) -> Self::Repr<B>;
}

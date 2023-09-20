pub trait FromContext<'ctx> {
    type Context: 'ctx;
    fn from_context(context: &'ctx Self::Context) -> Self;
}

pub trait Handler<'ctx, C: 'ctx, T, R> {
    fn call(self, context: &'ctx C) -> R;
}

impl<'ctx, C: 'ctx, F, T, R> Handler<'ctx, C, T, R> for F
where
    F: FnMut(T) -> R,
    T: FromContext<'ctx, Context = C>,
{
    fn call(mut self, context: &'ctx C) -> R {
        (self)(T::from_context(context))
    }
}

impl<'ctx, C: 'ctx, T1, T2, F, R> Handler<'ctx, C, (T1, T2), R> for F
where
    F: FnMut(T1, T2) -> R,
    T1: FromContext<'ctx, Context = C>,
    T2: FromContext<'ctx, Context = C>,
{
    fn call(mut self, context: &'ctx C) -> R {
        (self)(T1::from_context(context), T2::from_context(context))
    }
}

impl<'ctx, C: 'ctx, T1, T2, T3, F, R> Handler<'ctx, C, (T1, T2, T3), R> for F
where
    F: FnMut(T1, T2, T3) -> R,
    T1: FromContext<'ctx, Context = C>,
    T2: FromContext<'ctx, Context = C>,
    T3: FromContext<'ctx, Context = C>,
{
    fn call(mut self, context: &'ctx C) -> R {
        (self)(
            T1::from_context(context),
            T2::from_context(context),
            T3::from_context(context),
        )
    }
}

impl<'ctx, C: 'ctx, T1, T2, T3, T4, F, R> Handler<'ctx, C, (T1, T2, T3, T4), R> for F
where
    F: FnMut(T1, T2, T3, T4) -> R,
    T1: FromContext<'ctx, Context = C>,
    T2: FromContext<'ctx, Context = C>,
    T3: FromContext<'ctx, Context = C>,
    T4: FromContext<'ctx, Context = C>,
{
    fn call(mut self, context: &'ctx C) -> R {
        (self)(
            T1::from_context(context),
            T2::from_context(context),
            T3::from_context(context),
            T4::from_context(context),
        )
    }
}

impl<'ctx, C: 'ctx, T1, T2, T3, T4, T5, F, R> Handler<'ctx, C, (T1, T2, T3, T4, T5), R> for F
where
    F: FnMut(T1, T2, T3, T4, T5) -> R,
    T1: FromContext<'ctx, Context = C>,
    T2: FromContext<'ctx, Context = C>,
    T3: FromContext<'ctx, Context = C>,
    T4: FromContext<'ctx, Context = C>,
    T5: FromContext<'ctx, Context = C>,
{
    fn call(mut self, context: &'ctx C) -> R {
        (self)(
            T1::from_context(context),
            T2::from_context(context),
            T3::from_context(context),
            T4::from_context(context),
            T5::from_context(context),
        )
    }
}

// impl<'ctx, C: 'ctx, T1, T2, T3, T4, T5, T6, F, R> Handler<'ctx, C, (T1, T2, T3, T4, T5, T6), R>
//     for F
// where
//     F: FnMut(T1, T2, T3, T4, T5, T6) -> R,
//     T1: FromContext<'ctx, Context = C>,
//     T2: FromContext<'ctx, Context = C>,
//     T3: FromContext<'ctx, Context = C>,
//     T4: FromContext<'ctx, Context = C>,
//     T5: FromContext<'ctx, Context = C>,
//     T6: FromContext<'ctx, Context = C>,
// {
//     fn call(mut self, mut context: &'ctx mut C) -> R {
//         (self)(
//             T1::from_context(&mut context),
//             T2::from_context(&mut context),
//             T3::from_context(&mut context),
//             T4::from_context(&mut context),
//             T5::from_context(&mut context),
//             T6::from_context(&mut context),
//         )
//     }
// }

// impl<'ctx, C: 'ctx, T1, T2, T3, T4, T5, T6, T7, F, R>
//     Handler<'ctx, C, (T1, T2, T3, T4, T5, T6, T7), R> for F
// where
//     F: FnMut(T1, T2, T3, T4, T5, T6, T7) -> R,
//     T1: FromContext<'ctx, Context = C>,
//     T2: FromContext<'ctx, Context = C>,
//     T3: FromContext<'ctx, Context = C>,
//     T4: FromContext<'ctx, Context = C>,
//     T5: FromContext<'ctx, Context = C>,
//     T6: FromContext<'ctx, Context = C>,
//     T7: FromContext<'ctx, Context = C>,
// {
//     fn call(mut self, mut context: &'ctx mut C) -> R {
//         (self)(
//             T1::from_context(&mut context),
//             T2::from_context(&mut context),
//             T3::from_context(&mut context),
//             T4::from_context(&mut context),
//             T5::from_context(&mut context),
//             T6::from_context(&mut context),
//             T7::from_context(&mut context),
//         )
//     }
// }

// impl<'ctx, C: 'ctx, T1, T2, T3, T4, T5, T6, T7, T8, F, R>
//     Handler<'ctx, C, (T1, T2, T3, T4, T5, T6, T7, T8), R> for F
// where
//     F: FnMut(T1, T2, T3, T4, T5, T6, T7, T8) -> R,
//     T1: FromContext<'ctx, Context = C>,
//     T2: FromContext<'ctx, Context = C>,
//     T3: FromContext<'ctx, Context = C>,
//     T4: FromContext<'ctx, Context = C>,
//     T5: FromContext<'ctx, Context = C>,
//     T6: FromContext<'ctx, Context = C>,
//     T7: FromContext<'ctx, Context = C>,
//     T8: FromContext<'ctx, Context = C>,
// {
//     fn call(mut self, mut context: &'ctx mut C) -> R {
//         (self)(
//             T1::from_context(&mut context),
//             T2::from_context(&mut context),
//             T3::from_context(&mut context),
//             T4::from_context(&mut context),
//             T5::from_context(&mut context),
//             T6::from_context(&mut context),
//             T7::from_context(&mut context),
//             T8::from_context(&mut context),
//         )
//     }
// }

pub fn invoke<'ctx, C: 'ctx, T, H, R>(context: &'ctx mut C, handler: H) -> R
where
    H: Handler<'ctx, C, T, R>,
{
    handler.call(context)
}

#[cfg(test)]
mod tests {

    

    // pub struct EditorCxt<'ctx> {
    //     param: &'ctx str,
    //     id: u32,
    // }

    // impl<'ctx> EditorCxt<'ctx> {
    //     pub fn new(param: &'ctx str, id: u32) -> Self {
    //         EditorCxt { param, id }
    //     }
    //     pub fn invoke<T, H, R>(self, handler: H) -> R
    //     where
    //         H: Handler<'ctx, Self, T, R>,
    //     {
    //         handler.call(self)
    //     }
    // }

    // pub struct Param<'s>(pub &'s str);

    // pub struct Id(pub u32);

    // impl<'ctx> FromContext<'ctx> for Param<'ctx> {
    //     type Context = EditorCxt<'ctx>;
    //     fn from_context(context: &Self::Context) -> Self {
    //         Param(context.param)
    //     }
    // }

    // impl<'ctx> FromContext<'ctx> for Id {
    //     type Context = EditorCxt<'ctx>;
    //     fn from_context(context: &Self::Context) -> Self {
    //         Id(context.id)
    //     }
    // }

    // #[test]
    // fn magic_params() {
    //     use super::*;

    //     let editor = EditorCxt::new("param_1".into(), 20);

    //     let handler_1 = |Param(param): Param, Id(id): Id| -> () {};

    //     editor.invoke(handler_1);
    // }
}

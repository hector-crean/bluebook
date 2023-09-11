use std::marker::PhantomData;

use crate::span::Span;

pub struct StringSpanIter<'spans> {
    phantom: PhantomData<&'spans ()>,
}

impl<'spans> StringSpanIter<'spans> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl<'spans> Iterator for StringSpanIter<'spans> {
    type Item = Span;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

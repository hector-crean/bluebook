use crate::span::{Attributes, SpanData, Spanslike};
use std::{ops::Range, sync::Arc};
pub use xi_rope::{spans::Spans, Delta, RopeDelta, RopeInfo};
use xi_rope::{
    spans::{SpanIter, SpansBuilder},
    Interval,
};

pub struct RopeSpans(Spans<Attributes>);

impl RopeSpans {
    pub fn new(rope: &xi_rope::Rope) -> Self {
        let sb = xi_rope::spans::SpansBuilder::<Attributes>::new(rope.len());
        let spans = sb.build();
        Self(spans)
    }
    pub fn add(&mut self, range: Range<usize>, data: Attributes) {
        let mut sb = SpansBuilder::new(range.len());

        sb.add_span(
            Interval {
                start: range.start,
                end: range.end,
            },
            data,
        );
        let added_annotations = sb.build();

        let merged = self.0.merge(&added_annotations, |a, b| match b {
            Some(b) => a.clone() + b.clone(),
            None => a.clone(),
        });

        self.0 = merged;
    }

    pub fn iter(&self) -> SpanIter<'_, Attributes> {
        Spans::<Attributes>::iter(&self.0)
    }
}

impl Spanslike for RopeSpans {
    type Delta = Delta<RopeInfo>;
    fn update(&mut self, delta: &Delta<RopeInfo>) {
        self.0.apply_shape(delta);
    }
}

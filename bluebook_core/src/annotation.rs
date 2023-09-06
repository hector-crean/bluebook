use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, ops::Range};
use string_cache::DefaultAtom;

/// The annotated text span.

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Behavior {
    /// When calculating the final state, it will keep all the ranges even if they have the same type
    ///
    /// For example, we would like to keep both comments alive even if they have overlapped regions
    AllowMultiple = 2,
    /// When calculating the final state, it will merge the ranges that have overlapped regions and have the same type
    ///
    /// For example, [bold 2~5] can be merged with [bold 1~4] to produce [bold 1-5]
    Merge = 0,
    /// It will delete the overlapped range that has smaller lamport && has the same type.
    /// But it will keep the `AllowMultiple` type unchanged
    Delete = 1,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Annotation {
    pub range: Range<usize>,
    pub behavior: Behavior,
    pub type_: DefaultAtom,
    pub value: Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub range: Range<usize>,
    pub attributes: HashMap<DefaultAtom, Value>,
}

// impl From<Vec<Annotation>> for Span {
//     fn from(value: Vec<Annotation>) -> Self {
//         todo!()
//     }
// }

pub trait Annotatable {
    /// Apply a given formatting to the specified range of text.
    fn apply_annotation(&mut self, annotation: Annotation);

    /// Get the formattings applied to the text at the given position.
    fn spans_at(&self, position: usize) -> HashMap<DefaultAtom, Value>;

    /// Get the formattings applied to the text within the given range.
    fn spans_in_range(&self, range: Range<usize>) -> Vec<Span>;
}

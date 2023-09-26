pub mod interval_tree;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::HashMap, ops::Range};
use string_cache::DefaultAtom;

use fxhash::FxHasher;
use std::hash::BuildHasherDefault;

use crate::buffer::TextBuffer;

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
pub struct SpanData {
    pub behavior: Behavior,
    pub type_: DefaultAtom,
    pub value: Value,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Span<T> {
    pub range: Range<usize>,
    pub data: T,
}

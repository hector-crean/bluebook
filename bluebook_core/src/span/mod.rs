pub mod interval_tree;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{HashMap},
    ops::Range,
};
use string_cache::DefaultAtom;

use fxhash::FxHasher;
use std::hash::BuildHasherDefault;





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

pub trait SpanIterable {
    /// Apply a given formatting to the specified range of text.
    // fn apply_annotation(&mut self, annotation: Annotation);

    /// Get the formattings applied to the text at the given position.
    // fn spans_at(&self, position: usize) -> HashMap<DefaultAtom, Value>;

    /// Get the formattings applied to the text within the given range.
    fn spans<Drain: Iterator<Item = Span>>(&self, range: Range<usize>) -> Drain;
}

/// A builder for default Fx hashers.
pub type FxBuildHasher = BuildHasherDefault<FxHasher>;

/// A `HashMap` using a default Fx hasher.
pub type FxHashMap<K, V> = HashMap<K, V, FxBuildHasher>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    // pub range: Range<usize>,
    pub insert: String,
    pub attributes: FxHashMap<DefaultAtom, Value>,
}

// pub struct SpansIterator<'a, Buffer: TextBuffer> {
//     buffer: &'a Buffer,
//     span_tree: IntervalTree<usize, HashMap<DefaultAtom, Value>>,
//     boundaries: Vec<usize>,
//     cursor: usize,
// }

// impl<'a, Buffer: TextBuffer> SpansIterator<'a, Buffer> {
//     fn new(
//         buffer: &'a Buffer,
//         span_tree: IntervalTree<usize, HashMap<DefaultAtom, Value>>,
//     ) -> Self {
//         let mut boundaries = BTreeSet::new();

//         for Range { start, end } in spans.iter().map(|span| span.range) {
//             boundaries.insert(start);
//             boundaries.insert(end);
//         }

//         Self {
//             boundaries: boundaries.into_iter().collect(),
//             cursor: 0,
//             buffer,
//             span_tree,
//         }
//     }
// }

// impl<'a, Buffer: TextBuffer> Iterator for SpansIterator<'a, Buffer> {
//     type Item = &'a [HashMap<DefaultAtom, Value>];

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.boundaries.len() <= self.cursor {
//             return None;
//         };

//         let start = self.boundaries[self.cursor];
//         let end = self.boundaries[self.cursor + 1];

//         let query = self.span_tree.query(start..end);

//         let items = query.map(|x| x.value).collect();

//         return Some(items);
//     }
// }

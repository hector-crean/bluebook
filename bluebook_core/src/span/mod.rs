pub mod augmented_avl_tree;
pub mod interval;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::{Add, Sub};
use std::{collections::HashMap, ops::Range};
use string_cache::DefaultAtom;
use xi_rope::{RopeDelta, Transformer};

pub trait Spanslike {
    type Delta;
    // fn add(&mut self, range: Range<usize>, data: Attributes) -> ();
    fn update(&mut self, delta: &Self::Delta) -> ();
}

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

enum InsertDrift {
    Inside,
    Outside,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attributes {
    pub attributes: HashMap<DefaultAtom, Value>,
}

impl Add for Attributes {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self.attributes.clone();
        for (key, value) in other.attributes {
            // overwriting the value if the key already exists.
            result.insert(key, value);
        }
        Attributes { attributes: result }
    }
}

impl Sub for Attributes {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = self.attributes.clone();
        for key in other.attributes.keys() {
            result.remove(key);
        }
        Attributes { attributes: result }
    }
}

// struct NaiveSpans(Vec<Span<SpanData>>);

// impl NaiveSpans {
//     pub fn apply_delta(&mut self, delta: &RopeDelta) -> () {
//         let mut transformer = Transformer::new(delta);

//         for Span { range, .. } in &mut self.0 {
//             let (new_start, new_end) = (
//                 transformer.transform(range.start, false),
//                 transformer.transform(range.end, true),
//             );
//             range.start = new_start;
//             range.end = new_end;
//         }
//     }
// }

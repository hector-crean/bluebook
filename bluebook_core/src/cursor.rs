use crate::mode::CursorMode;
use serde::{Deserialize, Serialize};

/// We have a concrete cursor struct, which holds information about the current (byte) offset, mode etc.
/// We have a variety of cursor traits, which are used to find the next cursor offset, given an
/// underlying text buffer, cursor mode etc.

enum CursorOrientation {
    Foward,
    Backward,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Copy)]
pub struct CursorRange {
    pub anchor: usize,
    pub head: usize,
}

impl CursorRange {
    pub fn new(anchor: usize, head: usize) -> Self {
        Self { anchor, head }
    }

    pub fn set(&mut self, range: Self) -> &Self {
        self.anchor = range.anchor;
        self.head = range.head;
        self
    }
    pub fn set_head(&mut self, offset: usize) -> &Self {
        self.head = offset;
        self
    }
    pub fn set_anchor(&mut self, offset: usize) -> &Self {
        self.anchor = offset;
        self
    }

    pub fn set_point(&mut self, offset: usize) -> &Self {
        self.anchor = offset;
        self.head = offset;

        self
    }
    /// Start of the range.
    #[inline]
    #[must_use]
    pub fn from(&self) -> usize {
        std::cmp::min(self.anchor, self.head)
    }

    /// End of the range.
    #[inline]
    #[must_use]
    pub fn to(&self) -> usize {
        std::cmp::max(self.anchor, self.head)
    }

    /// Total length of the range.
    #[inline]
    #[must_use]
    pub fn len(&self) -> usize {
        self.to() - self.from()
    }

    /// `true` when head and anchor are at the same position.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.anchor == self.head
    }

    /// `orientation::Backward` when head < anchor.
    /// `orientation::Backward` otherwise.
    #[inline]
    #[must_use]
    pub fn orientation(&self) -> CursorOrientation {
        if self.head < self.anchor {
            CursorOrientation::Backward
        } else {
            CursorOrientation::Forward
        }
    }

    /// Flips the orientation of the selection
    pub fn flip(&self) -> Self {
        Self {
            anchor: self.head,
            head: self.anchor,
        }
    }

    /// Returns the selection if it goes in the orientation of `orientation`,
    /// flipping the selection otherwise.
    pub fn with_orientation(self, orientation: CursorOrientation) -> Self {
        if self.orientation() == orientation {
            self
        } else {
            self.flip()
        }
    }

    /// Check two ranges for overlap.
    #[must_use]
    pub fn overlaps(&self, other: &Self) -> bool {
        // To my eye, it's non-obvious why this works, but I arrived
        // at it after transforming the slower version that explicitly
        // enumerated more cases.  The unit tests are thorough.
        self.from() == other.from() || (self.to() > other.from() && other.to() > self.from())
    }

    #[inline]
    pub fn contains_range(&self, other: &Self) -> bool {
        self.from() <= other.from() && self.to() >= other.to()
    }

    pub fn contains(&self, pos: usize) -> bool {
        self.from() <= pos && pos < self.to()
    }

    /// Extend the range to cover at least `from` `to`.
    #[must_use]
    pub fn extend(self, from: usize, to: usize) -> Self {
        debug_assert!(from <= to);

        if self.anchor <= self.head {
            Self {
                anchor: self.anchor.min(from),
                head: self.head.max(to),
            }
        } else {
            Self {
                anchor: self.anchor.max(to),
                head: self.head.min(from),
            }
        }
    }

    // groupAt
}

pub struct Cursor {
    mode: CursorMode,
    range: CursorRange,
}

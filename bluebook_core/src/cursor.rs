use crate::mode::CursorMode;

/// We have a concrete cursor struct, which holds information about the current (byte) offset, mode etc.
/// We have a variety of cursor traits, which are used to find the next cursor offset, given an
/// underlying text buffer, cursor mode etc.

/// A single selection range.
///
/// A range consists of an "anchor" and "head" position in
/// the text.  The head is the part that the user moves when
/// directly extending a selection.  The head and anchor
/// can be in any order, or even share the same position.
///
/// The anchor and head positions use gap indexing, meaning
/// that their indices represent the gaps *between* `char`s
/// rather than the `char`s themselves. For example, 1
/// represents the position between the first and second `char`.
///
/// Below are some examples of `Range` configurations.
/// The anchor and head indices are shown as "(anchor, head)"
/// tuples, followed by example text with "[" and "]" symbols
/// representing the anchor and head positions:
///
/// - (0, 3): `[Som]e text`.
/// - (3, 0): `]Som[e text`.
/// - (2, 7): `So[me te]xt`.
/// - (1, 1): `S[]ome text`.
///
/// Ranges are considered to be inclusive on the left and
/// exclusive on the right, regardless of anchor-head ordering.
/// This means, for example, that non-zero-width ranges that
/// are directly adjacent, sharing an edge, do not overlap.
/// However, a zero-width range will overlap with the shared
/// left-edge of another range.
///
/// By convention, user-facing ranges are considered to have
/// a block cursor on the head-side of the range that spans a
/// single grapheme inward from the range's edge.  There are a
/// variety of helper methods on `Range` for working in terms of
/// that block cursor, all of which have `cursor` in their name.
///
///

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum CursorOrientation {
    Forward,
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

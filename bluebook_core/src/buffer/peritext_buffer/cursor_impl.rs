use std::borrow::Cow;

// use unicode_segmentation::Graphemes;

// use super::grapheme::Graphemes;
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete, UnicodeSegmentation};

use crate::text_buffer_cursor::TextBufferCursor;

use crate::movement::Direction;

use crate::graphemes::UnicodeSegmentationError;

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

enum CursorEnd {
    Head,
    Tail,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct PeritextCursor<'a> {
    pub buffer: Cow<'a, str>,
    pub cursor_range: CursorRange,
}

// #[derive(Debug)]
// pub struct StringCursor<'a> {
//     text: &'a mut String,
//     position: usize,
// }

impl<'a> PeritextCursor<'a> {
    /// Create a new cursor.
    pub fn new(buffer: Cow<'a, str>) -> Self {
        Self {
            buffer,
            cursor_range: CursorRange::new(0, 0),
        }
    }
}

impl<'cursor> TextBufferCursor<'cursor> for PeritextCursor<'cursor> {
    fn anchor(&self) -> usize {
        self.cursor_range.anchor
    }
    fn head(&self) -> usize {
        self.cursor_range.head
    }
    fn set_anchor(mut self, byte_offset: usize) -> Self {
        self.cursor_range.anchor = byte_offset;
        self
    }
    fn set_head(mut self, byte_offset: usize) -> Self {
        self.cursor_range.head = byte_offset;
        self
    }

    fn set_point(mut self, byte_offset: usize) -> Self {
        self.cursor_range.head = byte_offset;
        self.cursor_range.anchor = byte_offset;

        self
    }

    fn range(&self) -> CursorRange {
        self.cursor_range
    }

    fn is_grapheme_boundary(&self) -> Result<bool, UnicodeSegmentationError> {
        // let graphemes = Graphemes::new(&self.buffer, false);

        let s = self.buffer.as_ref();

        let mut gc = GraphemeCursor::new(self.head(), s.len(), false);

        gc.is_boundary(s, 0)
            .map_err(UnicodeSegmentationError::GraphemeIncompleteError)
    }

    fn next_grapheme_boundary(&self) -> Result<Option<usize>, UnicodeSegmentationError> {
        let s = self.buffer.as_ref();

        let mut gc = GraphemeCursor::new(self.head(), s.len(), false);

        gc.next_boundary(s, 0)
            .map_err(UnicodeSegmentationError::GraphemeIncompleteError)
    }

    fn prev_grapheme_boundary(&self) -> Result<Option<usize>, UnicodeSegmentationError> {
        let s = self.buffer.as_ref();

        let mut gc = GraphemeCursor::new(self.head(), s.len(), false);

        gc.prev_boundary(s, 0)
            .map_err(UnicodeSegmentationError::GraphemeIncompleteError)
    }

    fn nth_next_grapheme_boundary(
        &self,
        n: usize,
    ) -> Result<Option<usize>, UnicodeSegmentationError> {
        let s = self.buffer.as_ref();

        let mut gc = GraphemeCursor::new(self.head(), s.len(), false);

        for _ in 0..n {
            let next_byte_offset = gc
                .next_boundary(s, 0)
                .map_err(UnicodeSegmentationError::GraphemeIncompleteError)?;

            match next_byte_offset {
                Some(next_byte_offset) => {
                    gc.set_cursor(next_byte_offset);

                    continue;
                }
                None => {
                    return Ok(None);
                }
            }
        }

        Ok(Some(gc.cur_cursor()))
    }
    fn nth_prev_grapheme_boundary(
        &self,
        n: usize,
    ) -> Result<Option<usize>, UnicodeSegmentationError> {
        let s = self.buffer.as_ref();

        let mut gc = GraphemeCursor::new(self.head(), s.len(), false);

        for _ in 0..n {
            let prev_byte_offset = gc
                .prev_boundary(s, 0)
                .map_err(UnicodeSegmentationError::GraphemeIncompleteError)?;

            match prev_byte_offset {
                Some(prev_byte_offset) => {
                    gc.set_cursor(prev_byte_offset);
                    continue;
                }
                None => {
                    return Ok(None);
                }
            }
        }

        Ok(Some(gc.cur_cursor()))
    }
}

pub fn len_utf8_from_first_byte(b: u8) -> usize {
    match b {
        b if b < 0x80 => 1,
        b if b < 0xe0 => 2,
        b if b < 0xf0 => 3,
        _ => 4,
    }
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

    /// `Direction::Backward` when head < anchor.
    /// `Direction::Backward` otherwise.
    #[inline]
    #[must_use]
    pub fn direction(&self) -> Direction {
        if self.head < self.anchor {
            Direction::Backward
        } else {
            Direction::Forward
        }
    }

    /// Flips the direction of the selection
    pub fn flip(&self) -> Self {
        Self {
            anchor: self.head,
            head: self.anchor,
        }
    }

    /// Returns the selection if it goes in the direction of `direction`,
    /// flipping the selection otherwise.
    pub fn with_direction(self, direction: Direction) -> Self {
        if self.direction() == direction {
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

#[cfg(test)]
mod tests {
    use crate::{
        buffer::peritext_buffer::buffer_impl::Peritext, error::TextBufferWithCursorError,
        text_buffer::TextBuffer, text_buffer_cursor::CursorDocCoords,
    };

    use super::*;

    const TEXT: &str = &"Hello \nworld\n\n";
    const EMOJI: &str = "😀👋🌍";
    const COMPLEX_EMOJI: &str = "👨‍👩‍👧‍👦";

    fn peritext_buffer(s: &str) -> Peritext {
        let mut buffer = Peritext::new(1);
        let _ = buffer.write(0, s);

        buffer
    }

    #[test]
    fn test_cursor_coords_single_line() -> Result<(), TextBufferWithCursorError> {
        let mut buf = peritext_buffer(TEXT);

        let c = CursorRange { anchor: 7, head: 7 };
        let doc_coords = buf.cursor_coords(c)?;

        assert_eq!(doc_coords, CursorDocCoords::new(0, 7));

        Ok(())
    }

    #[test]
    fn test_cursor_coords_multiple_lines_middle() {}

    #[test]
    fn test_cursor_coords_multiple_lines_end() {}

    #[test]
    fn test_cursor_coords_at_start() {}

    #[test]
    fn test_cursor_coords_past_end() {}

    // Add more test cases as needed
}

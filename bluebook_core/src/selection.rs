use crate::cursor::{self, CursorRange};

/// Indicate whether a delta should be applied inside, outside non-caret selection or
/// after a caret selection (see [`Selection::apply_delta`].
#[derive(Copy, Clone)]
pub enum InsertDrift {
    /// Indicates this edit should happen within any (non-caret) selections if possible.
    Inside,
    /// Indicates this edit should happen outside any selections if possible.
    Outside,
    /// Indicates to do whatever the `after` bool says to do
    Default,
}

/// A selection holding one or more [`SelRegion`].
#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Selection {
    cursor_ranges: Vec<CursorRange>,
    last_inserted: usize,
}

impl AsRef<Selection> for Selection {
    fn as_ref(&self) -> &Selection {
        self
    }
}

impl Selection {
    pub fn new() -> Selection {
        Selection {
            cursor_ranges: Vec::new(),
            last_inserted: 0,
        }
    }

    pub fn add_curor_range(cursor_range: CursorRange) -> Selection {}

    pub fn contains(&self, offset: usize) -> bool {
        for cursor_range in self.cursor_ranges.iter() {
            if cursor_range.contains(offset) {
                return true;
            }
        }
        false
    }

    pub fn cursor_ranges(&self) -> &[CursorRange] {
        &self.cursor_ranges
    }

    /// A [`Selection`] is considered to be a caret if it contains
    /// only caret [`SelRegion`] (see [`SelRegion::is_caret`])
    pub fn is_point(&self) -> bool {
        self.cursor_ranges.iter().all(|r| r.is_empty())
    }
}

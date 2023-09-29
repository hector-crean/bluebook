use crate::cursor::CursorRange;

/// A selection holding one or more [`SelRegion`].
#[derive(Clone, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub struct Selection {
    regions: Vec<CursorRange>,
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
            regions: Vec::new(),
            last_inserted: 0,
        }
    }

    pub fn contains(&self, offset: usize) -> bool {
        for region in self.regions.iter() {
            if region.contains(offset) {
                return true;
            }
        }
        false
    }

    pub fn cursor_ranges(&self) -> &[CursorRange] {
        &self.regions
    }
}

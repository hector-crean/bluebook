use xi_rope::{RopeDelta, Transformer};

use crate::{
    cursor::CursorRange,
    selection::{InsertDrift, Selection},
};

impl Selection {
    pub fn apply_delta(&self, delta: &RopeDelta, after: bool, drift: InsertDrift) -> Selection {
        let mut result = Selection::new();
        let mut transformer = Transformer::new(delta);
        for cursor_range in self.cursor_ranges() {
            let is_region_forward = cursor_range.anchor < cursor_range.head;

            let (start_after, end_after) = match (drift, cursor_range.is_empty()) {
                (InsertDrift::Inside, false) => (!is_region_forward, is_region_forward),
                (InsertDrift::Outside, false) => (is_region_forward, !is_region_forward),
                _ => (after, after),
            };

            let new_range = CursorRange::new(
                transformer.transform(cursor_range.anchor, start_after),
                transformer.transform(cursor_range.head, end_after),
            );
            result.add_curor_range(new_range);
        }
        result
    }
}

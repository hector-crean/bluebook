use xi_rope::RopeDelta;

use crate::selection::Selection;

impl Selection {
    // pub fn apply_delta(&self, delta: &RopeDelta, after: bool, drift: InsertDrift) -> Selection {
    //     let mut result = Selection::new();
    //     let mut transformer = Transformer::new(delta);
    //     for region in self.regions() {
    //         let is_region_forward = region.start < region.end;

    //         let (start_after, end_after) = match (drift, region.is_caret()) {
    //             (InsertDrift::Inside, false) => (!is_region_forward, is_region_forward),
    //             (InsertDrift::Outside, false) => (is_region_forward, !is_region_forward),
    //             _ => (after, after),
    //         };

    //         let new_region = SelRegion::new(
    //             transformer.transform(region.start, start_after),
    //             transformer.transform(region.end, end_after),
    //             None,
    //         );
    //         result.add_region(new_region);
    //     }
    //     result
    // }
}

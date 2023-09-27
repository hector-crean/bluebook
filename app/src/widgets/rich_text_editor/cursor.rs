use bluebook_core::position::Position;
use egui::{epaint::text::Row, Galley, Vec2};
use std::cmp::Ordering::Equal;

/// Determines the `Position` of a cursor based on a given visual coordinate within a galley.
///
/// # Arguments
///
/// * `galley`: A reference to the Galley object, representing formatted text.
/// * `cursor_visual`: The visual coordinate representing the point of interest in the Galley.
///
/// # Returns
///
/// * A `Position` object representing the cursor’s position within the document.
pub fn cursor_from_visual_position(galley: &Galley, cursor_visual: Vec2) -> Position {
    // Enumerating over galley rows to find the closest row based on the y-coordinate of the cursor_visual
    let (row_nr, row, dist) = galley
        .rows
        .iter()
        .enumerate()
        .map(|(idx, row)| (idx, row, y_distance(row, cursor_visual.y)))
        // .min_by_key(|&(_, _, dist)| FloatOrd(dist)) could be used if using a crate like float-ord
        .min_by(|&(_, _, dist1), &(_, _, dist2)| dist1.partial_cmp(&dist2).unwrap_or(Equal))
        .expect("Galley rows cannot be empty"); // Providing a more descriptive panic message for safe unwrap.

    tracing::info!("Closest row to cursor visual: {:?}", &dist);

    // Extracting the character within the closest row based on the x-coordinate of the cursor_visual
    Position {
        line: row_nr,
        character: row.char_at(cursor_visual.x),
    }
}

/// Calculates the minimum vertical distance between a `Row` and a y-coordinate.
///
/// # Arguments
///
/// * `row`: A reference to a Row object.
/// * `y`: The y-coordinate representing a point in the vertical axis.
///
/// # Returns
///
/// * A floating-point value representing the minimum vertical distance.
fn y_distance(row: &Row, y: f32) -> f32 {
    (row.min_y() - y).abs().min((row.max_y() - y).abs())
}

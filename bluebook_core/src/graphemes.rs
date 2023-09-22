use unicode_segmentation::GraphemeIncomplete;

#[derive(Debug, thiserror::Error)]
pub enum UnicodeSegmentationError {
    #[error("GraphemeIncomplete")]
    GraphemeIncompleteError(GraphemeIncomplete),
}

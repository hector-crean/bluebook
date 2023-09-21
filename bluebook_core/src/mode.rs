use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MotionMode {
    Delete { count: usize },
    Yank { count: usize },
    Indent,
    Outdent,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum CursorMode {
    Normal(usize),
    // Insert(Selection),
}

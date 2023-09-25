#[derive(Clone, Copy, PartialEq, Debug, serde::Serialize, serde::Deserialize)]
pub enum ColPosition {
    FirstNonBlank,
    Start,
    End,
    Col(usize),
}

pub enum RowPosition {
    Start,
    End,
    Row(usize),
}

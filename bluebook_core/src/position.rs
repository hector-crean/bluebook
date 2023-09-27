#[derive(
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Copy,
    Clone,
    Default,
    serde::Deserialize,
    serde::Serialize,
)]
pub struct Position {
    pub line: usize,
    pub character: usize,
}

impl Position {
    pub const fn new(line: usize, character: usize) -> Self {
        Self { line, character }
    }

    pub const fn is_zero(self) -> bool {
        self.line == 0 && self.character == 0
    }
}

impl From<(usize, usize)> for Position {
    fn from((line, character): (usize, usize)) -> Self {
        Self { line, character }
    }
}

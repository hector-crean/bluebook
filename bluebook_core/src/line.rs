pub struct LineWithEnding<'s> {
    input: &'s str,
}

impl<'s> LineWithEnding<'s> {
    pub fn new(s: &'s str) -> Self {
        Self { input: s }
    }
}

impl<'s> Iterator for LineWithEnding<'s> {
    type Item = &'s str;
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() {
            return None;
        }
        let split = self
            .input
            .find('\n')
            .map(|i| i + 1)
            .unwrap_or_else(|| self.input.len());
        let (line, rest) = self.input.split_at(split);
        self.input = rest;
        Some(line)
    }
}

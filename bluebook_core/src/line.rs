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

#[cfg(test)]
mod tests {
    use super::LineWithEnding;

    #[test]
    fn test_empty_string() {
        let s = "";
        let str_array: Vec<&str> = vec![];

        let lines: Vec<&str> = LineWithEnding::new(s).collect();
        assert_eq!(lines, str_array);
    }

    #[test]
    fn test_single_line() {
        let s = "Hello, world!";
        let lines: Vec<_> = LineWithEnding::new(s).collect();
        assert_eq!(lines, ["Hello, world!"]);
    }

    #[test]
    fn test_lines_with_newline() {
        let s = "Line 1\nLine 2\nLine 3\n";
        let lines: Vec<_> = LineWithEnding::new(s).collect();
        assert_eq!(lines, ["Line 1\n", "Line 2\n", "Line 3\n"]);
    }

    #[test]
    fn test_lines_without_newline() {
        let s = "Line 1\nLine 2";
        let lines: Vec<_> = LineWithEnding::new(s).collect();
        assert_eq!(lines, ["Line 1\n", "Line 2"]);
    }

    #[test]
    fn test_empty_lines() {
        let s = "\n\n\n";
        let lines: Vec<_> = LineWithEnding::new(s).collect();
        assert_eq!(lines, ["\n", "\n", "\n"]);
    }
}

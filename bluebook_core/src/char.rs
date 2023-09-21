pub struct CharIter<'s> {
    slice: &'s str,
    start: usize,
    end: usize,
}

impl<'s> CharIter<'s> {
    pub fn new(slice: &'s str) -> CharIter<'s> {
        CharIter {
            slice,
            start: 0,
            end: slice.len(),
        }
    }
}

impl<'s> Iterator for CharIter<'s> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let ch = self.slice[self.start..].chars().next().unwrap();
            self.start += ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }
}

impl<'s> DoubleEndedIterator for CharIter<'s> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.start < self.end {
            let slice = &self.slice[..self.end - self.start];
            let ch = slice.chars().rev().next().unwrap();
            self.end -= ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }
}

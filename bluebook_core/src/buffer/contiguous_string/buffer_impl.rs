use super::cursor_impl::StringCursor;
use crate::{
    text_buffer::{TextBuffer, TextBufferError},
    text_buffer_cursor::TextBufferCursor,
};
use std::{
    borrow::{BorrowMut, Cow},
    ops::{Range, RangeBounds},
};
use unicode_segmentation::{GraphemeCursor, GraphemeIncomplete, UnicodeSegmentation};

// impl<'a, T: TextBuffer> Iterator for Drain<'a, T>
// where
//     T: AsMut<str>, // Allow us to work directly with the string data
// {
//     type Item = char;

//     fn next(&mut self) -> Option<Self::Item> {
//         if self.range.start < self.range.end {
//             let current_char = self
//                 .text_buffer
//                 .as_mut()
//                 .chars()
//                 .nth(self.range.start)
//                 .expect("Invalid char boundary");

//             // Remove the char from the text buffer
//             self.text_buffer.as_mut().replace_range(
//                 self.range.start..self.range.start + current_char.len_utf8(),
//                 "",
//             );

//             // Update the range end to reflect the change in length
//             self.range.end -= current_char.len_utf8();

//             Some(current_char)
//         } else {
//             None
//         }
//     }
// }

impl TextBuffer for String {
    type Cursor<'cursor> = StringCursor<'cursor> where Self:'cursor;

    fn cursor<'cursor>(&'cursor self, position: usize) -> Option<Self::Cursor<'cursor>> {
        let new_cursor = StringCursor {
            text: self,
            position,
        };

        if new_cursor.is_boundary() {
            Some(new_cursor)
        } else {
            None
        }
    }

    fn write(&mut self, offset: usize, s: &str) -> Result<(), TextBufferError> {
        self.insert_str(offset, s);
        Ok(())
    }

    fn replace_range<R>(&mut self, range: R, replace_with: &str)
    where
        R: RangeBounds<usize>,
    {
        self.replace_range(range, replace_with)
    }
    // fn edit(&mut self, range: Range<usize>, new: impl Into<String>) {
    //     self.replace_range(range, &new.into());
    // }

    fn take(&self) -> Cow<str> {
        self.as_str().into()
    }

    fn slice(&self, range: Range<usize>) -> Option<Cow<str>> {
        self.get(range).map(Cow::from)
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn prev_grapheme_offset(&self, from: usize) -> Option<usize> {
        let mut c = GraphemeCursor::new(from, self.len(), true);
        c.prev_boundary(self, 0).unwrap()
    }

    fn next_grapheme_offset(&self, from: usize) -> Option<usize> {
        let mut c = GraphemeCursor::new(from, self.len(), true);
        c.next_boundary(self, 0).unwrap()
    }

    fn prev_codepoint_offset(&self, from: usize) -> Option<usize> {
        let mut c = self.cursor(from).unwrap();
        c.prev()
    }

    fn next_codepoint_offset(&self, from: usize) -> Option<usize> {
        let mut c = self.cursor(from).unwrap();
        if c.next().is_some() {
            Some(c.pos())
        } else {
            None
        }
    }

    fn prev_word_offset(&self, from: usize) -> Option<usize> {
        let mut offset = from;
        let mut passed_alphanumeric = false;
        for prev_grapheme in self.get(0..from)?.graphemes(true).rev() {
            let is_alphanumeric = prev_grapheme.chars().next()?.is_alphanumeric();
            if is_alphanumeric {
                passed_alphanumeric = true;
            } else if passed_alphanumeric {
                return Some(offset);
            }
            offset -= prev_grapheme.len();
        }
        None
    }

    fn next_word_offset(&self, from: usize) -> Option<usize> {
        let mut offset = from;
        let mut passed_alphanumeric = false;
        for next_grapheme in self.get(from..)?.graphemes(true) {
            let is_alphanumeric = next_grapheme.chars().next()?.is_alphanumeric();
            if is_alphanumeric {
                passed_alphanumeric = true;
            } else if passed_alphanumeric {
                return Some(offset);
            }
            offset += next_grapheme.len();
        }
        Some(self.len())
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn from_str(s: &str) -> Self {
        s.to_string()
    }

    fn preceding_line_break(&self, from: usize) -> usize {
        let mut offset = from;

        for byte in self.get(0..from).unwrap_or("").bytes().rev() {
            if byte == 0x0a {
                return offset;
            }
            offset -= 1;
        }

        0
    }

    fn next_line_break(&self, from: usize) -> usize {
        let mut offset = from;

        for char in self.get(from..).unwrap_or("").bytes() {
            if char == 0x0a {
                return offset;
            }
            offset += 1;
        }

        self.len()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn replace() {
        let mut buf = String::from("hello world");

        buf.replace_range(1..9, "era");

        assert_eq!("herald", buf);
    }

    #[test]
    fn prev_codepoint_offset() {
        let a = String::from("a\u{00A1}\u{4E00}\u{1F4A9}");
        assert_eq!(Some(6), a.prev_codepoint_offset(10));
        assert_eq!(Some(3), a.prev_codepoint_offset(6));
        assert_eq!(Some(1), a.prev_codepoint_offset(3));
        assert_eq!(Some(0), a.prev_codepoint_offset(1));
        assert_eq!(None, a.prev_codepoint_offset(0));
        let b = a.slice(1..10).unwrap().to_string();
        assert_eq!(Some(5), b.prev_codepoint_offset(9));
        assert_eq!(Some(2), b.prev_codepoint_offset(5));
        assert_eq!(Some(0), b.prev_codepoint_offset(2));
        assert_eq!(None, b.prev_codepoint_offset(0));
    }

    #[test]
    fn next_codepoint_offset() {
        let a = String::from("a\u{00A1}\u{4E00}\u{1F4A9}");
        assert_eq!(Some(10), a.next_codepoint_offset(6));
        assert_eq!(Some(6), a.next_codepoint_offset(3));
        assert_eq!(Some(3), a.next_codepoint_offset(1));
        assert_eq!(Some(1), a.next_codepoint_offset(0));
        assert_eq!(None, a.next_codepoint_offset(10));
        let b = a.slice(1..10).unwrap().to_string();
        assert_eq!(Some(9), b.next_codepoint_offset(5));
        assert_eq!(Some(5), b.next_codepoint_offset(2));
        assert_eq!(Some(2), b.next_codepoint_offset(0));
        assert_eq!(None, b.next_codepoint_offset(9));
    }

    #[test]
    fn prev_next() {
        let mut buf = String::from("abc");
        let mut cursor = buf.cursor(0).unwrap();

        assert_eq!(cursor.next(), Some(0));
        assert_eq!(cursor.next(), Some(1));
        assert_eq!(cursor.prev(), Some(1));
        assert_eq!(cursor.next(), Some(1));
        assert_eq!(cursor.next(), Some(2));
    }

    #[test]
    fn peek_next_codepoint() {
        let mut inp = String::from("$¢€£💶");
        let mut cursor = inp.cursor(0).unwrap();
        assert_eq!(cursor.peek_next_codepoint(), Some('$'));
        assert_eq!(cursor.peek_next_codepoint(), Some('$'));
        assert_eq!(cursor.next_codepoint(), Some('$'));
        assert_eq!(cursor.peek_next_codepoint(), Some('¢'));
        assert_eq!(cursor.prev_codepoint(), Some('$'));
        assert_eq!(cursor.peek_next_codepoint(), Some('$'));
        assert_eq!(cursor.next_codepoint(), Some('$'));
        assert_eq!(cursor.next_codepoint(), Some('¢'));
        assert_eq!(cursor.peek_next_codepoint(), Some('€'));
        assert_eq!(cursor.next_codepoint(), Some('€'));
        assert_eq!(cursor.peek_next_codepoint(), Some('£'));
        assert_eq!(cursor.next_codepoint(), Some('£'));
        assert_eq!(cursor.peek_next_codepoint(), Some('💶'));
        assert_eq!(cursor.next_codepoint(), Some('💶'));
        assert_eq!(cursor.peek_next_codepoint(), None);
        assert_eq!(cursor.next_codepoint(), None);
        assert_eq!(cursor.peek_next_codepoint(), None);
    }

    #[test]
    fn prev_grapheme_offset() {
        // A with ring, hangul, regional indicator "US"
        let a = String::from("A\u{030a}\u{110b}\u{1161}\u{1f1fa}\u{1f1f8}");
        assert_eq!(Some(9), a.prev_grapheme_offset(17));
        assert_eq!(Some(3), a.prev_grapheme_offset(9));
        assert_eq!(Some(0), a.prev_grapheme_offset(3));
        assert_eq!(None, a.prev_grapheme_offset(0));
    }

    #[test]
    fn next_grapheme_offset() {
        // A with ring, hangul, regional indicator "US"
        let a = String::from("A\u{030a}\u{110b}\u{1161}\u{1f1fa}\u{1f1f8}");
        assert_eq!(Some(3), a.next_grapheme_offset(0));
        assert_eq!(Some(9), a.next_grapheme_offset(3));
        assert_eq!(Some(17), a.next_grapheme_offset(9));
        assert_eq!(None, a.next_grapheme_offset(17));
    }

    #[test]
    fn prev_word_offset() {
        let a = String::from("Technically a word: ৬藏A\u{030a}\u{110b}\u{1161}");
        assert_eq!(Some(20), a.prev_word_offset(35));
        assert_eq!(Some(20), a.prev_word_offset(27));
        assert_eq!(Some(20), a.prev_word_offset(23));
        assert_eq!(Some(14), a.prev_word_offset(20));
        assert_eq!(Some(14), a.prev_word_offset(19));
        assert_eq!(Some(12), a.prev_word_offset(13));
        assert_eq!(None, a.prev_word_offset(12));
        assert_eq!(None, a.prev_word_offset(11));
        assert_eq!(None, a.prev_word_offset(0));
    }

    #[test]
    fn next_word_offset() {
        let a = String::from("Technically a word: ৬藏A\u{030a}\u{110b}\u{1161}");
        assert_eq!(Some(11), a.next_word_offset(0));
        assert_eq!(Some(11), a.next_word_offset(7));
        assert_eq!(Some(13), a.next_word_offset(11));
        assert_eq!(Some(18), a.next_word_offset(14));
        assert_eq!(Some(35), a.next_word_offset(18));
        assert_eq!(Some(35), a.next_word_offset(19));
        assert_eq!(Some(35), a.next_word_offset(20));
        assert_eq!(Some(35), a.next_word_offset(26));
        assert_eq!(Some(35), a.next_word_offset(35));
    }

    #[test]
    fn preceding_line_break() {
        let a = String::from("Technically\na word:\n ৬藏A\u{030a}\n\u{110b}\u{1161}");
        assert_eq!(0, a.preceding_line_break(0));
        assert_eq!(0, a.preceding_line_break(11));
        assert_eq!(12, a.preceding_line_break(12));
        assert_eq!(12, a.preceding_line_break(13));
        assert_eq!(20, a.preceding_line_break(21));
        assert_eq!(31, a.preceding_line_break(31));
        assert_eq!(31, a.preceding_line_break(34));

        let b = String::from("Technically a word: ৬藏A\u{030a}\u{110b}\u{1161}");
        assert_eq!(0, b.preceding_line_break(0));
        assert_eq!(0, b.preceding_line_break(11));
        assert_eq!(0, b.preceding_line_break(13));
        assert_eq!(0, b.preceding_line_break(21));
    }

    #[test]
    fn next_line_break() {
        let a = String::from("Technically\na word:\n ৬藏A\u{030a}\n\u{110b}\u{1161}");
        assert_eq!(11, a.next_line_break(0));
        assert_eq!(11, a.next_line_break(11));
        assert_eq!(19, a.next_line_break(13));
        assert_eq!(30, a.next_line_break(21));
        assert_eq!(a.len(), a.next_line_break(31));

        let b = String::from("Technically a word: ৬藏A\u{030a}\u{110b}\u{1161}");
        assert_eq!(b.len(), b.next_line_break(0));
        assert_eq!(b.len(), b.next_line_break(11));
        assert_eq!(b.len(), b.next_line_break(13));
        assert_eq!(b.len(), b.next_line_break(19));
    }
}

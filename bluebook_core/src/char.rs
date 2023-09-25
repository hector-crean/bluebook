use crate::text_buffer::TextBuffer;

pub struct CharIter<'s> {
    slice: &'s str,
    index: usize,
}

impl<'s> CharIter<'s> {
    pub fn new(slice: &'s str) -> CharIter<'s> {
        CharIter { slice, index: 0 }
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
            let ch = slice.chars().next_back().unwrap();
            self.end -= ch.len_utf8();
            Some(ch)
        } else {
            None
        }
    }
}

/// Determine whether a character is a line ending.
#[inline]
pub fn char_is_line_ending(ch: char) -> bool {
    matches!(ch, '\u{000A}')
}

/// Determine whether a character qualifies as (non-line-break)
/// whitespace.
#[inline]
pub fn char_is_whitespace(ch: char) -> bool {
    // TODO: this is a naive binary categorization of whitespace
    // characters.  For display, word wrapping, etc. we'll need a better
    // categorization based on e.g. breaking vs non-breaking spaces
    // and whether they're zero-width or not.
    match ch {
        //'\u{1680}' | // Ogham Space Mark (here for completeness, but usually displayed as a dash, not as whitespace)
        '\u{0009}' | // Character Tabulation
        '\u{0020}' | // Space
        '\u{00A0}' | // No-break Space
        '\u{180E}' | // Mongolian Vowel Separator
        '\u{202F}' | // Narrow No-break Space
        '\u{205F}' | // Medium Mathematical Space
        '\u{3000}' | // Ideographic Space
        '\u{FEFF}'   // Zero Width No-break Space
        => true,

        // En Quad, Em Quad, En Space, Em Space, Three-per-em Space,
        // Four-per-em Space, Six-per-em Space, Figure Space,
        // Punctuation Space, Thin Space, Hair Space, Zero Width Space.
        ch if ('\u{2000}' ..= '\u{200B}').contains(&ch) => true,

        _ => false,
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CharCursorError {
    #[error("Invalid character encountered")]
    InvalidCharacter,
    // Add more error variants as needed
}

///Single Unicode Character:
/// In Rust, a char represents a single Unicode character. It's a 32-bit (4-byte) value that can hold
/// any valid Unicode code point. This means that a char can represent characters from the basic
/// multilingual plane (BMP) and characters outside the BMP.
///Code Point Alignment: Each char in Rust corresponds to a single Unicode code point. When you work
/// with char values, you are dealing with characters at code point boundaries. This ensures that a
/// char represents a complete and valid Unicode character, and it won't be split across multiple code
/// points.
///
pub trait CharCursor<'buffer> {
    type Buffer: TextBuffer;
    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self;
}

use crate::buffer::TextBuffer;

/// Describe char classifications used to compose word boundaries
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CharClassification {
    /// Carriage Return (`r`)
    Cr,
    /// Line feed (`\n`)
    Lf,
    /// Whitespace character
    Space,
    /// Any punctuation character
    Punctuation,
    /// Includes letters and all of non-ascii unicode
    Other,
}

impl CharClassification {
    /// Return the [`CharClassification`] of the input character
    pub fn new(codepoint: char) -> Self {
        match codepoint {
            '\r' => CharClassification::Cr,
            '\n' => CharClassification::Lf,
            _ => CharClassification::Other,
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
pub enum CodepointCursorError {
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
pub trait CodepointCursor<'buffer>:
    Iterator<Item = usize> + DoubleEndedIterator<Item = usize>
{
    type Buffer: TextBuffer;
    fn new(text: &'buffer Self::Buffer, pos: usize) -> Self;
    fn offset(&self) -> usize;
}

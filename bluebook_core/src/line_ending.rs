use std::ops::Range;

use crate::text_buffer::{self, TextBuffer};

#[cfg(target_os = "windows")]
pub const NATIVE_LINE_ENDING: LineEnding = LineEnding::Crlf;
#[cfg(not(target_os = "windows"))]
pub const NATIVE_LINE_ENDING: LineEnding = LineEnding::LF;

/// Represents one of the valid Unicode line endings.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum LineEnding {
    Crlf, // CarriageReturn followed by LineFeed
    LF,   // U+000A -- LineFeed
    #[cfg(feature = "unicode-lines")]
    VT, // U+000B -- VerticalTab
    #[cfg(feature = "unicode-lines")]
    FF, // U+000C -- FormFeed
    #[cfg(feature = "unicode-lines")]
    CR, // U+000D -- CarriageReturn
    #[cfg(feature = "unicode-lines")]
    Nel, // U+0085 -- NextLine
    #[cfg(feature = "unicode-lines")]
    LS, // U+2028 -- Line Separator
    #[cfg(feature = "unicode-lines")]
    PS, // U+2029 -- ParagraphSeparator
}

impl LineEnding {
    #[inline]
    pub const fn len_chars(&self) -> usize {
        match self {
            Self::Crlf => 2,
            _ => 1,
        }
    }

    #[inline]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Crlf => "\u{000D}\u{000A}",
            Self::LF => "\u{000A}",
            #[cfg(feature = "unicode-lines")]
            Self::VT => "\u{000B}",
            #[cfg(feature = "unicode-lines")]
            Self::FF => "\u{000C}",
            #[cfg(feature = "unicode-lines")]
            Self::CR => "\u{000D}",
            #[cfg(feature = "unicode-lines")]
            Self::Nel => "\u{0085}",
            #[cfg(feature = "unicode-lines")]
            Self::LS => "\u{2028}",
            #[cfg(feature = "unicode-lines")]
            Self::PS => "\u{2029}",
        }
    }

    #[inline]
    pub const fn from_char(ch: char) -> Option<LineEnding> {
        match ch {
            '\u{000A}' => Some(LineEnding::LF),
            #[cfg(feature = "unicode-lines")]
            '\u{000B}' => Some(LineEnding::VT),
            #[cfg(feature = "unicode-lines")]
            '\u{000C}' => Some(LineEnding::FF),
            #[cfg(feature = "unicode-lines")]
            '\u{000D}' => Some(LineEnding::CR),
            #[cfg(feature = "unicode-lines")]
            '\u{0085}' => Some(LineEnding::Nel),
            #[cfg(feature = "unicode-lines")]
            '\u{2028}' => Some(LineEnding::LS),
            #[cfg(feature = "unicode-lines")]
            '\u{2029}' => Some(LineEnding::PS),
            // Not a line ending
            _ => None,
        }
    }

    // Normally we'd want to implement the FromStr trait, but in this case
    // that would force us into a different return type than from_char or
    // or from_rope_slice, which would be weird.
    #[allow(clippy::should_implement_trait)]
    // #[inline]
    pub fn from_str(g: &str) -> Option<LineEnding> {
        match g {
            "\u{000D}\u{000A}" => Some(LineEnding::Crlf),
            "\u{000A}" => Some(LineEnding::LF),
            #[cfg(feature = "unicode-lines")]
            "\u{000B}" => Some(LineEnding::VT),
            #[cfg(feature = "unicode-lines")]
            "\u{000C}" => Some(LineEnding::FF),
            #[cfg(feature = "unicode-lines")]
            "\u{000D}" => Some(LineEnding::CR),
            #[cfg(feature = "unicode-lines")]
            "\u{0085}" => Some(LineEnding::Nel),
            #[cfg(feature = "unicode-lines")]
            "\u{2028}" => Some(LineEnding::LS),
            #[cfg(feature = "unicode-lines")]
            "\u{2029}" => Some(LineEnding::PS),
            // Not a line ending
            _ => None,
        }
    }

    // #[inline]
    // pub fn from_text_buffer_slice<B: TextBuffer>(
    //     buffer: B,
    //     range: Range<usize>,
    // ) -> Option<LineEnding> {
    //     let slice = buffer.slice(range);

    //     match slice {
    //         Some(ref slice) => LineEnding::from_str(slice),
    //         None => None,
    //     }
    // }
}

#[inline]
pub fn str_is_line_ending(s: &str) -> bool {
    LineEnding::from_str(s).is_some()
}

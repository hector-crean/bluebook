pub mod citation;

pub use citation::Citation;
use egui::{Align, Color32, FontId, Stroke, TextFormat};
use serde_json::{json, Value};
use string_cache::{Atom, EmptyStaticAtomSet};

#[derive(
    strum::Display,
    // strum::EnumString,
    // strum::EnumIter,
    Clone,
    PartialEq,
    Eq,
    Debug,
    strum::EnumMessage,
    strum::IntoStaticStr,
)]
pub enum Formatting {
    Bold,
    NotBold,
    Italic,
    NotItalic,
    // Underline,
    // NotUnderline,
    // StrikeThrough,
    // NotStrikeThrough,
    // FontSize(u32),
    // FontColor(u8, u8, u8), // RGB
    Link { url: String },
    NotLink,
    // Citation(Citation),
    Comment(String),
    // Unknown,
}

// impl From<Formatting> for peritext::Style {
//     fn from(value: Formatting) -> Self {
//         match value {
//             Formatting::Bold => peritext::Style::new_bold_like(value.atom(), json!(0)),
//             Formatting::NotBold => peritext::Style::new_erase_bold_like(value.atom()),
//             Formatting::Italic => peritext::Style::new_bold_like(value.atom(), json!(0)),
//             Formatting::NotItalic => peritext::Style::new_erase_bold_like(value.atom()),
//             Formatting::Link { ref url } => {
//                 peritext::Style::new_link_like(value.atom(), json!(url))
//             }
//             Formatting::NotLink => peritext::Style::new_erase_link_like(value.atom()),
//             Formatting::Comment(ref comment) => {
//                 peritext::Style::new_comment_like(value.atom(), json!(comment))
//             }
//         }
//     }
// }

#[derive(Clone)]
pub struct TextFormatBuilder {
    font_id: Option<egui::FontId>,
    color: Option<egui::Color32>,
    background: Option<egui::Color32>,
    italics: Option<bool>,
    underline: Option<egui::Stroke>,
    strikethrough: Option<egui::Stroke>,
    valign: Option<egui::Align>,
}

impl TextFormatBuilder {
    pub fn new() -> Self {
        TextFormatBuilder {
            font_id: None,
            color: None,
            background: None,
            italics: None,
            underline: None,
            strikethrough: None,
            valign: None,
        }
    }

    pub fn font_id(mut self, font_id: FontId) -> Self {
        self.font_id = Some(font_id);
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }

    pub fn background(mut self, background: Color32) -> Self {
        self.background = Some(background);
        self
    }

    pub fn italics(mut self, italics: bool) -> Self {
        self.italics = Some(italics);
        self
    }

    pub fn underline(mut self, underline: Stroke) -> Self {
        self.underline = Some(underline);
        self
    }

    pub fn strikethrough(mut self, strikethrough: Stroke) -> Self {
        self.strikethrough = Some(strikethrough);
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn build(self) -> TextFormat {
        TextFormat {
            font_id: self.font_id.unwrap_or(FontId::default()),
            color: self.color.unwrap_or(Color32::GRAY),
            background: self.background.unwrap_or(Color32::TRANSPARENT),
            italics: self.italics.unwrap_or(false),
            underline: self.underline.unwrap_or(Stroke::NONE),
            strikethrough: self.strikethrough.unwrap_or(Stroke::NONE),
            valign: self.valign.unwrap_or(Align::BOTTOM),
        }
    }
}

impl Formatting {
    pub fn atom(&self) -> Atom<EmptyStaticAtomSet> {
        match self {
            Formatting::Bold => Atom::from("Bold"),
            Formatting::NotBold => Atom::from("NotBold"),
            Formatting::Italic => Atom::from("Italic"),
            Formatting::NotItalic => Atom::from("NotItalic"),
            Formatting::Link { .. } => Atom::from("Link"),
            Formatting::NotLink => Atom::from("NotLink"),
            // Formatting::Citation(_) => Atom::from("Citation"),
            Formatting::Comment(_) => Atom::from("Comment"),
            // Formatting::Unknown => Atom::from("Unknown"),
        }
    }
}

impl From<(&Atom<EmptyStaticAtomSet>, &Value)> for Formatting {
    fn from((atom, value): (&Atom<EmptyStaticAtomSet>, &Value)) -> Self {
        match (atom.as_ref(), value) {
            ("Bold", _) => Formatting::Bold,
            ("NotBold", _) => Formatting::NotBold,
            ("Italic", _) => Formatting::Italic,
            ("NotItalic", _) => Formatting::NotItalic,
            ("Link", url) => Formatting::Link {
                url: url.to_string(),
            },
            ("NotLink", _) => Formatting::NotLink,
            ("Comment", comment) => Formatting::Comment(comment.to_string()),
            _ => unreachable!(),
        }
    }
}

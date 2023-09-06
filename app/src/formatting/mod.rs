pub mod citation;

use std::default;

pub use citation::Citation;
use egui::{Color32, Stroke, TextFormat};
use serde_json::Value;
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
    Citation(Citation),
    Comment(String),
    Unknown,
}

impl From<Formatting> for TextFormat {
    fn from(value: Formatting) -> Self {
        match value {
            Formatting::Italic => TextFormat {
                color: Color32::DARK_BLUE,
                background: Color32::WHITE,
                ..Default::default()
            },
            _ => TextFormat::default(),
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
            Formatting::Citation(_) => Atom::from("Citation"),
            Formatting::Comment(_) => Atom::from("Comment"),
            Formatting::Unknown => Atom::from("Unknown"),
        }
    }
}

impl From<(&Atom<EmptyStaticAtomSet>, &Value)> for Formatting {
    fn from((atom, value): (&Atom<EmptyStaticAtomSet>, &Value)) -> Self {
        match (atom.as_ref(), value) {
            ("Bold", _) => Formatting::Bold,
            ("Italic", _) => Formatting::Italic,
            _ => Formatting::Unknown,
        }
    }
}

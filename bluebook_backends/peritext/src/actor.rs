use std::cell::RefCell;
use std::ops::RangeBounds;

use crate::{formatting::Formatting, Behavior, Expand, RichText, Style};

//probaby want to use a RefCell
pub struct Actor {
    pub text: RichText,
}

impl Actor {
    pub fn new(id: usize) -> Self {
        Self {
            text: RichText::new(id as u64),
        }
    }

    pub fn insert(&mut self, pos: usize, content: &str) {
        self.text.insert(pos, content);
    }

    pub fn delete(&mut self, pos: usize, len: usize) {
        self.text.delete(pos..pos + len)
    }

    pub fn annotate(&mut self, range: impl RangeBounds<usize>, type_: Formatting) {
        match type_ {
            Formatting::Bold => self.text.annotate(
                range,
                Style {
                    expand: Expand::After,
                    behavior: Behavior::Merge,
                    type_: "bold".into(),
                    value: serde_json::Value::Null,
                },
            ),
            Formatting::Link { url } => self.text.annotate(
                range,
                Style {
                    expand: Expand::None,
                    behavior: Behavior::Merge,
                    type_: "link".into(),
                    value: serde_json::Value::Bool(true),
                },
            ),
            Formatting::Comment(comment) => self.text.annotate(
                range,
                Style {
                    expand: Expand::None,
                    behavior: Behavior::AllowMultiple,
                    type_: "comment".into(),
                    value: serde_json::Value::String(comment.to_owned()),
                },
            ),
            Formatting::NotBold => self.text.annotate(
                range,
                Style {
                    expand: Expand::After,
                    behavior: Behavior::Delete,
                    type_: "bold".into(),
                    value: serde_json::Value::Null,
                },
            ),
            Formatting::NotLink => self.text.annotate(
                range,
                Style {
                    expand: Expand::Both,
                    behavior: Behavior::Delete,
                    type_: "link".into(),
                    value: serde_json::Value::Null,
                },
            ),
            _ => {}
        };
    }

    fn merge(&mut self, other: &Self) {
        self.text.merge(&other.text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_rte() {
        let mut actor = Actor::new(1);

        actor.insert(0, "Hello, this is Hector");

        actor.annotate(.., Formatting::Comment("This is a comment".to_string()));

        let spans = actor.text.get_spans();

        for span in &spans {
            print!("{}", span);
        }
    }
}

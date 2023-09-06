use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum Author {
    Individual(String), // "John Doe"
    Group(String),      // "The XYZ Group"
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct JournalArticle {
    title: String,
    authors: Vec<Author>,
    journal_name: String,
    publication_date: DateTime<Utc>, // This could also be a Date type
    volume: u32,
    issue: Option<u32>,
    page_numbers: (u32, u32), // start and end pages
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum Citation {
    Apa(JournalArticle),
    Mla(JournalArticle),
    Chicago(JournalArticle),
    // ... other styles ...
}

impl Citation {
    fn format(&self) -> String {
        match self {
            Citation::Apa(article) => {
                let authors = article
                    .authors
                    .iter()
                    .map(|a| match a {
                        Author::Individual(name) => format!("{}", name),
                        Author::Group(name) => format!("{}", name),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                format!(
                    "{} ({}). {}. {}. {}({}), {}-{}.",
                    authors,
                    article.publication_date,
                    article.title,
                    article.journal_name,
                    article.volume,
                    article.issue.as_ref().unwrap_or(&0),
                    article.page_numbers.0,
                    article.page_numbers.1
                )
            }
            Citation::Mla(article) => {
                // Similar logic but different formatting
                // For simplicity, using a stub string here
                format!("MLA Format for: {}", article.title)
            }
            Citation::Chicago(article) => {
                // Similar logic but different formatting
                // For simplicity, using a stub string here
                format!("Chicago Format for: {}", article.title)
            } // ... other styles ...
        }
    }
}

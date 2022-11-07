pub use element::Element;

use std::fmt::{Display, Formatter, Result as FmtResult};

mod element;
mod indentation;

pub enum HtmlData {
    Element(Element),
    Comment(String),
    PlainText(String),
}

impl Display for HtmlData {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let text = match &self {
            HtmlData::Element(element) => format!("{}", element),
            HtmlData::Comment(text) => format!("<-- {} -->", text),
            HtmlData::PlainText(text) => text.to_string(),
        };
        write!(f, "{}", text)
    }
}

pub use element::Element;

mod element;
mod indentation;

pub enum HtmlData {
    Element(Element),
    Comment(String),
    PlainText(String),
}

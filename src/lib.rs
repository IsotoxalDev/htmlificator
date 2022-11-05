pub use element::Element;

mod element;
mod indentation;

enum HtmlData {
    Element(Element),
    Comment(String),
    PlainText(String),
}

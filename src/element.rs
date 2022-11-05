use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    collections::HashMap,
};

use crate::indentation::indent;

use crate::HtmlData;

pub struct Element {
    name: String,
    self_closing: bool,
    attributes: HashMap<String, String>,
    children: Vec<HtmlData>,
}

impl Element {
    pub fn new(name: &str, self_closing: bool) -> Self {
        Element {
            name: name.to_string(),
            self_closing,
            attributes: HashMap::new(),
            children: vec!()
        }
    }
    
    pub fn add_id(&mut self, id: &str) {
        self.attributes.insert(
            String::from("id"), id.to_string(),
        );
    }
    
    pub fn add_class(&mut self, class: &str) {
        match self.attributes.get("class") {
            Some (classes) => {
                self.attributes.insert(String::from("class"), format!("{} {}", classes, class))
            },
            None => self.attributes.insert(String::from("class"), class.to_string()),
        };
    }
    pub fn add_element(&mut self, element: Element) {
        if self.self_closing {return}
        self.children.push(HtmlData::Element(element));
    }
    pub fn add_comment(&mut self, comment: &str) {
        if self.self_closing {return}
        self.children.push(HtmlData::Comment(comment.to_string()));
    }
    pub fn add_text(&mut self, text: &str) {
        if self.self_closing {return}
        self.children.push(HtmlData::PlainText(text.to_string()));
    }
    pub fn add_attribute(&mut self, attribute: &str, value: &str) {
        self.attributes.insert(
            attribute.to_string(), value.to_string(),
        );
    }
}

impl Display for Element {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let mut attributes = String::new();
        for (attribute, value) in self.attributes.iter() {
            attributes = format!("{} {}=\"{}\"", attributes, attribute, value)
        }
        if self.self_closing {
             write!(f, "<{}{} />", self.name, attributes)
        }
        else {
            let mut children = String::new();
            for child in &self.children {
                let child_text = match child {
                    HtmlData::Element(element) => format!("{}", element),
                    HtmlData::Comment(text) => format!("<-- {} -->", text),
                    HtmlData::PlainText(text) => text.to_string(),
                };
                children = format!("{}\n{}", children, indent(&child_text, "  "));
            }
            write!(f, "<{}{}>{}\n</{}>", self.name, attributes, children, self.name)
        }
    }
}
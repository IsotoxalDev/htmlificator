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
    /// This function returns an empty element.
    pub fn new(name: &str, self_closing: bool) -> Self {
        Element {
            name: name.to_string(),
            self_closing,
            attributes: HashMap::new(),
            children: vec!(),
        }
    }
    
    /// This function returns an element from the given data
    pub fn from(name: &str, self_closing: bool, 
        attributes: HashMap<String, String>,
        children: Vec<HtmlData>) -> Self {
        Element {
            name: name.to_string(),
            self_closing,
            attributes,
            children,
        }
    }
    
    /// Set the ID of the element.
    pub fn set_id(&mut self, id: &str) {
        self.attributes.insert(
            String::from("id"), id.to_string(),
        );
    }
    
    /// Add a class to the element.
    pub fn add_class(&mut self, class: &str) {
        match self.attributes.get("class") {
            Some (classes) => {
                self.attributes.insert(String::from("class"), format!("{} {}", classes, class))
            },
            None => self.attributes.insert(String::from("class"), class.to_string()),
        };
    }
    
    /// Add a child element to the element.
    pub fn add_element(&mut self, element: Element) {
        if self.self_closing {return}
        self.children.push(HtmlData::Element(element));
    }
    
    /// Append a vector of HTML data into the element
    pub fn add_children(&mut self, children: &mut Vec<HtmlData>) {
        if self.self_closing {return}
        self.children.append(children);
    }
    
    /// Add a comment inside the element.
    pub fn add_comment(&mut self, comment: &str) {
        if self.self_closing {return}
        self.children.push(HtmlData::Comment(comment.to_string()));
    }
    
    /// Add plain text inside teh element.
    pub fn add_text(&mut self, text: &str) {
        if self.self_closing {return}
        self.children.push(HtmlData::PlainText(text.to_string()));
    }
    
    /// Add an attribute to the element
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
                let text = format!("{}", child);
                children = format!("{}\n{}", children, indent(&text, "  "));
            }
            write!(f, "<{}{}>{}\n</{}>", self.name, attributes, children, self.name)
        }
    }
}
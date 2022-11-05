use htmlificator::*;

fn main() {
    let mut el = Element::new("Div", false);
    el.add_class("TestClass");
    el.add_class("AnotherClass");
    let mut h1 = Element::new("H1", false);
    h1.add_text("This is a heading!!");
    el.add_element(h1);
    el.add_comment("A Comment");
    let mut btn = Element::new("Button", true);
    btn.add_attribute("text", "This is a Button");
    el.add_element(btn);
    println!("{}", el)
}

# Htmlificator

This crate provides an element struch which can be displayed as HTML.

## License
This crate is  licensed under the MIT license

## Credit
This crate has used a snippet from the [TextWrap](https://crates.io/crates/textwrap) crate for indentation of the child elements.

## Usage
```toml
[dependencies]
htmlificator = "0.2"

```

```rust
use htmlificator::Element;

fn main() {
    let mut el = Element::new("Div", false);
    el.add_class("TestClass");
    let mut h1 = Element::new("H1", false);
    h1.add_text("This is a heading!!");
    el.add_element(h1);
    el.add_comment("A Comment");
    let mut btn = Element::new("Button", true);
    btn.add_attribute("text", "This is a Button");
    el.add_element(btn);
    println!("{}", el)
}
```

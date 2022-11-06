use htmlificator::Element;

#[test]
fn self_closing () {
    let el = Element::new("H1", true);
    assert_eq!(format!("{}", el), "<H1 />")
}
#[test]
fn non_self_closing () {
    let el = Element::new("Div", false);
    assert_eq!(format!("{}", el), "<Div>\n</Div>")
}
#[test]
fn comment () {
    let mut el = Element::new("Div", false);
    el.add_comment("A Comment");
    assert_eq!(format!("{}", el), "<Div>\n  <-- A Comment -->\n</Div>")
}
#[test]
fn child () {
    let mut el = Element::new("Div", false);
    let mut h1 = Element::new("H1", false);
    h1.add_text("Text");
    el.add_element(h1);
    assert_eq!(format!("{}", el), "<Div>\n  <H1>\n    Text\n  </H1>\n</Div>")
}

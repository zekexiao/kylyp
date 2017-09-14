use pulldown_cmark::{Parser,html};
use std::collections::HashSet;
use ammonia::Ammonia;

pub fn render_html(text: &str) -> String {
    let mut s = String::with_capacity(text.len() * 3 / 2);
    let p = Parser::new(&text);
    html::push_html(&mut s, p);
    let mut cleaner = Ammonia::default();
    let mut code_attributes = HashSet::new();
    code_attributes.insert("class");
    cleaner.tag_attributes.insert("code", code_attributes);
    cleaner.clean(&*s).to_owned()
}
use crate::inline_parsing::tag::HtmlTag;

pub enum Token<'a> {
    // Plaintext(usize, usize),
    Plaintext(&'a str),
    SingleFormatChar(&'a HtmlTag<'a>),
    Space,
}

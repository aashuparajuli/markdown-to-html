use crate::inline_parsing::tag::HtmlTag;

#[derive(Clone, Copy, Debug)]
pub enum Token<'a> {
    // Plaintext(usize, usize),
    Plaintext(&'a str),
    SingleFormatChar(char),
    Space,
    DoubleFormatChar(&'a HtmlTag<'a>), //each character, except double asterisk gets it own character
}
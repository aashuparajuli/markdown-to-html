use crate::single_char_parse::single_char_parser::HtmlTag;

#[derive(Clone, Copy, Debug)]
pub enum Token<'a> {
    // Plaintext(usize, usize),
    Plaintext(&'a str),
    Asterisk(char),
    Space,
    DoubleAsterisk(&'a HtmlTag<'a>), //each character, except double asterisk gets it own character
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HtmlTag<'a> {
    pub opening_tag: &'a str,
    pub closing_tag: &'a str,
    pub matching_char: char,
}
impl HtmlTag<'_> {
    pub fn is_special_char(&self, c: char) -> bool {
        self.matching_char == c
    }
}
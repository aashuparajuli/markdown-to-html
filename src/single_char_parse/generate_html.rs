use super::token::Token;
use crate::inline_parsing::tag::HtmlTag;

/*
HtmlTag needs a tuple of: matching_char, start_tag, end_tag
FormatSection needs a tuple of: matching_char, start_tag, end_tag
 */

impl HtmlTag<'_> {
    fn wrap_text(&self, s: &str) -> String {
        format!("{}{s}{}", self.opening_tag, self.closing_tag)
    }
    fn escape(&self) -> String {
        format!("{}", self.matching_char)
    }
}
#[derive(Clone, Debug)]
enum FormatSection<'a> {
    Text(String),
    Format(&'a HtmlTag<'a>),
}
impl<'a> FormatSection<'a> {
    fn get_html(&self) -> String {
        match self {
            FormatSection::Text(x) => x.to_string(),
            FormatSection::Format(tag) => tag.escape(),
        }
    }
}
impl PartialEq for FormatSection<'_>{
    fn eq(&self, other: &Self) -> bool {
        match(self, other){
            (FormatSection::Text(x), FormatSection::Text(y)) => x== y,
            (FormatSection::Text(_), FormatSection::Format(_)) => false,
            (FormatSection::Format(_), FormatSection::Text(_)) => false,
            (FormatSection::Format(x), FormatSection::Format(y)) => x ==y,
        }
    }
}
// impl PartialEq for HtmlTag<'_>{
//     fn eq(&self, other: &Self) -> bool {
//         self.matching_char == other.matching_char && self.opening_tag == other.opening_tag && self.closing_tag == other.opening_tag
//     }
// }
trait GrowText {
    fn expand(&mut self, s: &str);
}
impl GrowText for Option<String> {
    fn expand(&mut self, s: &str) {
        match self {
            Some(x) => {
                x.push_str(s);
            }
            None => {
                *self = Some(s.to_string());
            }
        }
    }
}
pub fn tokens_to_html(tokens: &Vec<Token>) -> String {
    let mut result: String = String::new();
    let mut curr_plaintext: Option<String> = None;
    let mut section_stack: Vec<FormatSection> = Vec::new();
    for next_token in tokens {
        //stack will store FormatSection will be stored in
        //push items into the FormatSection as we parse
        //when the stack has the correct values, pop values, format them, push them back on

        match (&mut curr_plaintext, next_token) {
            (Some(x), Token::Plaintext(s)) => {
                curr_plaintext.expand(s);
                // x.push_str(s);
                // todo!("extend plaintext");
            }
            (Some(x), Token::SingleFormatChar(formatting_tag)) => {
                let prev_token = section_stack.last().clone();
                // todo!("only pop value if tags are the same");
                // if prev_token.is_some() && prev_token.unwrap(). == formatting_tag {
                //     //pop the value
                //     section_stack.pop();
                //     //push text formatted with the <i> tag
                //     *x = formatting_tag.wrap_text(x);
                // }
                // if let Some(FormatSection::Format(formatting_tag)) = prev_token {
                   
                //     //pop value from stack
                //     section_stack.pop();
                //     //push text formatted with the <i> tag
                //     *x = formatting_tag.wrap_text(x);
                //     // *x = format!(
                //     //     "{}{x}{}",
                //     //     formatting_tag.opening_tag, formatting_tag.closing_tag
                //     // );
                //     //continue building the formatted text after this
                // } 
                else {
                    //push standard non-formatted text
                    section_stack.push(FormatSection::Text(x.to_string()));
                    curr_plaintext = None;
                    section_stack.push(FormatSection::Format(formatting_tag));
                }
                //todo!("push current String to stack as FormatSection::Text, push DoubleAsterisk to stack. (Also check for the DoubleAsterisk before");
            }
            (None, Token::Plaintext(s)) => {
                curr_plaintext.expand(*s); //expanding using Space
                                           //curr_plaintext = Some(String::from(*s));
                                           //todo!("start new plaintext");
            }
            (Some(ref mut x), Token::Space) => {
                //todo!("push space as plaintext");
                curr_plaintext.expand(" "); //convert Token::Space into plaintext
                                            //x.push(' ');
            }
            (None, Token::Space) => {
                let prev_token = section_stack.last().cloned();
                //pop bold token(e)
                if let Some(FormatSection::Format(tag)) = prev_token {
                    section_stack.pop();
                    //Push escaped plaintext to plaintext
                    curr_plaintext.expand(&tag.escape()); //escape the FormatToken that was previously here
                                                          //curr_plaintext.expand(" ");//expanding using Space
                                                          // curr_plaintext = Some(format!("{0} ", tag.escape())); //pushing the double asterisk from the escaped bold, then space
                }
                //add space to plaintext
                curr_plaintext.expand(" ");
            }
            (None, Token::SingleFormatChar(tag)) => {
                section_stack.push(FormatSection::Format(tag));
                // todo!("push double asterisk to stack");
            }
        };
    }
    //push FormatSection if it has not been pushed
    if let Some(x) = curr_plaintext {
        section_stack.push(FormatSection::Text(x.to_string()));
    }

    section_stack
        .iter()
        .for_each(|section| result.push_str(&section.get_html()));
    result
}
#[cfg(test)]
mod test_token_parser {
    use super::tokens_to_html;
    use super::Token;
    use crate::inline_parsing::tag::HtmlTag;
    const ITALICS_ASTERISK_TAG: HtmlTag = HtmlTag {
        opening_tag: "<i>",
        closing_tag: "</i>",
        matching_char: '*',
    };
    #[test]
    fn one_token() {
        //string with space before pound sign should not be converted
        let tokens = vec![Token::SingleFormatChar(&ITALICS_ASTERISK_TAG)];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("*");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn two_tokens() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![Token::SingleFormatChar(&ITALICS_ASTERISK_TAG), Token::Space];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("* ");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn three_tokens() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![
            Token::SingleFormatChar(&ITALICS_ASTERISK_TAG),
            Token::Space,
            Token::Plaintext("p"),
        ];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("* p");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn longer_plaintext() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![
            Token::Plaintext("some"),
            Token::SingleFormatChar(&ITALICS_ASTERISK_TAG),
            Token::Space,
        ];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("some* ");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn single_double_asterisk() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![
            Token::Plaintext("some"),
            Token::SingleFormatChar(&ITALICS_ASTERISK_TAG),
            Token::Space,
        ];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("some* ");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn short_bold() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![
            Token::SingleFormatChar(&ITALICS_ASTERISK_TAG),
            Token::Plaintext("some"),
            Token::SingleFormatChar(&ITALICS_ASTERISK_TAG),
        ];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("<i>some</i>");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn valid_two_words() {
        //string with space before pound sign should not be converted

        let tokens: Vec<Token> = vec![
            Token::SingleFormatChar(&ITALICS_ASTERISK_TAG),
            Token::Plaintext("so"),
            Token::Space,
            Token::Plaintext("me"),
            Token::SingleFormatChar(&ITALICS_ASTERISK_TAG),
        ];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("<i>so me</i>");
        assert_eq!(output, expected_output);

        //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
    }
}
#[cfg(test)]
mod test_mixed_tokens {
    use super::tokens_to_html;
    use super::Token;
    use crate::inline_parsing::tag::HtmlTag;
    const ITALICS_UNDERSCORE_TAG: HtmlTag = HtmlTag {
        opening_tag: "<i>",
        closing_tag: "</i>",
        matching_char: '_',
    };
    const CODE_TAG: HtmlTag = HtmlTag {
        opening_tag: "<code>",
        closing_tag: "</code>",
        matching_char: '`',
    };
    const ITALICS_ASTERISK_TAG: HtmlTag = HtmlTag {
        opening_tag: "<i>",
        closing_tag: "</i>",
        matching_char: '*',
    };
    #[test]
    fn two_tokens() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![Token::SingleFormatChar(&ITALICS_ASTERISK_TAG), Token::SingleFormatChar(&ITALICS_UNDERSCORE_TAG)];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("*_");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn three_tokens() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![
            Token::SingleFormatChar(&ITALICS_ASTERISK_TAG),
            Token::Plaintext("some"),
            Token::SingleFormatChar(&ITALICS_UNDERSCORE_TAG),
        ];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("*some_");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn asterisk_pair_w_underscore() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![
            Token::SingleFormatChar(&ITALICS_ASTERISK_TAG),
            Token::Plaintext("some"),
            Token::SingleFormatChar(&ITALICS_ASTERISK_TAG),
            Token::SingleFormatChar(&ITALICS_UNDERSCORE_TAG),
        ];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("<i>some</i>_");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn two_words_mixed() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![
            Token::SingleFormatChar(&ITALICS_UNDERSCORE_TAG),
            Token::Plaintext("some"),
            Token::Space,
            Token::Plaintext("one"),
            Token::SingleFormatChar(&ITALICS_UNDERSCORE_TAG),
            Token::SingleFormatChar(&ITALICS_ASTERISK_TAG),
        ];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("<i>some one</i>*");
        assert_eq!(output, expected_output);
    }
}

use super::token::Token;

#[derive(Clone, Debug)]
enum FormatSection {
    Text(String),
    Bold(char),
}
impl FormatSection {
    fn get_html(&self) -> String {
        match self {
            FormatSection::Text(x) => x.to_string(),
            FormatSection::Bold(c) => format!("{c}{c}"),
        }
    }
}
pub fn tokens_to_html(tokens: &Vec<Token>) -> String {
    let mut result: String = String::new();
    let mut curr_format_section: Option<String> = None;
    let mut section_stack: Vec<FormatSection> = Vec::new();
    for next_token in tokens {
        //stack will store FormatSection will be stored in
        //push items into the FormatSection as we parse
        //when the stack has the correct values, pop values, format them, push them back on

        match (&mut curr_format_section, next_token) {
            (Some(x), Token::Plaintext(s)) => {
                x.push_str(s);
                // todo!("extend plaintext");
            }
            (Some(x), Token::DoubleFormatChar(formatting_tag)) => {
                if let Some(FormatSection::Bold(_)) = section_stack.last() {
                    //pop value from stack
                    section_stack.pop();
                    //push text formatted with the <b> tag
                    *x = format!(
                        "{}{x}{}",
                        formatting_tag.opening_tag, formatting_tag.closing_tag
                    );
                    //continue building the formatted text after this
                } else {
                    //push standard non-formatted text
                    section_stack.push(FormatSection::Text(x.to_string()));
                    curr_format_section = None;
                    section_stack.push(FormatSection::Bold(formatting_tag.matching_char));
                }
                //todo!("push current String to stack as FormatSection::Text, push DoubleAsterisk to stack. (Also check for the DoubleAsterisk before");
            }
            (Some(ref mut x), Token::SingleFormatChar(c)) => {
                x.push(*c);
                // todo!("push as plaintext");
            }
            (Some(ref mut x), Token::Space) => {
                //todo!("push space as plaintext");
                x.push(' ');
            }
            (None, Token::Plaintext(s)) => {
                curr_format_section = Some(String::from(*s));
                //todo!("start new plaintext");
            }
            (None, Token::SingleFormatChar(c)) => {
                curr_format_section = Some(String::from(*c));
                // todo!("start new plaintext");
            }
            (None, Token::Space) => {
                //pop bold token
                if let Some(FormatSection::Bold(c)) = section_stack.pop() {
                    //create new Plaintext to start building
                    curr_format_section = Some(format!("{0}{0} ", c)); //pushing the double asterisk from the escaped bold, then space
                } else {
                    //create new Plaintext to start building
                    curr_format_section = Some(String::from(" ")); //pushing the double asterisk from the escaped bold, then space
                }
            }
            (None, Token::DoubleFormatChar(tag)) => {
                section_stack.push(FormatSection::Bold(tag.matching_char));
                // todo!("push double asterisk to stack");
            }
        };
    }
    //push FormatSection if it has not been pushed
    if let Some(x) = curr_format_section {
        section_stack.push(FormatSection::Text(x.to_string()));
    }

    section_stack
        .iter()
        .for_each(|section| result.push_str(&section.get_html()));
    result
}
#[cfg(test)]
mod test_token_parser {
    use crate::inline_parsing::tag::HtmlTag;
    use super::tokens_to_html;
    use super::Token;
    const BOLD_ASTERISK_TAG: HtmlTag = HtmlTag {
        opening_tag: "<b>",
        closing_tag: "</b>",
        matching_char: '*',
    };
    #[test]
    fn one_token() {
        //string with space before pound sign should not be converted
        let tokens = vec![Token::SingleFormatChar('*')];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("*");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn two_tokens() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![Token::SingleFormatChar('*'), Token::Space];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("* ");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn three_tokens() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![Token::SingleFormatChar('*'), Token::Space, Token::Plaintext("p")];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("* p");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn longer_plaintext() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![Token::Plaintext("some"), Token::SingleFormatChar('*'), Token::Space];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("some* ");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn single_double_asterisk() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![
            Token::Plaintext("some"),
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG),
            Token::Space,
        ];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("some** ");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn short_bold() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG),
            Token::Plaintext("some"),
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG),
        ];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("<b>some</b>");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn valid_two_words() {
        //string with space before pound sign should not be converted

        let tokens: Vec<Token> = vec![
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG),
            Token::Plaintext("so"),
            Token::Space,
            Token::Plaintext("me"),
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG),
        ];
        let output: String = tokens_to_html(&tokens);
        let expected_output = String::from("<b>so me</b>");
        assert_eq!(output, expected_output);

        //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
    }
}

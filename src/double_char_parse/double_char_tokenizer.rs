use crate::single_char_parse::single_char_parser::HtmlTag;
use super::tokens::Token;

mod tokenizer {
    use crate::single_char_parse::single_char_parser::HtmlTag;
    use super::Token;
    //use super::BOLD_ASTERISK_TAG;
    #[derive(Clone, Copy)]
    enum CharType {
        Asterisk,
        Plaintext,
        Space,
    }
    impl CharType {
        pub fn new(c: char, tag: char) -> CharType {
            if c == tag {
                CharType::Asterisk
            } else if c == ' ' {
                CharType::Space
            } else {
                CharType::Plaintext
            }
        }
    }
    pub fn double_char_tokenizer<'a>(str: &'a str, tag: &'a HtmlTag) -> Vec<Token<'a>> {
        if str.is_empty() {
            return Vec::new();
        }
        //make it generic over any type that implements
        let mut token_stream: Vec<Token> = Vec::new();
        let mut curr_section: Option<CharType> = None;
        let mut start_idx: usize = usize::max_value();
        let mut reading_plaintext: bool = false;
        for (i, c) in str.char_indices() {
            let next_char: CharType = CharType::new(c, tag.matching_char);

            //preconditions, dealing with altering previous elements
            //if prev char is not plaintext && curr char is plaintext => set start_idx
            //if prev char is plaintext && curr char is not plaintext => use start_idx and i to get substring
            //if prev char is plaintext && curr char is plaintext, do nothing, let i continue growing
            //do nothing

            //otherwise, just push to stack
            match (curr_section, next_char) {
                (Some(CharType::Plaintext), CharType::Plaintext) => (),
                (_, CharType::Plaintext) => {
                    start_idx = i;
                    reading_plaintext = true;
                }
                (Some(CharType::Plaintext), _) => {
                    //append plaintext
                    //token_stream.push(Token::Plaintext(start_idx, i));
                    token_stream.push(Token::Plaintext(&str[start_idx..i]));
                    reading_plaintext = false;
                }
                (_, _) => (),
            };

            match next_char {
                CharType::Asterisk if matches!(token_stream.last(), Some(Token::Asterisk(_))) => {
                    //trigger double asterisk
                    token_stream.pop();
                    token_stream.push(Token::DoubleAsterisk(tag));
                }
                CharType::Asterisk => {
                    //push asterisk normally
                    token_stream.push(Token::Asterisk(c));
                }
                CharType::Space => {
                    //push space normally
                    token_stream.push(Token::Space);
                }
                CharType::Plaintext => {
                    //do nothing, plaintext taken care of by previous match
                }
            };
            curr_section = Some(next_char);
            //also perform parsing here
        }

        //if plaintext is still open, close it, then add
        if reading_plaintext {
            //token_stream.push(Token::Plaintext(start_idx, str.len()));
            token_stream.push(Token::Plaintext(&str[start_idx..]));
        }

        token_stream
    }
    #[cfg(test)]
    mod test_tokenizer {
        use super::double_char_tokenizer;
        use super::HtmlTag;
        use super::Token;
        const BOLD_ASTERISK_TAG: HtmlTag = HtmlTag {
            opening_tag: "<b>",
            closing_tag: "</b>",
            matching_char: '*',
        };
        #[test]
        fn basic() {
            //string with space before pound sign should not be converted
            let input_str = "some";
            let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
            assert_eq!(actual_result.len(), 1);
            assert!(matches!(actual_result[0], Token::Plaintext("some")));
            //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
        }
        #[test]
        fn single_asterisk() {
            //string with space before pound sign should not be converted
            let input_str = "som*";
            let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
            assert_eq!(actual_result.len(), 2);
            //assert!(matches!(actual_result[0], Token::Plaintext(0, 3)));
            assert!(matches!(actual_result[0], Token::Plaintext("som")));
            assert!(matches!(actual_result[1], Token::Asterisk('*')));
        }
        #[test]
        fn valid_two_words() {
            //string with space before pound sign should not be converted
            let input_str = "**so me**";
            let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
            assert_eq!(actual_result.len(), 5);
            assert!(matches!(
                actual_result[0],
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG)
            ));
            assert!(matches!(actual_result[1], Token::Plaintext("so")));
            assert!(matches!(actual_result[2], Token::Space));
            assert!(matches!(actual_result[3], Token::Plaintext("me")));
            assert!(matches!(
                actual_result[4],
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG)
            ));

            //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
        }
        #[test]
        fn double_asterisk() {
            //string with space before pound sign should not be converted
            let input_str = "some**";
            let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
            assert_eq!(actual_result.len(), 2);
            //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
            assert!(matches!(actual_result[0], Token::Plaintext("some")));
            assert!(matches!(
                actual_result[1],
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG)
            ));
            //assert!(matches!(actual_result[1], Token::DoubleAsterisk("<b>","</b>")));
        }
        #[test]
        fn invalid_double_spaces() {
            //string with space before pound sign should not be converted
            //string with space before pound sign should not be converted
            let input_str = "** some **";
            let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
            assert_eq!(actual_result.len(), 5);
            //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
            assert!(matches!(
                actual_result[0],
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG)
            ));
            assert!(matches!(actual_result[1], Token::Space));
            assert!(matches!(actual_result[2], Token::Plaintext("some")));
            assert!(matches!(actual_result[3], Token::Space));
            assert!(matches!(
                actual_result[4],
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG)
            ));
        }
        #[test]
        fn valid_double_spaces() {
            //string with space before pound sign should not be converted
            //string with space before pound sign should not be converted
            let input_str = "**some** ";
            let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
            assert_eq!(actual_result.len(), 4);
            //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
            assert!(matches!(
                actual_result[0],
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG)
            ));
            assert!(matches!(actual_result[1], Token::Plaintext("some")));
            assert!(matches!(
                actual_result[2],
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG)
            ));
            assert!(matches!(actual_result[3], Token::Space));
        }
        #[test]
        fn mixed() {
            //string with space before pound sign should not be converted
            let input_str = "some *";
            let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
            assert_eq!(actual_result.len(), 3);
            //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
            assert!(matches!(actual_result[0], Token::Plaintext("some")));
            assert!(matches!(actual_result[1], Token::Space));
            assert!(matches!(actual_result[2], Token::Asterisk('*')));
        }
        #[test]
        fn mixed_more() {
            //string with space before pound sign should not be converted
            let input_str = "some * here";
            let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
            assert_eq!(actual_result.len(), 5);
            //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
            assert!(matches!(actual_result[0], Token::Plaintext("some")));
            assert!(matches!(actual_result[1], Token::Space));
            assert!(matches!(actual_result[2], Token::Asterisk('*')));
            assert!(matches!(actual_result[3], Token::Space));
            //assert!(matches!(actual_result[4], Token::Plaintext(7, 11)));
            assert!(matches!(actual_result[4], Token::Plaintext("here")));
        }
    }
}
mod parse_tokens {
    use super::Token;
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
                (Some(x), Token::DoubleAsterisk(formatting_tag)) => {
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
                (Some(ref mut x), Token::Asterisk(c)) => {
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
                (None, Token::Asterisk(c)) => {
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
                (None, Token::DoubleAsterisk(tag)) => {
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

        use crate::single_char_parse::single_char_parser::HtmlTag;

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
            let tokens = vec![Token::Asterisk('*')];
            let output: String = tokens_to_html(&tokens);
            let expected_output = String::from("*");
            assert_eq!(output, expected_output);
        }
        #[test]
        fn two_tokens() {
            //string with space before pound sign should not be converted
            let tokens: Vec<Token> = vec![Token::Asterisk('*'), Token::Space];
            let output: String = tokens_to_html(&tokens);
            let expected_output = String::from("* ");
            assert_eq!(output, expected_output);
        }
        #[test]
        fn three_tokens() {
            //string with space before pound sign should not be converted
            let tokens: Vec<Token> =
                vec![Token::Asterisk('*'), Token::Space, Token::Plaintext("p")];
            let output: String = tokens_to_html(&tokens);
            let expected_output = String::from("* p");
            assert_eq!(output, expected_output);
        }
        #[test]
        fn longer_plaintext() {
            //string with space before pound sign should not be converted
            let tokens: Vec<Token> =
                vec![Token::Plaintext("some"), Token::Asterisk('*'), Token::Space];
            let output: String = tokens_to_html(&tokens);
            let expected_output = String::from("some* ");
            assert_eq!(output, expected_output);
        }
        #[test]
        fn single_double_asterisk() {
            //string with space before pound sign should not be converted
            let tokens: Vec<Token> = vec![
                Token::Plaintext("some"),
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG),
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
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG),
                Token::Plaintext("some"),
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG),
            ];
            let output: String = tokens_to_html(&tokens);
            let expected_output = String::from("<b>some</b>");
            assert_eq!(output, expected_output);
        }
        #[test]
        fn valid_two_words() {
            //string with space before pound sign should not be converted

            let tokens: Vec<Token> = vec![
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG),
                Token::Plaintext("so"),
                Token::Space,
                Token::Plaintext("me"),
                Token::DoubleAsterisk(&BOLD_ASTERISK_TAG),
            ];
            let output: String = tokens_to_html(&tokens);
            let expected_output = String::from("<b>so me</b>");
            assert_eq!(output, expected_output);

            //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
        }
    }
}

pub fn parse_double_char(s: &str, tag: &HtmlTag) -> String {
    //next step: don't want to pass BOLD_ASTERISK_TAG into
    let tokens: Vec<Token> = tokenizer::double_char_tokenizer(s, tag);
    let parsed_string = parse_tokens::tokens_to_html(&tokens);

    parsed_string
}
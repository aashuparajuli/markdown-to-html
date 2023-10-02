use crate::inline_parsing::tag::HtmlTag;
use double_char_generator::parse_double_char;
mod double_char_generator {
    use crate::double_char_parse::generate_html;
    use crate::double_char_parse::tokenizer;
    use crate::double_char_parse::tokens::Token;
    use crate::inline_parsing::tag::HtmlTag;
    pub fn parse_double_char(s: &str, tag: &HtmlTag) -> String {
        //next step: don't want to pass BOLD_ASTERISK_TAG into
        let tokens: Vec<Token> = tokenizer::double_char_tokenizer(s, tag);
        let parsed_string = generate_html::tokens_to_html(&tokens);
        parsed_string
    }

    #[cfg(test)]
    mod test_token_parser {
        use super::Token;
        use crate::double_char_parse::generate_html::tokens_to_html;
        use crate::inline_parsing::tag::HtmlTag;
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
            let tokens: Vec<Token> = vec![
                Token::SingleFormatChar('*'),
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
                Token::SingleFormatChar('*'),
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
}
pub mod bold {
    use super::{parse_double_char, HtmlTag};
    pub fn parse_bold_asterisk(s: &str) -> String {
        const BOLD_ASTERISK_TAG: HtmlTag = HtmlTag {
            opening_tag: "<b>",
            closing_tag: "</b>",
            matching_char: '*',
        };
        //next step: don't want to pass BOLD_ASTERISK_TAG into
        parse_double_char(s, &BOLD_ASTERISK_TAG)
    }
    #[cfg(test)]
    mod bold_asterisk_tests {
        use super::parse_bold_asterisk;
        #[test]
        fn convert_bold() {
            let input_str = String::from("some **text**");
            let expected_result = String::from("some <b>text</b>");
            let actual_result = parse_bold_asterisk(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_bold_invalid_one() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some ** text* *");
            let expected_result = String::from("some ** text* *");
            let actual_result: String = parse_bold_asterisk(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn invalid_double_spaces() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some ** text **");
            let expected_result = String::from("some ** text **");
            let actual_result: String = parse_bold_asterisk(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn valid_single_spaces() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some **text** ");
            let expected_result = String::from("some <b>text</b> ");
            let actual_result: String = parse_bold_asterisk(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_bold_invalid_two() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some * * text**");
            let expected_result = String::from("some * * text**");
            let actual_result: String = parse_bold_asterisk(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_bold_invalid_three() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some **text");
            let expected_result = String::from("some **text");
            let actual_result: String = parse_bold_asterisk(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_bold_invalid_four() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some **text\n");
            let expected_result = String::from("some **text\n");
            let actual_result: String = parse_bold_asterisk(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_bold_invalid_five() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some **text*\n");
            let expected_result = String::from("some **text*\n");
            let actual_result: String = parse_bold_asterisk(&input_str);
            assert_eq!(actual_result, expected_result);
        }
    }

    pub fn parse_bold_underscore(s: &str) -> String {
        const BOLD_UNDERSCORE_TAG: HtmlTag = HtmlTag {
            opening_tag: "<b>",
            closing_tag: "</b>",
            matching_char: '_',
        };
        //next step: don't want to pass BOLD_ASTERISK_TAG into
        parse_double_char(s, &BOLD_UNDERSCORE_TAG)
    }
    #[cfg(test)]
    mod bold_underscore_tests {
        use super::parse_bold_underscore;
        #[test]
        fn convert_bold() {
            let input_str = String::from("some __text__");
            let expected_result = String::from("some <b>text</b>");
            let actual_result = parse_bold_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_bold_invalid_one() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some __ text_ _");
            let expected_result = String::from("some __ text_ _");
            let actual_result: String = parse_bold_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn invalid_double_spaces() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some __ text __");
            let expected_result = String::from("some __ text __");
            let actual_result: String = parse_bold_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn valid_single_spaces() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some __text__ ");
            let expected_result = String::from("some <b>text</b> ");
            let actual_result: String = parse_bold_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_bold_invalid_two() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some _ _ text__");
            let expected_result = String::from("some _ _ text__");
            let actual_result: String = parse_bold_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_bold_invalid_three() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some __text");
            let expected_result = String::from("some __text");
            let actual_result: String = parse_bold_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_bold_invalid_four() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some __text\n");
            let expected_result = String::from("some __text\n");
            let actual_result: String = parse_bold_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_bold_invalid_five() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some __text_\n");
            let expected_result = String::from("some __text_\n");
            let actual_result: String = parse_bold_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
    }
}

pub mod strikethrough {
    use super::{parse_double_char, HtmlTag};
    pub fn parse_strikethrough(s: &str) -> String {
        const STRIKETHROUGH_TAG: HtmlTag = HtmlTag {
            opening_tag: "<s>",
            closing_tag: "</s>",
            matching_char: '~',
        };
        //next step: don't want to pass BOLD_ASTERISK_TAG into
        parse_double_char(s, &STRIKETHROUGH_TAG)
    }
    #[cfg(test)]
    mod strikethrough_tests {
        use super::parse_strikethrough;
        #[test]
        fn convert_strikethrough() {
            let input_str = String::from("some ~~text~~");
            let expected_result = String::from("some <s>text</s>");
            let actual_result = parse_strikethrough(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn two_words() {
            let input_str = String::from("~~some text~~");
            let expected_result = String::from("<s>some text</s>");
            let actual_result = parse_strikethrough(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_strikethrough_invalid_one() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some ~~ text~ ~");
            let expected_result = String::from("some ~~ text~ ~");
            let actual_result: String = parse_strikethrough(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_strikethrough_invalid_two() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some ~ ~ text~~");
            let expected_result = String::from("some ~ ~ text~~");
            let actual_result: String = parse_strikethrough(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_strikethrough_invalid_three() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some ~~text");
            let expected_result = String::from("some ~~text");
            let actual_result: String = parse_strikethrough(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_strikethrough_invalid_four() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some ~~text\n");
            let expected_result = String::from("some ~~text\n");
            let actual_result: String = parse_strikethrough(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_strikethrough_invalid_five() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some ~~text~\n");
            let expected_result = String::from("some ~~text~\n");
            let actual_result: String = parse_strikethrough(&input_str);
            assert_eq!(actual_result, expected_result);
        }
    }
}

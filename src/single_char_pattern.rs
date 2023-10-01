//basic ideas: user passes in an enum that impl FormattingToken
//then all other operations work the same

//or user passes in a character, then a TextState is built off that
//or user passes in a function to check if a character is a token, then that is used to build a function
pub mod single_char_parser {

    pub struct FormatText<'a> {
        pub formatted: bool,
        pub substring: &'a str,
    }
    pub struct HtmlTag<'a> {
        pub opening_tag: &'a str,
        pub closing_tag: &'a str,
        pub matching_char: char,
    }
    impl HtmlTag<'_> {
        fn is_special_char(&self, c: char) -> bool {
            self.matching_char == c
        }
    }
    impl FormatText<'_> {
        pub fn new(formatted: bool, substring: &str) -> FormatText {
            FormatText {
                formatted,
                substring,
            }
        }
        fn get_html(&self, tag: &HtmlTag) -> String {
            match self.formatted {
                true => format!("{}{}{}", tag.opening_tag, self.substring, tag.closing_tag),
                false => self.substring.to_string(),
            }
        }
    }
    pub fn process_single_char_formats(str: &str, html_tag: HtmlTag) -> String {
        let mut result: String = String::new();
        let mut stack: Vec<FormatText> = Vec::new();
        let mut parsing_formatted_text: bool = false;
        let mut start_idx: usize = 0;
        if str.is_empty() {
            return String::new();
        }
        for (curr_idx, c) in str.char_indices() {
            //initially:currently_matching = false;
            /*cases for string matching:
               !currently_matching && c != '_' => endIdx +=1
               !currently_matching && c == '_' => {
                    push the substring to result: result.push_str(&str[startIdx..endIdx])
                    currently_matching = true
                    startIdx =
                }
                if currently_matching && c == '_' => {

                }
            */
            /*
            cases:
            - not in italics, adding a char
            - switching into italics
            - in italics, adding a char
            - switching out of italics
            */
            //switching in or out of italics
            // match (parsing_plaintext, c) {
            //     (false, '_') => {}
            //     (true, '_')  => {}
            //     (false, '_') => {}
            // };
            //case: where we see an escape character
            if parsing_formatted_text
                && (c == ' ' || html_tag.is_special_char(c))
                && start_idx == curr_idx
            {
                //move start_idx backwards so that the previously captured '*' is captured in plaintext
                start_idx -= 1;
                //switch to parsing italics
                parsing_formatted_text = false;
            }
            //other cases: we need to go from formatted text to plain text or vice versa
            if html_tag.is_special_char(c) {
                //a section of text has been finished
                //either push plain FormatText or FormatText that represents a special character
                if parsing_formatted_text {
                    //construct a FormatText struct storing TextState::Italics, append it to the stack
                    let italics_text =
                        FormatText::new(parsing_formatted_text, &str[start_idx..curr_idx]);
                    stack.push(italics_text);
                    start_idx = curr_idx;
                    parsing_formatted_text = false;
                } else {
                    //construct a FormattedText struct storing TextState::Plaintext, append it to the stack
                    let plain_text: FormatText =
                        FormatText::new(parsing_formatted_text, &str[start_idx..curr_idx]);
                    stack.push(plain_text);
                    //increment start pointer
                    start_idx = curr_idx + 1;
                    //switch into parsing italics mode
                    parsing_formatted_text = true;
                }
            }
        }
        //append any strings that have not been completed yet
        if parsing_formatted_text {
            //if there was an asterisk that is unmatched, then push it
            let plain_text = FormatText::new(false, &str[start_idx - 1..]);
            stack.push(plain_text);
        } else if start_idx != str.len() - 1 {
            //if a plaintext substring reaches the end of the fullstring, then push the entire substring to the stack
            let plain_text = FormatText::new(false, &str[start_idx..]);
            stack.push(plain_text);
        }
        stack
            .iter()
            .for_each(|subsection| result.push_str(&subsection.get_html(&html_tag)));
        result
    }
}
use single_char_parser::HtmlTag;
pub const ITALICS_UNDERSCORE_TAG: HtmlTag = HtmlTag {
    opening_tag: "<i>",
    closing_tag: "</i>",
    matching_char: '_',
};
pub const CODE_TAG: HtmlTag = HtmlTag {
    opening_tag: "<code>",
    closing_tag: "</code>",
    matching_char: '`',
};
pub const ITALICS_ASTERISK_TAG: HtmlTag = HtmlTag {
    opening_tag: "<i>",
    closing_tag: "</i>",
    matching_char: '*',
};
#[cfg(test)]
mod italics_underscore_test {
    use super::single_char_parser::process_single_char_formats;
    use super::single_char_parser::HtmlTag;
    use super::ITALICS_UNDERSCORE_TAG;
    fn is_underscore_token(c: char) -> bool {
        match c {
            '_' => true,
            _ => false,
        }
    }

    #[test]
    fn convert_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _text_");
        let expected_result = String::from("some <i>text</i>");
        let actual_result = process_single_char_formats(&input_str, ITALICS_UNDERSCORE_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_no_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("plain text");
        let expected_result = String::from("plain text");
        let actual_result = process_single_char_formats(&input_str, ITALICS_UNDERSCORE_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _text _");
        let expected_result = String::from("some <i>text </i>");
        let actual_result = process_single_char_formats(&input_str, ITALICS_UNDERSCORE_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _ text _");
        let expected_result = String::from("some _ text _");
        let actual_result: String = process_single_char_formats(&input_str, ITALICS_UNDERSCORE_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some __text");
        let expected_result = String::from("some __text");
        let actual_result: String = process_single_char_formats(&input_str, ITALICS_UNDERSCORE_TAG);
        assert_eq!(actual_result, expected_result);
    }
}
#[cfg(test)]
mod italics_asterisk_test {
    fn is_asterisk_token(c: char) -> bool {
        match c {
            '*' => true,
            _ => false,
        }
    }
    use super::single_char_parser::HtmlTag;
    use super::single_char_parser::*;
    use super::ITALICS_ASTERISK_TAG;
    #[test]
    fn convert_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *text*");
        let expected_result = String::from("some <i>text</i>");
        let actual_result = process_single_char_formats(&input_str, ITALICS_ASTERISK_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_no_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("plain text");
        let expected_result = String::from("plain text");
        let actual_result = process_single_char_formats(&input_str, ITALICS_ASTERISK_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *text *");
        let expected_result = String::from("some <i>text </i>");
        let actual_result = process_single_char_formats(&input_str, ITALICS_ASTERISK_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some * text *");
        let expected_result = String::from("some * text *");
        let actual_result: String = process_single_char_formats(&input_str, ITALICS_ASTERISK_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn invalid_single_star() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some * text");
        let expected_result = String::from("some * text");
        let actual_result: String = process_single_char_formats(&input_str, ITALICS_ASTERISK_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some **text");
        let expected_result = String::from("some **text");
        let actual_result: String = process_single_char_formats(&input_str, ITALICS_ASTERISK_TAG);
        assert_eq!(actual_result, expected_result);
    }
}

#[cfg(test)]
mod code_snippet_tests {
    use super::single_char_parser::HtmlTag;
    use super::single_char_parser::*;
    fn is_code_token(c: char) -> bool {
        match c {
            '`' => true,
            _ => false,
        }
    }
    use super::CODE_TAG;
    #[test]
    fn convert_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some `text`");
        let expected_result = String::from("some <code>text</code>");
        let actual_result: String = process_single_char_formats(&input_str, CODE_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_no_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("plain text");
        let expected_result = String::from("plain text");
        let actual_result: String = process_single_char_formats(&input_str, CODE_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some `text `");
        let expected_result = String::from("some <code>text </code>");
        let actual_result: String = process_single_char_formats(&input_str, CODE_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ` text `");
        let expected_result = String::from("some ` text `");
        let actual_result: String = process_single_char_formats(&input_str, CODE_TAG);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ``text");
        let expected_result = String::from("some ``text");
        let actual_result: String = process_single_char_formats(&input_str, CODE_TAG);
        assert_eq!(actual_result, expected_result);
    }
}

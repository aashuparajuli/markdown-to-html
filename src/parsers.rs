
use crate::single_char_pattern::single_char_parser::HtmlTag;
use crate::single_char_pattern::single_char_parser::process_single_char_formats;

mod inline_code {
    use super::HtmlTag;
    use super::process_single_char_formats;
    fn is_code_token(c: char) -> bool {
        match c {
            '`' => true,
            _ => false,
        }
    }
    const CODE_TAG: HtmlTag = HtmlTag {
        opening: "<code>",
        closing: "</code>",
    };
    pub fn process_inline_code(str: &str) -> String {
        process_single_char_formats(str, is_code_token, CODE_TAG)
    }
}
mod italics {
    use super::HtmlTag;
    use super::process_single_char_formats;
    fn is_asterisk_token(c: char) -> bool {
        match c {
            '*' => true,
            _ => false,
        }
    }
    fn is_underscore_token(c: char) -> bool {
        match c {
            '_' => true,
            _ => false,
        }
    }
    pub const ITALICS_TAG: HtmlTag = HtmlTag {
        opening: "<i>",
        closing: "</i>",
    };
    pub fn process_asterisk(str: &str) -> String {
        process_single_char_formats(str, is_asterisk_token, ITALICS_TAG)
    }
    pub fn process_underscore(str: &str) -> String {
        process_single_char_formats(str, is_underscore_token, ITALICS_TAG)
    }
}

#[cfg(test)]
mod italics_underscore_test {
    use super::italics::process_underscore;

    #[test]
    fn convert_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _text_");
        let expected_result = String::from("some <i>text</i>");
        let actual_result: String = process_underscore(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_no_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("plain text");
        let expected_result = String::from("plain text");    
            let actual_result: String = process_underscore(&input_str);

        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _text _");
        let expected_result = String::from("some <i>text </i>");
        let actual_result: String = process_underscore(&input_str);

        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _ text _");
        let expected_result = String::from("some _ text _");
        let actual_result: String = process_underscore(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some __text");
        let expected_result = String::from("some __text");
        let actual_result: String = process_underscore(&input_str);

        assert_eq!(actual_result, expected_result);
    }
}

#[cfg(test)]
mod italics_asterisk_test {
    use super::italics::process_asterisk;

    #[test]
    fn convert_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *text*");
        let expected_result = String::from("some <i>text</i>");
        let actual_result = process_asterisk(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_no_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("plain text");
        let expected_result = String::from("plain text");
        let actual_result = process_asterisk(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *text *");
        let expected_result = String::from("some <i>text </i>");
        let actual_result = process_asterisk(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some * text *");
        let expected_result = String::from("some * text *");
        let actual_result = process_asterisk(&input_str);

        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some **text");
        let expected_result = String::from("some **text");
        let actual_result = process_asterisk(&input_str);

        assert_eq!(actual_result, expected_result);
    }
}

#[cfg(test)]
mod code_snippet_tests {
    use super::inline_code::process_inline_code;
    #[test]
    fn convert_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some `text`");
        let expected_result = String::from("some <code>text</code>");
        let actual_result: String = process_inline_code(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_no_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("plain text");
        let expected_result = String::from("plain text");
        let actual_result: String = process_inline_code(&input_str);

        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some `text `");
        let expected_result = String::from("some <code>text </code>");
        let actual_result: String = process_inline_code(&input_str);

        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ` text `");
        let expected_result = String::from("some ` text `");
        let actual_result: String = process_inline_code(&input_str);

        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ``text");
        let expected_result = String::from("some ``text");
        let actual_result: String = process_inline_code(&input_str);

        assert_eq!(actual_result, expected_result);
    }
}

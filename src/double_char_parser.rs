use crate::{single_char_pattern::single_char_parser::HtmlTag, double_char_tokenizer::parse_double_char};


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
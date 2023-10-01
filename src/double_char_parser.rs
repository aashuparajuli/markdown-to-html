pub fn parse_bold(s: &str) -> String {
    const BOLD_ASTERISK_TAG: HtmlTag = HtmlTag {
        opening_tag: "<b>",
        closing_tag: "</b>",
       matching_char: '*',
   };
    //next step: don't want to pass BOLD_ASTERISK_TAG into
    parse_double_char(s, &BOLD_ASTERISK_TAG)
    
}
#[cfg(test)]
mod bold_tests {
    use super::parse_bold;
    #[test]
    fn convert_bold() {
        let input_str = String::from("some **text**");
        let expected_result = String::from("some <b>text</b>");
        let actual_result = parse_bold(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_bold_invalid_one() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ** text* *");
        let expected_result = String::from("some ** text* *");
        let actual_result: String = parse_bold(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn invalid_double_spaces() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ** text **");
        let expected_result = String::from("some ** text **");
        let actual_result: String = parse_bold(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn valid_single_spaces() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some **text** ");
        let expected_result = String::from("some <b>text</b> ");
        let actual_result: String = parse_bold(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_bold_invalid_two() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some * * text**");
        let expected_result = String::from("some * * text**");
        let actual_result: String = parse_bold(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_bold_invalid_three() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some **text");
        let expected_result = String::from("some **text");
        let actual_result: String = parse_bold(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_bold_invalid_four() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some **text\n");
        let expected_result = String::from("some **text\n");
        let actual_result: String = parse_bold(&input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_bold_invalid_five() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some **text*\n");
        let expected_result = String::from("some **text*\n");
        let actual_result: String = parse_bold(&input_str);
        assert_eq!(actual_result, expected_result);
    }
}

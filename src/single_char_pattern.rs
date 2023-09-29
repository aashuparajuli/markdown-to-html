trait FormattingToken {
    fn get_text(&self) -> String;
}

enum ItalicsAsteriskState {
    Italics,
    Plaintext,
}

//basic ideas: user passes in an enum that impl FormattingToken
//then all other operations work the same

//or user passes in a character, then a TextState is built off that
//or user passes in a function to check if a character is a token, then that is used to build a function
mod single_char_parser {
    use super::FormattingToken;
    trait FormattedText<'a> {
        fn get_text(&'a self) -> String;
        fn is_formatting_token(&self, c: char) -> bool;
    }
    mod italics_underscore {
        use super::FormattedText;
        pub struct ItalicsUnderscoreText<'a> {
            formatted: bool,    //is either formatted or not
            substring: &'a str, //the text to format
        }
        impl FormattedText<'_> for ItalicsUnderscoreText<'_> {
            fn get_text<'a>(&'a self) -> String {
                match self.formatted {
                    true => format!("<i>{}</i>", self.substring),
                    false => self.substring.to_string(),
                }
            }
            fn is_formatting_token(&self, c: char) -> bool {
                match c {
                    '_' => true,
                    _ => false,
                }
            }
        }
        impl ItalicsUnderscoreText<'_> {
            pub fn new(formatted: bool, substring: &str) -> ItalicsUnderscoreText {
                ItalicsUnderscoreText {
                    formatted,
                    substring,
                }
            }
        }
    }
    mod italics_asterisk {
        use super::FormattedText;
        pub struct ItalicsAsteriskText<'a> {
            formatted: bool, //is either formatted or not. FUTURE: convert this into enum so that all patterns can be parsed simaeltaenesouly
            substring: &'a str, //the text to format
        }
        impl FormattedText<'_> for ItalicsAsteriskText<'_> {
            fn get_text<'a>(&'a self) -> String {
                match self.formatted {
                    true => format!("<i>{}</i>", self.substring),
                    false => self.substring.to_string(),
                }
            }
            fn is_formatting_token(c: char) -> bool {
                match c {
                    '*' => true,
                    _ => false,
                }
            }
        }
        impl ItalicsAsteriskText<'_> {
            pub fn new(formatted: bool, substring: &str) -> ItalicsAsteriskText {
                ItalicsAsteriskText {
                    formatted,
                    substring,
                }
            }
        }
    }
    fn is_formatting_token(c: char) -> bool {
        match c {
            '*' => true,
            _ => false,
        }
    }
    pub struct FormatText {
        formatted: bool,
        substring: &str,
    }
    impl FormatText {
        fn new(formatted: bool, substring: &str) -> FormatText {
            ItalicsAsteriskText {
                formatted,
                substring,
            }
        }
    }
    fn get_text(&f: FormatText) -> String {
        match f.formatted {
            true => format!("<i>{}</i>", self.substring),
            false => self.substring.to_string(),
        }
    }
    pub fn process_single_char_formats(str: &str) -> String {
        //make it generic over any type that implements
        let mut result: String = String::new();
        let mut stack: Vec<&dyn FormattedText> = Vec::new();
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
            if parsing_formatted_text
                && (c == ' ' || is_formatting_token(c))
                && start_idx == curr_idx
            {
                //move start_idx backwards so that the previously captured '*' is captured in plaintext
                start_idx -= 1;
                //switch to parsing italics
                parsing_formatted_text = false;
            }
            if parsing_formatted_text && is_formatting_token(c) {
                //construct a FormattedText struct storing TextState::Italics, append it to the stack
                let italics_text = ItalicsUnderscoreText::new(true, &str[start_idx..curr_idx]);
                stack.push(&italics_text);
                start_idx = curr_idx;
                parsing_formatted_text = false;
            } else if !parsing_formatted_text && is_formatting_token(c) {
                //construct a FormattedText struct storing TextState::Plaintext, append it to the stack
                //let italics_text = FormattedText::new(TextState::Plaintext, start_idx, curr_idx);
                let italics_text: ItalicsUnderscoreText =
                    ItalicsUnderscoreText::new(false, &str[start_idx..curr_idx]);

                stack.push(&italics_text);
                //increment start pointer
                start_idx = curr_idx + 1;
                //switch into parsing italics mode
                parsing_formatted_text = true;
            }
        }
        //append any strings that have not been completed yet
        if parsing_formatted_text {
            //println!("found unmatched asterisk");
            let plain_text = FormatText::new(false, &str[start_idx - 1..]);
            stack.push(&mut plain_text);
        } else if start_idx != str.len() - 1 {
            //if a plaintext substring reaches the end of the fullstring, then push the entire substring to the stack
            //println!("found unterminated plain text");
            let plain_text = FormatText::new(false, &str[start_idx..]);
            stack.push(&mut plain_text);
        }
        stack
            .iter()
            .for_each(|state| result.push_str(&(state).get_text()));
        result
    }

    #[cfg(test)]
    mod italics_underscore_test {
        use super::*;
        #[test]
        fn convert_italics() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some _text_");
            let expected_result = String::from("some <i>text</i>");
            let actual_result = process_single_char_formats(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_no_italics() {
            //string with space before pound sign should not be converted
            let input_str = String::from("plain text");
            let expected_result = String::from("plain text");
            let actual_result = process_single_char_formats(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_italics_2() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some _text _");
            let expected_result = String::from("some <i>text </i>");
            let actual_result = process_single_char_formats(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_italics_invalid() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some _ text _");
            let expected_result = String::from("some _ text _");
            let actual_result: String = process_single_char_formats(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_italics_invalid_2() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some __text");
            let expected_result = String::from("some __text");
            let actual_result: String = process_single_char_formats(&input_str);
            assert_eq!(actual_result, expected_result);
        }
    }
    mod italics_asterisk_test {
        use super::*;
        #[test]
        fn convert_italics() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some *text*");
            let expected_result = String::from("some <i>text</i>");
            let actual_result = process_single_char_formats(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_no_italics() {
            //string with space before pound sign should not be converted
            let input_str = String::from("plain text");
            let expected_result = String::from("plain text");
            let actual_result = process_single_char_formats(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_italics_2() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some *text *");
            let expected_result = String::from("some <i>text </i>");
            let actual_result = process_single_char_formats(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_italics_invalid() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some * text *");
            let expected_result = String::from("some * text *");
            let actual_result: String = process_single_char_formats(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_italics_invalid_2() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some **text");
            let expected_result = String::from("some **text");
            let actual_result: String = process_single_char_formats(&input_str);
            assert_eq!(actual_result, expected_result);
        }
    }
    // #[cfg(test)]
    // mod buffer_tests {
    //     use super::*;
    //     #[test]
    //     fn second_last() {
    //         //valid use of second_last
    //         let first_state = ItalicsUnderscoreState::Italics;
    //         let second_state = ItalicsUnderscoreState::Plaintext;
    //         let buffer: Vec<ItalicsUnderscoreState> = vec![first_state, second_state];
    //         assert!(buffer.second_last().is_some());
    //     }
    //     #[test]
    //     fn second_last_invalid() {
    //         let first_state = ItalicsUnderscoreState::Plaintext;
    //         let second_state = ItalicsUnderscoreState::Italics;
    //         let buffer: Vec<ItalicsUnderscoreState> = vec![first_state, second_state];
    //         assert!(buffer.second_last().is_none());
    //     }
    // }
}

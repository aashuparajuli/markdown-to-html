fn is_underscore(c: char) -> bool {
    match c {
        '_' => true,
        _ => false,
    }
}
trait FormattingToken {
    fn is_token(c: char) -> bool;
    fn get_formatted_text(s: &str) -> String;
}
#[derive(Clone, Debug)]
enum TextState {
    Formatted,
    Plaintext,
}
impl FormattingToken for TextState {
    fn is_token(c: char) -> bool {
        match c {
            '_' => true,
            _ => false,
        }
    }
    fn get_formatted_text(&self, s: &str) -> String {
        match self {
            TextState::Formatted => {
                format!("<i>{}</i>", &full_string[self.start_idx..self.end_idx])
            }
            TextState::Plaintext => full_string[self.start_idx..self.end_idx].to_string(),
        }
    }
}
trait Stack {
    fn second_last(&self) -> Option<&TextState>;
}
impl Stack for Vec<TextState> {
    fn second_last(&self) -> Option<&TextState> {
        if self.len() < 2 {
            return None;
        }
        match self[self.len() - 2] {
            TextState::Formatted => Some(&TextState::Formatted), //return italics
            TextState::Plaintext => None,
        }
    }
}

mod single_char_parser {
    use super::is_underscore;

    use super::Plaintext;

    use super::Formatted;

    use super::TextState;
    impl FormattedText {
        fn new(format: TextState, start_idx: usize, end_idx: usize) -> FormattedText {
            FormattedText {
                format,
                start_idx,
                end_idx,
            }
        }
        fn get_text<'a>(&'a self, full_string: &'a str) -> String {
            //add the extra text formatted using format!
            match self.format {
                TextState::Formatted => {
                    format!("<i>{}</i>", &full_string[self.start_idx..self.end_idx])
                }
                TextState::Plaintext => full_string[self.start_idx..self.end_idx].to_string(),
            }
        }
    }

    fn process_italics_underscore(str: &str) -> String {
        let mut result: String = String::new();
        let mut stack: Vec<FormattedText> = Vec::new();
        let mut parsing_formatted_text: bool = false;
        let mut start_idx: usize = 0;
        let mut _current_state: TextState = TextState::Plaintext;
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
            if parsing_formatted_text && (c == ' ' || is_underscore(c)) && start_idx == curr_idx {
                //move start_idx backwards so that the previously captured '*' is captured in plaintext
                start_idx -= 1;
                //switch to parsing italics
                parsing_formatted_text = false;
            }
            if parsing_formatted_text && is_underscore(c) {
                //construct a FormattedText struct storing TextState::Italics, append it to the stack
                let italics_text = FormattedText::new(TextState::Formatted, start_idx, curr_idx);
                stack.push(italics_text);
                start_idx = curr_idx;
                parsing_formatted_text = false;
            } else if !parsing_formatted_text && is_underscore(c) {
                //construct a FormattedText struct storing TextState::Plaintext, append it to the stack
                let italics_text = FormattedText::new(TextState::Plaintext, start_idx, curr_idx);
                stack.push(italics_text);
                //increment start pointer
                start_idx = curr_idx + 1;
                //switch into parsing italics mode
                parsing_formatted_text = true;
            }
        }
        //append any strings that have not been completed yet
        if parsing_formatted_text {
            //println!("found unmatched asterisk");
            let plain_text = FormattedText::new(TextState::Plaintext, start_idx - 1, str.len());
            stack.push(plain_text);
        } else if start_idx != str.len() - 1 {
            //if a plaintext substring reaches the end of the fullstring, then push the entire substring to the stack
            //println!("found unterminated plain text");
            let plain_text = FormattedText::new(TextState::Plaintext, start_idx, str.len());
            stack.push(plain_text);
        }
        stack
            .iter()
            .for_each(|state: &FormattedText| result.push_str(&state.get_text(&str)));
        result
    }

    #[cfg(test)]
    mod italics_tests {
        use super::*;
        #[test]
        fn convert_italics() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some _text_");
            let expected_result = String::from("some <i>text</i>");
            let actual_result = process_italics_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_no_italics() {
            //string with space before pound sign should not be converted
            let input_str = String::from("plain text");
            let expected_result = String::from("plain text");
            let actual_result = process_italics_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_italics_2() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some _text _");
            let expected_result = String::from("some <i>text </i>");
            let actual_result = process_italics_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_italics_invalid() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some _ text _");
            let expected_result = String::from("some _ text _");
            let actual_result: String = process_italics_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
        #[test]
        fn convert_italics_invalid_2() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some __text");
            let expected_result = String::from("some __text");
            let actual_result: String = process_italics_underscore(&input_str);
            assert_eq!(actual_result, expected_result);
        }
    }

    #[cfg(test)]
    mod buffer_tests {
        use super::*;
        #[test]
        fn second_last() {
            //valid use of second_last
            let first_state = TextState::Formatted;
            let second_state = TextState::Plaintext;
            let buffer: Vec<TextState> = vec![first_state, second_state];
            assert!(buffer.second_last().is_some());
        }
        #[test]
        fn second_last_invalid() {
            let first_state = TextState::Plaintext;
            let second_state = TextState::Formatted;
            let buffer: Vec<TextState> = vec![first_state, second_state];
            assert!(buffer.second_last().is_none());
        }
    }
}

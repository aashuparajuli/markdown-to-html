#[derive(Clone, Debug)]
enum TextState {
    Italics,
    Plaintext,
}
trait FormattingToken {
    fn get_text(&self, s: &str) -> String;
}
impl FormattingToken for TextState {
    fn get_text(&self, s: &str) -> String {
        match self {
            TextState::Italics => format!("<i>{}</i>", s),
            TextState::Plaintext => s.to_string(),
        }
    }
}
//basic ideas: user passes in an enum that impl FormattingToken
//then all other operations work the same
mod single_char_parser {
    use super::FormattingToken;
    use super::TextState;
    trait Stack {
        fn second_last(&self) -> Option<&TextState>;
    }
    impl Stack for Vec<TextState> {
        fn second_last(&self) -> Option<&TextState> {
            if self.len() < 2 {
                return None;
            }
            match self[self.len() - 2] {
                TextState::Italics => Some(&TextState::Italics), //return italics
                TextState::Plaintext => None,
            }
        }
    }

    struct FormattedText<'a> {
        format: TextState,
        substring: &'a str,
    }
    impl FormattedText<'_> {
        fn new(format: TextState, substring: &str) -> FormattedText {
            FormattedText { format, substring }
        }
        fn get_text<'a>(&'a self) -> String {
            //add the extra text formatted using format!
            self.format.get_text(self.substring)
            // match self.format {
            //     TextState::Italics => {
            //         format!("<i>{}</i>", self.substring)
            //     }
            //     TextState::Plaintext => self.substring.to_string(),
            // }
        }
    }
    pub fn process_italics_underscore(str: &str) -> String {
        let mut result: String = String::new();
        let mut stack: Vec<FormattedText> = Vec::new();
        let mut parsing_italics: bool = false;
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
            if parsing_italics && (c == ' ' || c == '_') && start_idx == curr_idx {
                //move start_idx backwards so that the previously captured '*' is captured in plaintext
                start_idx -= 1;
                //switch to parsing italics
                parsing_italics = false;
            }
            if parsing_italics && c == '_' {
                //construct a FormattedText struct storing TextState::Italics, append it to the stack
                let italics_text =
                    FormattedText::new(TextState::Italics, &str[start_idx..curr_idx]);
                stack.push(italics_text);
                start_idx = curr_idx;
                parsing_italics = false;
            } else if !parsing_italics && c == '_' {
                //construct a FormattedText struct storing TextState::Plaintext, append it to the stack
                //let italics_text = FormattedText::new(TextState::Plaintext, start_idx, curr_idx);
                let italics_text: FormattedText =
                    FormattedText::new(TextState::Plaintext, &str[start_idx..curr_idx]);

                stack.push(italics_text);
                //increment start pointer
                start_idx = curr_idx + 1;
                //switch into parsing italics mode
                parsing_italics = true;
            }
        }
        //append any strings that have not been completed yet
        if parsing_italics {
            //println!("found unmatched asterisk");
            let plain_text = FormattedText::new(TextState::Plaintext, &str[start_idx - 1..]);
            stack.push(plain_text);
        } else if start_idx != str.len() - 1 {
            //if a plaintext substring reaches the end of the fullstring, then push the entire substring to the stack
            //println!("found unterminated plain text");
            let plain_text = FormattedText::new(TextState::Plaintext, &str[start_idx..]);
            stack.push(plain_text);
        }
        stack
            .iter()
            .for_each(|state: &FormattedText| result.push_str(&state.get_text()));
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
            let first_state = TextState::Italics;
            let second_state = TextState::Plaintext;
            let buffer: Vec<TextState> = vec![first_state, second_state];
            assert!(buffer.second_last().is_some());
        }
        #[test]
        fn second_last_invalid() {
            let first_state = TextState::Plaintext;
            let second_state = TextState::Italics;
            let buffer: Vec<TextState> = vec![first_state, second_state];
            assert!(buffer.second_last().is_none());
        }
    }
}

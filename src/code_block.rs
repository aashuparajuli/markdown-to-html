#[derive(Clone, Debug)]
enum TextState {
    CodeSnippet,
    Plaintext,
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
            TextState::CodeSnippet => Some(&TextState::CodeSnippet), //return italics
            TextState::Plaintext => None,
        }
    }
}

struct FormattedText {
    format: TextState,
    start_idx: usize,
    end_idx: usize,
}
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
            TextState::CodeSnippet => {
                format!(
                    "<code>{}</code>",
                    &full_string[self.start_idx..self.end_idx]
                )
            }
            TextState::Plaintext => full_string[self.start_idx..self.end_idx].to_string(),
        }
    }
}
pub fn process_inline_code(str: String) -> String {
    let mut result: String = String::new();
    let mut stack: Vec<FormattedText> = Vec::new();
    let mut parsing_italics: bool = false;
    let mut start_idx: usize = 0;
    let mut _current_state: TextState = TextState::Plaintext;
    if str.is_empty() {
        return str;
    }
    for (curr_idx, c) in str.chars().enumerate() {
        //initially:currently_matching = false;
        /*cases for string matching:
           !currently_matching && c != '*' => endIdx +=1
           !currently_matching && c == '*' => {
                push the substring to result: result.push_str(&str[startIdx..endIdx])
                currently_matching = true
                startIdx =
            }
            if currently_matching && c == '*' => {

            }
        */
        /*
        cases:
        - not in italics, adding a char
        - switching into italics
        - in italics, adding a char
        - switching out of italics
        */
        //switching in or out of code snippets
        // match (parsing_plaintext, c) {
        //     (false, '*') => {}
        //     (true, '*')  => {}
        //     (false, '*') => {}
        // };
        if parsing_italics && (c == ' ' || c == '`') && start_idx == curr_idx {
            //move start_idx backwards so that the previously captured '*' is captured in plaintext
            start_idx -= 1;
            //switch to parsing italics
            parsing_italics = false;
        }
        if parsing_italics && c == '`' {
            //construct a FormattedText struct storing TextState::CodeSnippet, append it to the stack
            let italics_text = FormattedText::new(TextState::CodeSnippet, start_idx, curr_idx);
            stack.push(italics_text);
            start_idx = curr_idx;
            parsing_italics = false;
        } else if !parsing_italics && c == '`' {
            //construct a FormattedText struct storing TextState::Plaintext, append it to the stack
            let italics_text = FormattedText::new(TextState::Plaintext, start_idx, curr_idx);
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
mod code_snippet_tests {
    use super::*;
    #[test]
    fn convert_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some `text`");
        let expected_result = String::from("some <code>text</code>");
        let actual_result = process_inline_code(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_no_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("plain text");
        let expected_result = String::from("plain text");
        let actual_result = process_inline_code(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some `text `");
        let expected_result = String::from("some <code>text </code>");
        let actual_result = process_inline_code(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ` text `");
        let expected_result = String::from("some ` text `");
        let actual_result: String = process_inline_code(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ``text");
        let expected_result = String::from("some ``text");
        let actual_result: String = process_inline_code(input_str);
        assert_eq!(actual_result, expected_result);
    }
}

#[cfg(test)]
mod buffer_tests {
    use super::*;
    #[test]
    fn second_last() {
        //valid use of second_last
        let first_state = TextState::CodeSnippet;
        let second_state = TextState::Plaintext;
        let buffer: Vec<TextState> = vec![first_state, second_state];
        assert!(buffer.second_last().is_some());
    }
    #[test]
    fn second_last_invalid() {
        let first_state = TextState::Plaintext;
        let second_state = TextState::CodeSnippet;
        let buffer: Vec<TextState> = vec![first_state, second_state];
        assert!(buffer.second_last().is_none());
    }
}
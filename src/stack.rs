#[derive(Clone, Debug)]
enum TextState {
    Italics,
    Plaintext(String),
}
impl TextState {
    fn get_text(&self) -> String {
        match self {
            TextState::Italics => String::from("*"),
            TextState::Plaintext(s) => s.to_string(),
        }
    }
}
trait Stack {
    fn second_last(&self) -> Option<&TextState>;
    fn append_char(&mut self, c: char);
}
impl Stack for Vec<TextState> {
    fn second_last(&self) -> Option<&TextState> {
        if self.len() < 2 {
            return None;
        }
        match self[self.len() - 2] {
            TextState::Italics => Some(&TextState::Italics), //return italics
            TextState::Plaintext(_) => None,
        }
    }

    fn append_char(&mut self, c: char) {
        //create a plaintext struct at the top of the stack if needed
        if let Some(TextState::Plaintext(ref x)) = self.last() {
            //maybe I only need to call this once, not every time I want to push a char
            let len = self.len();
            let mut current_string: String = x.clone();
            current_string.push(c);
            (*self)[len - 1] = TextState::Plaintext(current_string);
        } else {
            let current_string: String = String::from(c); //consider using String::with_capacity here
            (*self).push(TextState::Plaintext(current_string));
        }
    }
}

pub fn process_italics(str: String) -> String {
    let mut result: String = String::new();
    let mut stack: Vec<TextState> = Vec::new();
    let mut currently_matching = false;
    for c in str.chars() {
        /*
        cases:
        - not in italics, adding a char
        - switching into italics
        - in italics, adding a char
        - switching out of italics
        */
        //switching in or out of italics

        if c == '*' {
            //switching out of italics
            if let Some(&TextState::Italics) = stack.second_last() {
                let string_to_format: String = stack.pop().unwrap().get_text();
                stack.pop().unwrap(); //pop the TextState::Italics off of the stack
                                      //append the new string
                let formatted_string = format!("<i>{}</i>", string_to_format);
                stack.push(TextState::Plaintext(formatted_string));
            } else {
                //switching into italics
                stack.push(TextState::Italics)
            }
        } else {
            //a space directly after italics will escape the italics
            if c == ' ' {
                if let Some(&TextState::Italics) = stack.last() {
                    stack.pop();
                    stack.append_char('*');
                }
            }
            stack.append_char(c);
        }
    }
    //concatenate the stack to generate the final result string
    stack
        .iter()
        .for_each(|state| result.push_str(&state.get_text()));
    result
}

#[cfg(test)]
mod italics_tests {
    use super::*;
    #[test]
    fn convert_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *text*");
        let expected_result = String::from("some <i>text</i>");
        let actual_result = process_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *text *");
        let expected_result = String::from("some <i>text </i>");
        let actual_result = process_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some * text *");
        let expected_result = String::from("some * text *");
        let actual_result: String = process_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some **text");
        let expected_result = String::from("some **text");
        let actual_result: String = process_italics(input_str);
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
        let second_state = TextState::Plaintext(String::from("second"));
        let buffer: Vec<TextState> = vec![first_state, second_state];
        assert!(buffer.second_last().is_some());
    }
    #[test]
    fn second_last_invalid() {
        let first_state = TextState::Plaintext(String::from("second"));
        let second_state = TextState::Italics;
        let buffer: Vec<TextState> = vec![first_state, second_state];
        assert!(buffer.second_last().is_none());
    }
}

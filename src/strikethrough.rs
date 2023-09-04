#[derive(PartialEq, Copy, Clone, Debug)]
enum TextStates {
    StrikethroughOne,
    StrikethroughTwo,
    StrikethroughThree,
    Plaintext,
}
impl TextStates {
    fn new() -> Self {
        TextStates::Plaintext
    }
}
enum CharTypes {
    NewLine,
    Space,
    Strikethrough,
    Text,
}
impl CharTypes {
    fn new(c: char) -> Self {
        match c {
            ' ' => CharTypes::Space,
            '~' => CharTypes::Strikethrough,
            '\n' => CharTypes::NewLine,
            _ => CharTypes::Text,
        }
    }
}
struct Buffer {
    buffer: String,
    state: TextStates,
}
impl Buffer {
    fn new() -> Self {
        Buffer {
            buffer: String::new(),
            state: TextStates::new(),
        }
    }
    fn get_string(&self) -> String {
        match self.state {
            TextStates::Plaintext => self.buffer.clone(),
            TextStates::StrikethroughOne => format!("~{}", self.buffer),
            TextStates::StrikethroughTwo => format!("~~{}", self.buffer),
            TextStates::StrikethroughThree => format!("~~{}~", self.buffer),
        }
    }
    fn escape(&mut self, c: char) -> String {
        let escape_string = match self.state {
            TextStates::StrikethroughOne => format!("~{c}"),
            TextStates::StrikethroughTwo => format!("~~{}{c}", self.buffer),
            TextStates::StrikethroughThree => format!("~~{}~{c}", self.buffer),
            TextStates::Plaintext => self.buffer.clone(),
        };
        self.buffer.clear();
        escape_string
    }
    fn add_char(&mut self, c: char) -> String {
        let next_char: CharTypes = CharTypes::new(c);
        let mut return_string: String = String::new();
        self.state = match (self.state, next_char) {
            (_, CharTypes::NewLine) => {
                return_string = self.escape(c);
                TextStates::Plaintext
            }
            (TextStates::Plaintext, CharTypes::Strikethrough) => {
                //flush the current buffer
                return_string = String::clone(&self.buffer);
                self.buffer.clear();
                TextStates::StrikethroughOne
            }
            (TextStates::Plaintext, _) => {
                self.buffer.push(c);
                TextStates::Plaintext
            }
            (TextStates::StrikethroughOne, CharTypes::Strikethrough) => {
                TextStates::StrikethroughTwo
            }
            (TextStates::StrikethroughOne, _) => {
                //escaping from underscore, return the current buffer to be displayed
                return_string = self.escape(c);
                TextStates::Plaintext
            }
            (TextStates::StrikethroughTwo, CharTypes::Strikethrough) => {
                TextStates::StrikethroughThree
            }
            (TextStates::StrikethroughTwo, _) => {
                //handles Space and Text cases
                self.buffer.push(c);
                TextStates::StrikethroughTwo
            }
            (TextStates::StrikethroughThree, CharTypes::Strikethrough) => {
                //When this branch  is reached, it is time to generate the text, with the bold tag,
                return_string = format!("<del>{}</del>", self.buffer);
                self.buffer.clear();
                TextStates::Plaintext
            }
            (TextStates::StrikethroughThree, _) => {
                return_string = self.escape(c);
                TextStates::Plaintext
            }
        };
        return_string
    }
}
pub fn process_strikethrough(str: String) -> String {
    let mut result: String = String::new();
    let mut _stack: Vec<&str> = Vec::new();
    //let mut buffer: String = String::new();
    let mut buffer = Buffer::new();
    let mut substring: String;
    let mut _current_state = TextStates::new();
    for c in str.chars() {
        /*
        cases:
        - not in italics, adding a char
        - switching into italics
        - in italics, adding a char
        - switching out of italics
        */
        substring = buffer.add_char(c);
        result.push_str(&substring);
    }
    //any data still in current_state should be output
    result.push_str(&buffer.get_string());
    result
}

#[cfg(test)]
mod strikethrough_tests {
    use super::*;
    #[test]
    fn convert_strikethrough() {
        let input_str = String::from("some ~~text~~");
        let expected_result = String::from("some <del>text</del>");
        let actual_result = process_strikethrough(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_strikethrough_invalid_one() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ~~ text~ ~");
        let expected_result = String::from("some ~~ text~ ~");
        let actual_result: String = process_strikethrough(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_strikethrough_invalid_two() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ~ ~ text~~");
        let expected_result = String::from("some ~ ~ text~~");
        let actual_result: String = process_strikethrough(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_strikethrough_invalid_three() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ~~text");
        let expected_result = String::from("some ~~text");
        let actual_result: String = process_strikethrough(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_strikethrough_invalid_four() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ~~text\n");
        let expected_result = String::from("some ~~text\n");
        let actual_result: String = process_strikethrough(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_strikethrough_invalid_five() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some ~~text~\n");
        let expected_result = String::from("some ~~text~\n");
        let actual_result: String = process_strikethrough(input_str);
        assert_eq!(actual_result, expected_result);
    }
}

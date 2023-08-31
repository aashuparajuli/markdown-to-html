pub fn process_italics(str: String) -> String {
    let mut result: String = String::new();
    let mut stack: Vec<&str> = Vec::new();
    let mut buffer: String = String::new();

    for c in str.chars() {
        /*
        cases:
        - not in italics, adding a char
        - switching into italics
        - in italics, adding a char
        - switching out of italics
        */
        //two consecutive '*' should convert to plaintext
        if c == '*' {
            //switching in or out of italics
            //if top of stack is *, then we are switching out of italics
            if stack.last() == Some(&"*") {
                //pop the asterisk, update the buffer correctly
                stack.pop();
                //update the buffer with italicized text
                buffer = format!("<i>{buffer}</i>");
            } else {
                //else, we are switching into italics
                stack.push("*"); //update the stack
            }
            result.push_str(buffer.as_str()); //push the current contents of the buffer
            buffer = String::new(); //reset the buffer to being empty
        } else if stack.last() == Some(&"*") && buffer.is_empty() && c == ' ' {
            //if top of stack is '*' and buffer is empty and current char is space, then we need to escape italics
            stack.pop();
            buffer.push('*');
            buffer.push(' ');
        } else {
            buffer.push(c);
        }
    }
    result.push_str(&buffer);
    if !stack.is_empty() {
        //push remaining characters onto the stack
        for substring in stack {
            result.push_str(substring);
        }
    }
    result
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum TextStates {
    BoldOne,
    BoldTwo,
    BoldThree,
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
    Underscore,
    Text,
}
impl CharTypes {
    fn new(c: char) -> Self {
        match c {
            ' ' => CharTypes::Space,
            '_' => CharTypes::Underscore,
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
            TextStates::BoldOne => format!("_{}", self.buffer),
            TextStates::BoldTwo => format!("__{}", self.buffer),
            TextStates::BoldThree => format!("__{}_", self.buffer),
        }
    }
    fn add_char(&mut self, c: char) -> String {
        let next_char: CharTypes = CharTypes::new(c);
        let mut return_string: String = String::new();
        self.state = match self.state {
            TextStates::Plaintext => match next_char {
                CharTypes::Underscore => {
                    //flush the current buffer
                    return_string = String::clone(&self.buffer);
                    //println!("Line 311: Plaintext to BoldOne:{}|", self.buffer);
                    self.buffer = String::new();
                    TextStates::BoldOne
                }
                _ => {
                    self.buffer.push(c);
                    TextStates::Plaintext
                }
            },
            TextStates::BoldOne => match next_char {
                CharTypes::Underscore => {
                    //println!("Line 318: BoldOne to BoldTwo:{c}");
                    TextStates::BoldTwo
                }
                _ => {
                    //println!("Line 321: BoldOne to Plaintext:{c}");
                    //escaping from underscore, return the current buffer to be displayed
                    return_string = format!("_{}{c}", self.buffer);
                    self.buffer = String::new();
                    TextStates::Plaintext
                }
            },
            TextStates::BoldTwo => match next_char {
                CharTypes::Underscore => TextStates::BoldThree,
                CharTypes::NewLine => {
                    return_string = format!("__{}{c}", self.buffer);
                    self.buffer = String::new();
                    TextStates::Plaintext
                }
                CharTypes::Space | CharTypes::Text => {
                    self.buffer.push(c);
                    TextStates::BoldTwo
                }
            },
            TextStates::BoldThree => {
                match next_char {
                    CharTypes::Underscore => {
                        //println!("Created a bold fragment");
                        return_string = format!("<b>{}</b>", self.buffer);
                        self.buffer = String::new();
                        TextStates::Plaintext
                    } //When this branch  is reached, it is time to generate the text, with the bold tag
                    CharTypes::NewLine => {
                        return_string = format!("__{}_\n", self.buffer);
                        self.buffer = String::new();
                        TextStates::Plaintext
                    }
                    CharTypes::Space | CharTypes::Text => {
                        return_string = format!("__{}_{c}", self.buffer);
                        self.buffer = String::new();
                        TextStates::Plaintext
                    }
                }
            }
        };
        return_string
    }
}
pub fn process_bold(str: String) -> String {
    let mut result: String = String::new();
    let mut _stack: Vec<&str> = Vec::new();
    //let mut buffer: String = String::new();
    let mut buffer = Buffer::new();
    let mut _current_state = TextStates::new();
    for c in str.chars() {
        /*
        cases:
        - not in italics, adding a char
        - switching into italics
        - in italics, adding a char
        - switching out of italics
        */
        //two consecutive '*' should convert to plaintext
        //println!("currState:{:?}, nextChar:{c}|", buffer.state);
        let res = buffer.add_char(c);
        result.push_str(&res);
        //current_state.transition(CharTypes::new(c));
        //println!("{:?}\n", buffer.state);
    }
    //any data still in current_state should be output
    result.push_str(&buffer.get_string());
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
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some * text *");
        let expected_result = String::from("some * text *");
        let actual_result: String = process_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
}

#[cfg(test)]
mod bold_tests {
    use super::*;
    #[test]
    fn convert_bold() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some __text__");
        let expected_result = String::from("some <b>text</b>");
        let actual_result = process_bold(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_bold_invalid_one() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some __ text_ _");
        let expected_result = String::from("some __ text_ _");
        let actual_result: String = process_bold(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_bold_invalid_two() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _ _ text__");
        let expected_result = String::from("some _ _ text__");
        let actual_result: String = process_bold(input_str);
        assert_eq!(actual_result, expected_result);
    }
}

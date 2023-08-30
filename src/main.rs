pub mod file_io {
    /**
     * Functions to read/write lines from a file
     */
    use std::fs::File;
    use std::io::Write;
    use std::io::{self, BufRead};
    use std::path::Path;

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    pub fn get_file_lines(filename: &str) -> Vec<String> {
        //let filename = "./hosts.txt";
        let mut file_lines: Vec<String> = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                let ip = line.unwrap();
                file_lines.push(ip);
            }
        }
        file_lines
    }

    pub fn write_line_to_file_true(str: &Vec<String>, filename: &str) {
        //let data = "Some data!";
        let mut f = File::create(filename).expect("Unable to create file");
        for s in str {
            f.write_all(s.as_bytes()).expect("Unable to write data");
        }
    }
    pub fn write_line_to_file(str: &str, file: &mut Vec<String>) {
        //write a line to a file
        //TODO: should write to file
        file.push(str.to_string());
    }
}

pub mod parse_line_formatting {
    use crate::{file_io, parse_text_formatting};
    /**
     * Module to parse markdown selectors that affect the entire line: lines: Headers, list elements
     * Currently supports: h1, h2, h3, unordered list, and unordered list
     */

    #[derive(PartialEq)]
    enum LineType {
        UnorderedList,
        OrderedList,
        Header1,
        Header2,
        Header3,
        Other,
    }
    /**
     * Input: a Vec<String> - one String for each line in the input file
     * For each String, check if there are any headers at the start of the file
     * if there are: surround the remainder of the string with the correct tag
     *
     * Returns a Vec<String> where each String has been converted to HTML code
     * Writes the resulting html to a file
     */
    pub fn parse_all_lines(lines: Vec<String>) -> Vec<String> {
        let mut proxy_file: Vec<String> = Vec::new();
        let mut current_line_state: LineType = LineType::Other;

        //process the current line, determine its state
        for line in lines {
            let (parsed_line, new_line_state) = determine_line_type(line);
            //format the other text in the string

            //parse and format the italics
            let parsed_line = parse_text_formatting::process_bold(parsed_line);
            let parsed_line = parse_text_formatting::process_italics(parsed_line);
            //add the line-level tags at the end
            let prefix = insert_list_start_or_end(&current_line_state, &new_line_state);
            let parsed_line: String = match new_line_state {
                LineType::UnorderedList => {
                    format!("{}<li>{}</li>\n", prefix, parsed_line)
                }
                LineType::OrderedList => {
                    format!("{}<li>{}</li>\n", prefix, parsed_line)
                }
                LineType::Header1 => {
                    format!("{}<h1>{}</h1>\n", prefix, parsed_line)
                }
                LineType::Header2 => {
                    format!("{}<h2>{}</h2>\n", prefix, parsed_line)
                }
                LineType::Header3 => {
                    format!("{}<h3>{}</h3>\n", prefix, parsed_line)
                }
                LineType::Other => {
                    format!("{}{}\n", prefix, parsed_line)
                }
            };

            file_io::write_line_to_file(&parsed_line, &mut proxy_file);
            current_line_state = new_line_state;
        }
        //close any tags that are still open:
        if current_line_state == LineType::OrderedList {
            let parsed_line = String::from("</ol>");
            file_io::write_line_to_file(&parsed_line, &mut proxy_file);
        } else if current_line_state == LineType::UnorderedList {
            let parsed_line = String::from("</ul>");
            file_io::write_line_to_file(&parsed_line, &mut proxy_file);
        }

        proxy_file
    }

    fn determine_line_type(line: String) -> (String, LineType) {
        if line.len() < 2 {
            (line, LineType::Other)
        } else if &line[0..2] == "# " {
            let remaining_str = &line[2..];
            (remaining_str.to_string(), LineType::Header1)
        } else if &line[0..3] == "## " {
            let remaining_str = &line[3..];
            (remaining_str.to_string(), LineType::Header2)
        } else if &line[0..4] == "### " {
            let remaining_str = &line[4..];
            (remaining_str.to_string(), LineType::Header3)
        } else if &line[0..2] == "- " {
            let remaining_str = &line[2..];
            (remaining_str.to_string(), LineType::UnorderedList)
        } else if &line[0..3] == "1. " {
            let remaining_str = &line[2..];
            (remaining_str.to_string(), LineType::OrderedList)
        } else {
            (line, LineType::Other)
        }
    }
    /*
    Parse all of the lines in the file
    for each line:
        - determine its type: header, unordered list, other
        - return the struct containing the LineState
        - generate the correct string
     */
    fn insert_list_start_or_end(current_line_state: &LineType, new_state: &LineType) -> String {
        if *current_line_state != LineType::UnorderedList && *new_state == LineType::UnorderedList {
            //we just started a bulleted list, so we need to insert a <ul> tag
            String::from("<ul>")
        } else if *current_line_state == LineType::UnorderedList
            && *new_state != LineType::UnorderedList
        {
            //we just exited a bulleted list, so we need to insert a </ul> tag
            String::from("</ul>")
        } else if *current_line_state != LineType::OrderedList
            && *new_state == LineType::OrderedList
        {
            //we just exited a bulleted list, so we need to insert a </ul> tag
            String::from("<ol>")
        } else if *current_line_state == LineType::OrderedList
            && *new_state != LineType::OrderedList
        {
            //we just exited a bulleted list, so we need to insert a </ul> tag
            String::from("</ol>")
        } else {
            String::new()
        }
    }

    pub fn process_headers(str: String) -> String {
        //if first characters are 'h1', then add the h1 tags
        if &str[0..2] == "# " {
            let remaining_str = &str[2..];
            format!("<h1>{}</h1>", remaining_str)
        } else if &str[0..3] == "## " {
            let remaining_str = &str[3..];
            format!("<h2>{}</h2>", remaining_str)
        } else if &str[0..4] == "### " {
            let remaining_str = &str[4..];
            format!("<h3>{}</h3>", remaining_str)
        } else {
            str
        }
    }
}

pub mod parse_text_formatting {

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
        fn transition(&mut self, next_char: CharTypes) {
            *self = match *self {
                TextStates::Plaintext => match next_char {
                    CharTypes::Underscore => TextStates::BoldOne,
                    _ => TextStates::Plaintext,
                },
                TextStates::BoldOne => match next_char {
                    CharTypes::Underscore => TextStates::BoldTwo,
                    _ => TextStates::Plaintext,
                },
                TextStates::BoldTwo => match next_char {
                    CharTypes::Underscore => TextStates::BoldThree,
                    CharTypes::NewLine => TextStates::Plaintext,
                    CharTypes::Space | CharTypes::Text => *self,
                },
                TextStates::BoldThree => {
                    match next_char {
                        CharTypes::Underscore => {
                            println!("Created a bold fragment");
                            TextStates::Plaintext
                        } //When this branch  is reached, it is time to generate the text, with the bold tag
                        CharTypes::NewLine => TextStates::Plaintext,
                        CharTypes::Space | CharTypes::Text => *self,
                    }
                }
            };
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
                        println!("Line 311: Plaintext to BoldOne:{}|", self.buffer);
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
                        println!("Line 318: BoldOne to BoldTwo:{c}");
                        TextStates::BoldTwo
                    }
                    _ => {
                        println!("Line 321: BoldOne to Plaintext:{c}");
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
                            println!("Created a bold fragment");
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
            println!("currState:{:?}, nextChar:{c}|", buffer.state);
            let res = buffer.add_char(c);
            result.push_str(&res);
            //current_state.transition(CharTypes::new(c));
            println!("{:?}\n", buffer.state);
        }
        //any data still in current_state should be output
        result.push_str(&buffer.get_string());
        result
    }
}

fn main() {
    let input_file_name = "./input/input.md";
    let output_file_name = "./output/output.html";
    let input_lines = file_io::get_file_lines(input_file_name); //get the lines from the file
    let output_lines = parse_line_formatting::parse_all_lines(input_lines); //process the lines

    for line in output_lines.clone() {
        println!("{}", line); //display the lines
    }
    file_io::write_line_to_file_true(&output_lines, output_file_name);

    let _italics_result: String =
        parse_text_formatting::process_italics(String::from("new *string*"));
    let _header_result: String =
        parse_line_formatting::process_headers(String::from("# new string"));
    // File hosts.txt must exist in the current path
}

#[cfg(test)]
mod italics_tests {
    use super::*;
    #[test]
    fn convert_italics() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *text*");
        let expected_result = String::from("some <i>text</i>");
        let actual_result = parse_text_formatting::process_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some * text *");
        let expected_result = String::from("some * text *");
        let actual_result: String = parse_text_formatting::process_italics(input_str);
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
        let actual_result = parse_text_formatting::process_bold(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_bold_invalid_one() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some __ text_ _");
        let expected_result = String::from("some __ text_ _");
        let actual_result: String = parse_text_formatting::process_bold(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_bold_invalid_two() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _ _ text__");
        let expected_result = String::from("some _ _ text__");
        let actual_result: String = parse_text_formatting::process_bold(input_str);
        assert_eq!(actual_result, expected_result);
    }
}

#[cfg(test)]
mod unordered_list_test {
    use super::*;
    #[test]
    fn test_one_line_list() {
        //string with space before pound sign should not be converted
        let file_lines: Vec<String> = vec![
            String::from("no list"),
            String::from("- list here"),
            String::from("-end list"),
        ];
        let expected_result: Vec<String> = vec![
            String::from("no list\n"),
            String::from("<ul><li>list here</li>\n"),
            String::from("</ul>-end list\n"),
        ];
        let actual_result = parse_line_formatting::parse_all_lines(file_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_result[0], expected_result[0]);
        assert_eq!(actual_result[1], expected_result[1]);
        assert_eq!(actual_result[2], expected_result[2]);
    }
    #[test]
    fn test_two_line_list() {
        //string with space before pound sign should not be converted
        let file_lines: Vec<String> = vec![
            String::from("no list"),
            String::from("- list here"),
            String::from("- another here"),
            String::from("end list"),
        ];
        let expected_result: Vec<String> = vec![
            String::from("no list\n"),
            String::from("<ul><li>list here</li>\n"),
            String::from("<li>another here</li>\n"),
            String::from("</ul>end list\n"),
        ];
        let actual_result = parse_line_formatting::parse_all_lines(file_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_result[0], expected_result[0]);
        assert_eq!(actual_result[1], expected_result[1]);
        assert_eq!(actual_result[2], expected_result[2]);
        assert_eq!(actual_result[3], expected_result[3]);
    }
}

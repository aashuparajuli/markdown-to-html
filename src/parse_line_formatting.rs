use crate::file_io;
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
    } else if &line[0..2] == "- " {
        let remaining_str = &line[2..];
        (remaining_str.to_string(), LineType::UnorderedList)
    } else if line.len() < 3 {
        (line, LineType::Other)
    } else if &line[0..3] == "## " {
        let remaining_str = &line[3..];
        (remaining_str.to_string(), LineType::Header2)
    } else if &line[0..3] == "1. " {
        let remaining_str = &line[2..];
        (remaining_str.to_string(), LineType::OrderedList)
    } else if line.len() < 4 {
        (line, LineType::Other)
    } else if &line[0..4] == "### " {
        let remaining_str = &line[4..];
        (remaining_str.to_string(), LineType::Header3)
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
    } else if *current_line_state != LineType::OrderedList && *new_state == LineType::OrderedList {
        //we just exited a bulleted list, so we need to insert a </ul> tag
        String::from("<ol>")
    } else if *current_line_state == LineType::OrderedList && *new_state != LineType::OrderedList {
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
}

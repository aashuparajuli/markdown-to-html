use crate::file_io;
use crate::parse_text_formatting;
use crate::stack;
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
        //let parsed_line = parse_text_formatting::process_italics(parsed_line);
        let parsed_line = stack::process_italics(parsed_line);
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
    match (current_line_state, new_state) {
        (LineType::UnorderedList, LineType::UnorderedList) => String::new(),
        (LineType::OrderedList, LineType::OrderedList) => String::new(),
        (_, LineType::UnorderedList) => String::from("<ul>"),
        (_, LineType::OrderedList) => String::from("<ol>"),
        (LineType::UnorderedList, _) => String::from("</ul>"),
        (LineType::OrderedList, _) => String::from("</ol>"),
        (_, _) => String::new(),
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
        let actual_result = parse_all_lines(file_lines);
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
        let actual_result = parse_all_lines(file_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_result[0], expected_result[0]);
        assert_eq!(actual_result[1], expected_result[1]);
        assert_eq!(actual_result[2], expected_result[2]);
        assert_eq!(actual_result[3], expected_result[3]);
    }
}

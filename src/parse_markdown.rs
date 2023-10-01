use crate::file_io::FileWriter;
use crate::single_char_parse::{italics_asterisk, italics_underscore, process_inline_code};

use crate::full_line_parsing::{LineType, determine_line_type, insert_list_start_or_end};
use crate::double_char_parse::{parse_bold_asterisk, parse_bold_underscore,parse_strikethrough};

/**
. * Module to parse markdown selectors that affect the entire line: lines: Headers, list elements
 * Currently supports: h1, h2, h3, unordered list, and unordered list
 */

/**
 * Input: a Vec<String> - one String for each line in the input file
 * For each String, check if there are any headers at the start of the file
 * if there are: surround the remainder of the string with the correct tag
 *
 * Returns a Vec<String> where each String has been converted to HTML code
 * Writes the resulting html to a file
 */

pub fn parse_all_lines(lines: Vec<String>, file_access: &mut dyn FileWriter) {
    let mut current_line_state: LineType = LineType::Other;

    //process the current line, determine its state
    for line in lines {
        let (parsed_line, new_line_state) = determine_line_type(&line);
        //format the other text in the string

        //parse and format the bold
        let parsed_line: String = parse_bold_asterisk(parsed_line); //parse bold with italics
        let parsed_line: String = parse_bold_underscore(&parsed_line); //parse bold with underscores
        // let parsed_line: String = bold::process_bold(parsed_line.to_string()); //parse bold with italics
        // let parsed_line: String = parse_bold_underscore::process_bold(parsed_line); //parse bold with underscores

        //parse strikethrough
        // let parsed_line: String = strikethrough::process_strikethrough(parsed_line);
        let parsed_line: String = parse_strikethrough(&parsed_line); //parse bold with underscores

        //parse italics under asterisk
        let parsed_line: String = italics_asterisk(&parsed_line);
        //parse italics using underscores
        let parsed_line: String = italics_underscore(&parsed_line);
        //parse and format inline code blocks
        let parsed_line: String = process_inline_code(&parsed_line);

        //add the line-level tags at the end
        let prefix = insert_list_start_or_end(&current_line_state, &new_line_state);
        let parsed_line = new_line_state.format_line(prefix, &parsed_line);
        file_access.write_line_to_file(&parsed_line);
        current_line_state = new_line_state;
    }
    //close any tags that are still open:
    if current_line_state == LineType::OrderedList {
        let parsed_line = String::from("</ol>");
        //file_io::write_line_to_file(&parsed_line, &mut proxy_file);
        file_access.write_line_to_file(&parsed_line);
    } else if current_line_state == LineType::UnorderedList {
        let parsed_line: String = String::from("</ul>");
        file_access.write_line_to_file(&parsed_line);
        //file_io::write_directly_to_file(&parsed_line, file_access);file_io::write_line_to_file(&parsed_line, &mut proxy_file);
    }
}
/*
Parse all of the lines in the file
for each line:
    - determine its type: header, unordered list, other
    - return the struct containing the LineState
    - generate the correct string
 */


#[cfg(test)]
mod unordered_list_test {
    use super::parse_all_lines;
    #[test]
    fn test_one_line_list() {
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> = vec![
            String::from("no list"),
            String::from("- list here"),
            String::from("-end list"), //this should not be converted into a list
        ];
        let expecte_html_lines: Vec<String> = vec![
            String::from("no list\n"),
            String::from("<ul><li>list here</li>\n"),
            String::from("</ul>-end list\n"),
        ];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expecte_html_lines[0]);
        assert_eq!(actual_html_lines[1], expecte_html_lines[1]);
        assert_eq!(actual_html_lines[2], expecte_html_lines[2]);
    }
    #[test]
    fn test_two_line_list() {
        //string with space before pound sign should not be converted
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> = vec![
            String::from("no list"),
            String::from("- first item"),
            String::from("- second item"),
            String::from("end list"),
        ];
        let expected_html_lines: Vec<String> = vec![
            String::from("no list\n"),
            String::from("<ul><li>first item</li>\n"),
            String::from("<li>second item</li>\n"),
            String::from("</ul>end list\n"),
        ];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expected_html_lines[0]);
        assert_eq!(actual_html_lines[1], expected_html_lines[1]);
        assert_eq!(actual_html_lines[2], expected_html_lines[2]);
        assert_eq!(actual_html_lines[3], expected_html_lines[3]);
    }
}
#[cfg(test)]
mod ordered_list {
    use super::parse_all_lines;
    #[test]
    fn test_one_line_list() {
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> = vec![
            String::from("no list"),
            String::from("1. list here"),
            String::from("end list"), //this should not be converted into a list
        ];
        let expecte_html_lines: Vec<String> = vec![
            String::from("no list\n"),
            String::from("<ol><li>list here</li>\n"),
            String::from("</ol>end list\n"),
        ];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expecte_html_lines[0]);
        assert_eq!(actual_html_lines[1], expecte_html_lines[1]);
        assert_eq!(actual_html_lines[2], expecte_html_lines[2]);
    }
    #[test]
    fn test_two_line_list() {
        //string with space before pound sign should not be converted
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> = vec![
            String::from("no list"),
            String::from("1. first item"),
            String::from("1. second item"),
            String::from("end list"),
        ];
        let expected_html_lines: Vec<String> = vec![
            String::from("no list\n"),
            String::from("<ol><li>first item</li>\n"),
            String::from("<li>second item</li>\n"),
            String::from("</ol>end list\n"),
        ];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expected_html_lines[0]);
        assert_eq!(actual_html_lines[1], expected_html_lines[1]);
        assert_eq!(actual_html_lines[2], expected_html_lines[2]);
        assert_eq!(actual_html_lines[3], expected_html_lines[3]);
    }
}
#[cfg(test)]
mod header_test {
    use super::parse_all_lines;
    #[test]
    fn test_header_1() {
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> = vec![String::from("# Header 1"), String::from("content")];
        let expecte_html_lines: Vec<String> = vec![
            String::from("<h1>Header 1</h1>\n"),
            String::from("content\n"),
        ];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expecte_html_lines[0]);
        assert_eq!(actual_html_lines[1], expecte_html_lines[1]);
    }
    #[test]
    fn test_header_2() {
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> =
            vec![String::from("## Header 2"), String::from("content")];
        let expecte_html_lines: Vec<String> = vec![
            String::from("<h2>Header 2</h2>\n"),
            String::from("content\n"),
        ];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expecte_html_lines[0]);
        assert_eq!(actual_html_lines[1], expecte_html_lines[1]);
    }
    #[test]
    fn test_header_3() {
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> =
            vec![String::from("### Header 3"), String::from("content")];
        let expecte_html_lines: Vec<String> = vec![
            String::from("<h3>Header 3</h3>\n"),
            String::from("content\n"),
        ];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expecte_html_lines[0]);
        assert_eq!(actual_html_lines[1], expecte_html_lines[1]);
    }
    #[test]
    fn invalid_test_header_1() {
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> = vec![String::from("#Not Header 1")];
        let expecte_html_lines: Vec<String> = vec![String::from("#Not Header 1\n")];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expecte_html_lines[0]);
    }
    #[test]
    fn test_header_2_invalid() {
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> = vec![String::from("##Not Header 2")];
        let expecte_html_lines: Vec<String> = vec![String::from("##Not Header 2\n")];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        assert_eq!(actual_html_lines[0], expecte_html_lines[0]);
    }
    #[test]
    fn test_header_3_invalid() {
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> = vec![String::from("###Not Header 3")];
        let expecte_html_lines: Vec<String> = vec![String::from("###Not Header 3\n")];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expecte_html_lines[0]);
    }
}
#[cfg(test)]
mod plaintext_test {
    use super::parse_all_lines;
    #[test]
    fn test_plaintext() {
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> = vec![String::from("Plain text")];
        let expecte_html_lines: Vec<String> = vec![String::from("Plain text\n")];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expecte_html_lines[0]);
    }
}

#[cfg(test)]
mod blockquote_test {
    use super::parse_all_lines;
    #[test]
    fn test_blockquote() {
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> = vec![String::from("> Quoted text")];
        let expecte_html_lines: Vec<String> =
            vec![String::from("<blockquote>Quoted text</blockquote>\n")];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expecte_html_lines[0]);
    }
    #[test]
    fn test_blockquote_invalid() {
        let mut actual_html_lines: Vec<String> = Vec::new();
        let markdown_lines: Vec<String> = vec![String::from(">Not quoted text")];
        let expecte_html_lines: Vec<String> = vec![String::from(">Not quoted text\n")];
        parse_all_lines(markdown_lines, &mut actual_html_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_html_lines[0], expecte_html_lines[0]);
    }
}

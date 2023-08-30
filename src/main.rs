mod parse_line_formatting;

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

fn main() {
    let input_file_name = "./benchmarks/benchmark1/input.md";
    let output_file_name = "./benchmarks/benchmark1/output.html";
    // let input_file_name = "./benchmars/benchmark1/input.md";
    // let output_file_name = "./benchmarks/benchmark1/output.html";
    let input_lines = file_io::get_file_lines(input_file_name); //get the lines from the file
    let output_lines = parse_line_formatting::parse_all_lines(input_lines); //process the lines

    // for line in output_lines.clone() {
    //     println!("{}", line); //display the lines
    // }
    file_io::write_line_to_file_true(&output_lines, output_file_name);

    // let _italics_result: String =
    //     parse_text_formatting::process_italics(String::from("new *struing*"));
    // let _header_result: String =
    //     parse_line_formatting::process_headers(String::from("# new string"));
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

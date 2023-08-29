pub mod file_io {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
        where P: AsRef<Path>, {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    pub fn get_file_lines() -> Vec<String>{
        let filename = "./hosts.txt";
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
}

fn process_headers(str: String) -> String{
    //if first characters are 'h1', then add the h1 tags
    if &str[0..2] == "# "{
        let remaining_str = &str[2..];
        format!("<h1>{}</h1>", remaining_str)
    } 
    else if &str[0..3] == "## "{
        let remaining_str = &str[3..];
        format!("<h2>{}</h2>", remaining_str)
    }
    else if &str[0..4] == "### "{
        let remaining_str = &str[4..];
        format!("<h3>{}</h3>", remaining_str)
    }
    else{
         str
    }
}

fn process_italics(str: String) -> String{
    let mut result = String::new();
    let mut stack: Vec<&str> = Vec::new();
    let mut buffer: String = String::new();

    for c in str.chars() {
        println!("while");
        /*
         cases:
         - not in italics, adding a char
         - switching into italics
         - in italics, adding a char
         - switching out of italics
         */
       
        if c == '*'{ //switching in or out of italics
            //if top of stack is *, then we are switching out
            if stack.last() == Some(&"*") {
                //pop the asterisk, update the buffer correctly
                stack.pop();
                //update the buffer with italicized text
                buffer = format!("<i>{buffer}</i>");
            }
            else{//else, we are switching into italics
                stack.push("*");//update the stack
            }
            result.push_str(buffer.as_str());//push the current contents of the buffer
            buffer = String::new();//reset the buffer to being empty
        }
        else if stack.last() == Some(&"*") && buffer.is_empty()&& c == ' '{
            //if top of stack is '*' and buffer is empty and current char is space, then we need to escape italics
            stack.pop();
            buffer.push('*');
            buffer.push(' ');
        }  
        else{
            buffer.push(c);
        }
        println!("current solution state 3: {}", result);
    }
    result.push_str(&buffer);
    println!("current solution state 4: {}", result);
    if !stack.is_empty() {
        //push remaining characters onto the stack
        for substring in stack{
            result.push_str(substring);
        }
    }
    println!("current solution state 5: {}", result);
    result
}

fn process_unordered_lists(str: String) -> (String, LineState){
    if &str[0..2] == "- "{
        let remaining_str = &str[2..];
        (format!("<li>{}</li>", remaining_str), LineState::UnorderedList)
    } else{
        (str, LineState::Other)
    }
}
#[derive(PartialEq)]
enum LineState{
    UnorderedList,
    Other
}

/**
 * Parses lines from the file, converting them into bulleted lists where needed
 */
fn parse_all_lines(lines: Vec<String>) -> Vec<String>{
    let mut proxy_file: Vec<String> = Vec::new();
    //iterate through all of the lines
    let mut current_line_state :LineState = LineState::Other;
    
    //process the current line, determine its state
    for line in lines {
        //parse the current line, determine the state
        let (mut parsed_line, new_state) = process_unordered_lists(line);
        //let new_state = LineState::UnorderedList;
        if current_line_state != LineState::UnorderedList && new_state == LineState::UnorderedList {

            //we just started a bulleted list, so we need to insert a <ul> tag
            parsed_line = format!("<ul>{}", parsed_line);

        } else if current_line_state == LineState::UnorderedList && new_state != LineState::UnorderedList{
            //we just exited a bulleted list, so we need to insert a </ul> tag
            parsed_line = format!("</ul>{}", parsed_line);
        }
        write_line_to_file(parsed_line, &mut proxy_file);
        current_line_state = new_state;
    }
    proxy_file
}

fn write_line_to_file(str: String,  file: &mut Vec<String>) {//write a line to a file
    //TODO: should write to file
    file.push(str);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.

fn main() {
    let inputs = file_io::get_file_lines();
    parse_all_lines(inputs);
    let italics_result: String = process_italics(String::from("new **string**"));
    let header_result: String = process_headers(String::from("# new string"));

   // File hosts.txt must exist in the current path
   println!("{}", italics_result);
   println!("{}", header_result);
}

#[cfg(test)]
mod header_tests {
    use super::*;
    #[test]
    fn convert_h1_header(){
        //valid string should receive tags
        let input_str= String::from("# Here is a header");
        let expected_result =  String::from("<h1>Here is a header</h1>");
        let actual_result = process_headers(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_improper_h1_header(){
        //string with space before pound sign should not be converted
        let input_str= String::from(" # Here is a header");
        let actual_result: String = process_headers(input_str.clone());
        assert_eq!(actual_result, input_str);

        let input_str_2= String::from("#Here is a header");
        let actual_result_2: String = process_headers(input_str_2.clone());
        assert_eq!(actual_result_2, input_str_2);
    }
    #[test]
    fn convert_h2_header(){
        //valid string should receive tags
        let input_str= String::from("## Here is a header");
        let expected_result =  String::from("<h2>Here is a header</h2>");
        let actual_result = process_headers(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_improper_h2_header(){
        //string with space before pound sign should not be converted
        let input_str= String::from(" ## Here is a header");
        let actual_result: String = process_headers(input_str.clone());
        assert_eq!(actual_result, input_str);

        let input_str_2= String::from("##Here is a header");
        let actual_result_2: String = process_headers(input_str_2.clone());
        assert_eq!(actual_result_2, input_str_2);
    }
}


#[cfg(test)]
mod italics_bold_tests {
    use super::*;
    #[test]
    fn convert_italics(){
        //string with space before pound sign should not be converted
        let input_str= String::from("some *text*");
        let expected_result =  String::from("some <i>text</i>");
        let actual_result = process_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid(){
        //string with space before pound sign should not be converted
        let input_str= String::from("some * text *");
        let expected_result =  String::from("some * text *");
        let actual_result: String = process_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
}

#[cfg(test)]
mod unordered_list_test{
    use super::*;
    #[test]
    fn test_list(){
        //string with space before pound sign should not be converted
        let input_str= String::from("- list here");
        let expected_result =  String::from("<li>list here</li>");
        let (actual_result, _) = process_unordered_lists(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn test_one_line_list(){
        //string with space before pound sign should not be converted
        let file_lines: Vec<String> = vec![String::from("no list"),String::from("- list here"), String::from("end list")];
        let expected_result: Vec<String> =  vec![String::from("no list"),String::from("<ul><li>list here</li>"), String::from("</ul>end list")];
        let actual_result = parse_all_lines(file_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_result[0], expected_result[0]);
        assert_eq!(actual_result[1], expected_result[1]);
        assert_eq!(actual_result[2], expected_result[2]);

    }
    #[test]
    fn test_two_line_list(){
        //string with space before pound sign should not be converted
        let file_lines: Vec<String> = vec![String::from("no list"),String::from("- list here"),String::from("- another here"), String::from("end list")];
        let expected_result: Vec<String> =  vec![String::from("no list"),String::from("<ul><li>list here</li>"),String::from("<li>another here</li>"), String::from("</ul>end list")];
        let actual_result = parse_all_lines(file_lines);
        //assert_eq!(actual_result.len(), expected_result.len());
        assert_eq!(actual_result[0], expected_result[0]);
        assert_eq!(actual_result[1], expected_result[1]);
        assert_eq!(actual_result[2], expected_result[2]);
        assert_eq!(actual_result[3], expected_result[3]);

    }
    #[test]
    fn test_list_invalid(){
        //string with space before pound sign should not be converted
        let input_str= String::from("-list here");
        let expected_result =  String::from("-list here");
        let (actual_result, _) = process_unordered_lists(input_str);
        assert_eq!(actual_result, expected_result);
    }

}
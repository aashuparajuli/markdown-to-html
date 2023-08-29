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
    let mut stack: Vec<char> = Vec::new();
    let mut buffer: String = String::new();
    for c in str.chars(){
        /*
         cases:
         - not in italics, adding a char
         - switching into italics
         - in italics, adding a char
         - switching out of italics
         */
        if c == '*'{ //switching in or out of italics
            //if top of stack is *, then we are switching out
            if stack.last() == Some(&'*') {
                //pop the asterisk, update the buffer correctly
                stack.pop();
                //update the buffer with italicized text
                buffer = format!("<i>{buffer}</i>");
            }
            else{//else, we are switching into italics
                stack.push('*');//update the stack
            }
            result.push_str(buffer.as_str());//push the current contents of the buffer
            buffer = String::new();//reset the buffer to being empty
        }
        //if top of stack is '*' and buffer is empty and current char is space, then we need to escape italics
        else if stack.last() == Some(&'*') && buffer.is_empty()&& c == ' '{
            stack.pop();
            buffer.push('*');
            buffer.push(' ');
        }  else if stack.last() == Some(&'*') && buffer.is_empty()&& c == '*'{
            //if top of stack is '*' and buffer is empty and current char is *, then we need to escape italics to create bold
            todo!("switch to bold if there are two consecutive occurences of '*' ");
            //stack.push('**');
        }  
        else{
            buffer.push(c);
        }
        
    }
    if !stack.is_empty() {
        //push remaining characters onto the stack
        for c in stack{
            result.push(c);
        }
    }
    result
}

fn main() {

   let italics_result: String = process_italics(String::from("new *string*"));
   let header_result: String = process_headers(String::from("# new string"));
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
mod italics_tests {
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
        let actual_result = process_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
}
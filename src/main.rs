
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
    let mut str_iter = str.chars();
    
    while let Some(c) = str_iter.next() {
        println!("while");
        /*
         cases:
         - not in italics, adding a char
         - switching into italics
         - in italics, adding a char
         - switching out of italics
         */
        if stack.last() == Some(&"*") && buffer.is_empty()&& c == '*'{
            //if top of stack is '*' and buffer is empty and current char is *, then we need to escape italics to create bold
            stack.pop();
            stack.push("<b>");
            println!("starting a bold tag")
            //todo!("switch to bold if there are two consecutive occurences of '*' ");
        } 
        else if c == '*'{ //switching in or out of italics
            //if the top of the stack is <b>, then we try to find a bold closing tag
            if stack.last() == Some(&"<b>") {
                println!("closing a bold tag");
                //if there is a next character, check if it is a *
                //if yes -> skip it, create a bold tag
                //if no -> return to italics tag, add it to the queue
                if let Some(next_character) = str_iter.next() { //if this works, then we can create a bold tag
                    if next_character == '*' {//next char is *, so we can add a bold tag
                        stack.pop();//pop the <b> tag
                        
                        buffer = format!("<b>{buffer}</b>");
                        println!("closing a bold tag {}", stack.len());
                        println!("current solution state 1: {}", result);
                    }
                    else{
                        /* else
                         - pop the bold tag
                         - put the italics tag back on the stack, 
                         - add italics string to buffer
                        */
                        println!("closing a bold tag with italics");
                        stack.pop();
                        stack.push("*");
                        buffer = format!("<i>{buffer}</i>{next_character}");
                    }
                } else{
                    println!("closing a bold tag with italics");
                    stack.pop();
                    buffer = format!("*<i>{buffer}</i>");
                }
            }
            //if top of stack is *, then we are switching out
            else if stack.last() == Some(&"*") {
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
            if substring == "<b>"{
               // result.push('*');
               result.push_str("<b>");
            } else{
                result.push_str(substring);
            }
        }
    }
    println!("current solution state 5: {}", result);
    result
}

fn main() {

   let italics_result: String = process_italics(String::from("new **string**"));
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

    #[test]
    fn convert_bold(){
        //string with space before pound sign should not be converted
        let input_str= String::from("some **text** a");
        let expected_result =  String::from("some <b>text</b> a");
        let actual_result = process_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_mixed(){
        //string with space before pound sign should not be converted
        let input_str= String::from("some **text*");
        let expected_result =  String::from("some *<i>text</i>");
        let actual_result = process_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
}
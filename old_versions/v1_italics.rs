//originally in parse_text_formatting
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

fn add_h1_header(str: String) -> String {
    format!("<h1>{}</h1>", str)
}

fn process_h1(str: String) -> String{
    //if first characters are 'h1', then add the h1 tags
    if &str[0..2] == "# "{
        let remaining_str = &str[2..];
        format!("<h1>{}</h1>", remaining_str)
    } else{
         str
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_h1_header(){
        let input_str= String::from("Here is a header");
        let expected_result =  String::from("<h1>Here is a header</h1>");
        let actual_result = add_h1_header(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_h1_header(){
        //valid string should receive tags
        let input_str= String::from("# Here is a header");
        let expected_result =  String::from("<h1>Here is a header</h1>");
        let actual_result = process_h1(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_improper_h1_header(){
        //string with space before pound sign should not be converted
        let input_str= String::from(" # Here is a header");
        let actual_result: String = process_h1(input_str.clone());
        assert_eq!(actual_result, input_str);
    }

}
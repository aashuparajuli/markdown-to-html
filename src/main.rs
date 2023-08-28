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

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
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
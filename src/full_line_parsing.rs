#[derive(PartialEq)]
pub enum LineType {
    UnorderedList,
    OrderedList,
    Header1,
    Header2,
    Header3,
    Blockquote,
    Other,
}

pub fn determine_line_type(line: String) -> (String, LineType) {
    if line.len() < 2 {
        (line, LineType::Other)
    } else if &line[0..2] == "# " {
        let remaining_str = &line[2..];
        (remaining_str.to_string(), LineType::Header1)
    } else if &line[0..2] == "> " {
        let remaining_str = &line[2..];
        (remaining_str.to_string(), LineType::Blockquote)
    } else if &line[0..2] == "- " {
        let remaining_str = &line[2..];
        (remaining_str.to_string(), LineType::UnorderedList)
    } else if line.len() < 3 {
        (line, LineType::Other)
    } else if &line[0..3] == "## " {
        let remaining_str = &line[3..];
        (remaining_str.to_string(), LineType::Header2)
    } else if &line[0..3] == "1. " {
        let remaining_str = &line[3..];
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
pub fn insert_list_start_or_end<'a> (current_line_state: &'a LineType, new_state: &'a LineType) -> &'a str {
    match (current_line_state, new_state) {
        (LineType::UnorderedList, LineType::UnorderedList) => "",
        (LineType::OrderedList, LineType::OrderedList) => "",
        (_, LineType::UnorderedList) => "<ul>",
        (_, LineType::OrderedList) => "<ol>",
        (LineType::UnorderedList, _) => "</ul>",
        (LineType::OrderedList, _) => "</ol>",
        (_, _) => "",
    }
}

#[cfg(test)]
mod test_determine_line_type{
    use super::determine_line_type;
    use super::LineType;
    #[test]
    fn too_short() {
        let input_str = String::from("##");
        let output = determine_line_type(input_str);
        assert_eq!(output.0, String::from("##"));
        matches!(output.1, LineType::Other);
    }
    #[test]
    fn unordered_list() {
        let input_str = String::from("- K");
        let output = determine_line_type(input_str);
        assert_eq!(output.0, String::from("K"));
        matches!(output.1, LineType::UnorderedList);
    }
    #[test]
    fn header_1() {
        let input_str = String::from("# H");
        let output = determine_line_type(input_str);
        assert_eq!(output.0, String::from("H"));
        matches!(output.1, LineType::Header1);
    }
    #[test]
    fn header_2() {
        let input_str = String::from("## H");
        let output = determine_line_type(input_str);
        assert_eq!(output.0, String::from("H"));
        matches!(output.1, LineType::Header2);
    }
    #[test]
    fn header_3() {
        let input_str = String::from("### H");
        let output = determine_line_type(input_str);
        assert_eq!(output.0, String::from("H"));
        matches!(output.1, LineType::Header3);
    }
    #[test]
    fn blockquote() {
        let input_str = String::from("> H");
        let output = determine_line_type(input_str);
        assert_eq!(output.0, String::from("H"));
        matches!(output.1, LineType::Blockquote);
    }
    #[test]
    fn ordered_list() {
        let input_str = String::from("1. H");
        let output = determine_line_type(input_str);
        assert_eq!(output.0, String::from("H"));
        matches!(output.1, LineType::OrderedList);
    }
    #[test]
    fn other() {
        let input_str = String::from("! H");
        let output = determine_line_type(input_str);
        assert_eq!(output.0, String::from("! H"));
        matches!(output.1, LineType::Other);
    }
   
}

#[cfg(test)]
mod test_start_end_list{
    use super::insert_list_start_or_end;
    use super::LineType;
    #[test]
    fn starting_unordered_list() {
        let first_line =  LineType::Header1; 
        let second_line =LineType::UnorderedList;
        let output = insert_list_start_or_end(&first_line, &second_line);
        assert_eq!(output, String::from("<ul>"));
    }
    #[test]
    fn in_unordered_list() {
        let first_line =  LineType::UnorderedList; 
        let second_line =LineType::UnorderedList;
        let output = insert_list_start_or_end(&first_line, &second_line);
        assert_eq!(output, "");
    }
    #[test]
    fn ending_unordered_list() {
        let first_line = LineType::UnorderedList;
        let second_line = LineType::Header1;
        let output = insert_list_start_or_end(&first_line, &second_line);
        assert_eq!(output, "</ul>");
    }
   

    #[test]
    fn starting_ordered_list() {
        let first_line =  LineType::Header1; 
        let second_line =LineType::OrderedList;
        let output = insert_list_start_or_end(&first_line, &second_line);
        assert_eq!(output,"<ol>");
    }
    #[test]
    fn in_ordered_list() {
        let first_line =  LineType::OrderedList; 
        let second_line =LineType::OrderedList;
        let output = insert_list_start_or_end(&first_line, &second_line);
        assert_eq!(output, "");
    }
  
    #[test]
    fn ending_ordered_list() {
        let first_line = LineType::OrderedList;
        let second_line = LineType::Header1;
        let output = insert_list_start_or_end(&first_line, &second_line);
        assert_eq!(output, "</ol>");
    }
}
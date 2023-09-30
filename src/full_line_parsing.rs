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

impl LineType {
    pub fn format_line(&self, prefix: &str, parsed_line: &str) -> String {
        match self {
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
            LineType::Blockquote => {
                format!("{}<blockquote>{}</blockquote>\n", prefix, parsed_line)
            }
        }
    }
}

pub fn determine_line_type(line: &str) -> (&str, LineType) {
    if line.len() < 2 {
        (line, LineType::Other)
    } else if line.starts_with("# ") {
        let remaining_str = &line[2..];
        (remaining_str, LineType::Header1)
    } else if line.starts_with( "> " ){
        let remaining_str = &line[2..];
        (remaining_str, LineType::Blockquote)
    } else if line.starts_with( "- ") {
        let remaining_str = &line[2..];
        (remaining_str, LineType::UnorderedList)
    }  else if line.starts_with( "## ") {
        let remaining_str = &line[3..];
        (remaining_str, LineType::Header2)
    } else if line.starts_with( "1. ") {
        let remaining_str = &line[3..];
        (remaining_str, LineType::OrderedList)
    } else if line.starts_with( "### ") {
        let remaining_str = &line[4..];
        (remaining_str, LineType::Header3)
    } else {
        (line, LineType::Other)
    }
}
pub fn insert_list_start_or_end<'a>(
    current_line_state: &'a LineType,
    new_state: &'a LineType,
) -> &'a str {
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
mod test_determine_line_type {
    use super::determine_line_type;
    use super::LineType;
    #[test]
    fn too_short() {
        let input_str = "##";
        let output = determine_line_type(input_str);
        assert_eq!(output.0, "##");
        matches!(output.1, LineType::Other);
    }
    #[test]
    fn unordered_list() {
        let input_str = "- K";
        let output = determine_line_type(input_str);
        assert_eq!(output.0, "K");
        matches!(output.1, LineType::UnorderedList);
    }
    #[test]
    fn header_1() {
        let input_str = "# H";
        let output = determine_line_type(input_str);
        assert_eq!(output.0, "H");
        matches!(output.1, LineType::Header1);
    }
    #[test]
    fn header_2() {
        let input_str = "## H";
        let output = determine_line_type(input_str);
        assert_eq!(output.0, "H");
        matches!(output.1, LineType::Header2);
    }
    #[test]
    fn header_3() {
        let input_str = "### H";
        let output = determine_line_type(input_str);
        assert_eq!(output.0, "H");
        matches!(output.1, LineType::Header3);
    }
    #[test]
    fn blockquote() {
        let input_str = "> H";
        let output = determine_line_type(input_str);
        assert_eq!(output.0, "H");
        matches!(output.1, LineType::Blockquote);
    }
    #[test]
    fn ordered_list() {
        let input_str = "1. H";
        let output = determine_line_type(input_str);
        assert_eq!(output.0, "H");
        matches!(output.1, LineType::OrderedList);
    }
    #[test]
    fn other() {
        let input_str = "! H";
        let output = determine_line_type(input_str);
        assert_eq!(output.0, "! H");
        matches!(output.1, LineType::Other);
    }
}

#[cfg(test)]
mod test_start_end_list {
    use super::insert_list_start_or_end;
    use super::LineType;
    #[test]
    fn starting_unordered_list() {
        let first_line = LineType::Header1;
        let second_line = LineType::UnorderedList;
        let output = insert_list_start_or_end(&first_line, &second_line);
        assert_eq!(output, String::from("<ul>"));
    }
    #[test]
    fn in_unordered_list() {
        let first_line = LineType::UnorderedList;
        let second_line = LineType::UnorderedList;
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
        let first_line = LineType::Header1;
        let second_line = LineType::OrderedList;
        let output = insert_list_start_or_end(&first_line, &second_line);
        assert_eq!(output, "<ol>");
    }
    #[test]
    fn in_ordered_list() {
        let first_line = LineType::OrderedList;
        let second_line = LineType::OrderedList;
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

#[cfg(test)]
mod test_format_line {
    use super::LineType;

    const prefix: &str = "";
    const parsed_line: &str = "Hello World";
    #[test]
    fn unordered_list() {
        let sample_line: LineType = LineType::UnorderedList;
        let output: String = sample_line.format_line(prefix, parsed_line);
        assert_eq!(
            output,
            format!("{prefix}{0}{parsed_line}{1}\n", "<li>", "</li>")
        );
    }
    #[test]
    fn header_1() {
        let sample_line: LineType = LineType::Header1;
        let output: String = sample_line.format_line(prefix, parsed_line);
        assert_eq!(
            output,
            format!("{prefix}{0}{parsed_line}{1}\n", "<h1>", "</h1>")
        );
    }
    #[test]
    fn header_2() {
        let sample_line: LineType = LineType::Header2;
        let output: String = sample_line.format_line(prefix, parsed_line);
        assert_eq!(
            output,
            format!("{prefix}{0}{parsed_line}{1}\n", "<h2>", "</h2>")
        );
    }
    #[test]
    fn header_3() {
        let sample_line: LineType = LineType::Header3;
        let output: String = sample_line.format_line(prefix, parsed_line);
        assert_eq!(
            output,
            format!("{prefix}{0}{parsed_line}{1}\n", "<h3>", "</h3>")
        );
    }
    #[test]
    fn blockquote() {
        let sample_line: LineType = LineType::Blockquote;
        let output: String = sample_line.format_line(prefix, parsed_line);
        assert_eq!(
            output,
            format!(
                "{prefix}{0}{parsed_line}{1}\n",
                "<blockquote>", "</blockquote>"
            )
        );
    }
    #[test]
    fn ordered_list() {
        let sample_line: LineType = LineType::OrderedList;
        let output: String = sample_line.format_line(prefix, parsed_line);
        assert_eq!(
            output,
            format!("{prefix}{0}{parsed_line}{1}\n", "<li>", "</li>")
        );
    }
    #[test]
    fn other() {
        let sample_line: LineType = LineType::Other;
        let output: String = sample_line.format_line(prefix, parsed_line);
        assert_eq!(output, format!("{prefix}{0}{parsed_line}{1}\n", "", ""));
    }
}

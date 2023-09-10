#[derive(Clone, Debug)]
enum TextType {
    Italics,
    Space,
    Plaintext,
}

pub struct FormattedText {
    format: TextType,
    start_idx: usize,
    end_idx: usize,
}
impl FormattedText {
    fn new(format: TextType, start_idx: usize, end_idx: usize) -> FormattedText {
        FormattedText {
            format,
            start_idx,
            end_idx,
        }
    }
}
pub fn italics_tokenizer(str: String) -> Vec<FormattedText> {
    let mut tokens: Vec<FormattedText> = Vec::new();
    let mut start_idx: usize = 0;

    for (curr_idx, c) in str.chars().enumerate() {
        match c {
            '*' => {
                //push the currently parsed tokens
                if curr_idx != start_idx {
                    tokens.push(FormattedText::new(TextType::Plaintext, start_idx, curr_idx));
                    println!("created 2 token in *:{curr_idx},{c}");
                } else {
                    println!("created 1 tokens in *:{curr_idx},{c}");
                }
                //push the italics token
                tokens.push(FormattedText::new(
                    TextType::Italics,
                    curr_idx,
                    curr_idx + 1,
                ));

                start_idx = curr_idx + 1;
            }
            ' ' => {
                //push the currently parsed plaintext, if there is any
                if curr_idx != start_idx {
                    tokens.push(FormattedText::new(TextType::Plaintext, start_idx, curr_idx));
                    println!("created 2 token in space:{curr_idx},{c}");
                } else {
                    println!("created 1 tokens in space:{curr_idx},{c}");
                }
                //push the space token
                tokens.push(FormattedText::new(TextType::Space, curr_idx, curr_idx + 1));
                start_idx = curr_idx + 1;
            }
            _ => {
                //expanding plaintext
                //continue growing the current token
            }
        };
    }
    //push the tokens that have not been pushed as plain text if there are any
    if start_idx != str.len() {
        tokens.push(FormattedText::new(
            TextType::Plaintext,
            start_idx,
            str.len(),
        ));
    }
    tokens
}

#[cfg(test)]
mod test_tokensizer {
    use crate::italics_lexer::TextType;

    use super::italics_tokenizer;
    use super::FormattedText;
    #[test]
    fn valid_tokens() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *text*");
        let actual_result = italics_tokenizer(input_str);
        assert_eq!(5, actual_result.len());
        matches!(actual_result[0].format, TextType::Plaintext);
        matches!(actual_result[1].format, TextType::Space);
        matches!(actual_result[2].format, TextType::Italics);
        matches!(actual_result[3].format, TextType::Plaintext);
        matches!(actual_result[4].format, TextType::Italics);
    }
    #[test]
    fn plain_text() {
        //string with space before pound sign should not be converted
        let input_str = String::from("plain text");
        let actual_result = italics_tokenizer(input_str);
        assert_eq!(3, actual_result.len());
    }
    #[test]
    fn invalid_one_space() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *text *");
        let actual_result = italics_tokenizer(input_str);
        assert_eq!(6, actual_result.len());
        assert!(matches!(actual_result[0].format, TextType::Plaintext));
        assert!(matches!(actual_result[1].format, TextType::Space));
        assert!(matches!(actual_result[2].format, TextType::Italics));
        assert!(matches!(actual_result[3].format, TextType::Plaintext));
        assert!(matches!(actual_result[4].format, TextType::Space));
        assert!(matches!(actual_result[5].format, TextType::Italics));
    }
    #[test]
    fn invalid_two_spaces() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some * text *");
        let actual_result = italics_tokenizer(input_str);
        assert_eq!(7, actual_result.len());
        assert!(matches!(actual_result[0].format, TextType::Plaintext));
        assert!(matches!(actual_result[1].format, TextType::Space));
        assert!(matches!(actual_result[2].format, TextType::Italics));
        assert!(matches!(actual_result[3].format, TextType::Space));
        assert!(matches!(actual_result[4].format, TextType::Plaintext));
        assert!(matches!(actual_result[5].format, TextType::Space));
        assert!(matches!(actual_result[6].format, TextType::Italics));
    }
    #[test]
    fn invalid_two_asterisk() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some **text");
        let actual_result = italics_tokenizer(input_str);
        assert_eq!(5, actual_result.len());
        assert!(matches!(actual_result[0].format, TextType::Plaintext));
        assert!(matches!(actual_result[1].format, TextType::Space));
        assert!(matches!(actual_result[2].format, TextType::Italics));
        assert!(matches!(actual_result[3].format, TextType::Italics));
        assert!(matches!(actual_result[4].format, TextType::Plaintext));
    }
}

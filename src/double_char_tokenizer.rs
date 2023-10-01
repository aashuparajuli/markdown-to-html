#[derive(Clone, Copy, Debug)]
pub enum Token {
    // Plaintext(usize, usize),
    Plaintext(usize, usize),
    Asterisk,
    Space,
    DoubleAsterisk, //each character, except double asterisk gets it own character
}
#[derive(Clone, Copy)]
enum CharType {
    Asterisk,
    Plaintext,
    Space,
}
impl CharType {
    pub fn new(c: char) -> CharType {
        match c {
            '*' => CharType::Asterisk,
            ' ' => CharType::Space,
            _ => CharType::Plaintext,
        }
    }
}
pub fn double_char_tokenizer(str: &str) -> Vec<Token> {
    if str.is_empty() {
        return Vec::new();
    }
    use Token;
    //make it generic over any type that implements
    let mut token_stream: Vec<Token> = Vec::new();
    let mut curr_token: Option<Token> = None;
    let mut curr_section: Option<CharType> = None;
    let mut start_idx: usize = usize::max_value();
    let mut reading_plaintext: bool = false;
    for (i, c) in str.char_indices() {
        let next_char: CharType = CharType::new(c);

        //preconditions, dealing with altering previous elements
        //if prev char is not plaintext && curr char is plaintext => set start_idx
        //if prev char is plaintext && curr char is not plaintext => use start_idx and i to get substring
        //if prev char is plaintext && curr char is plaintext, do nothing, let i continue growing
        //do nothing

        //otherwise, just push to stack
        match (curr_section, next_char) {
            (Some(CharType::Plaintext), CharType::Plaintext) => (),
            (_, CharType::Plaintext) => {
                start_idx = i;
                reading_plaintext = true;
            }
            (Some(CharType::Plaintext), _) => {
                //append plaintext
                token_stream.push(Token::Plaintext(start_idx, i));
                reading_plaintext = false;
            }
            (_, _) => (),
        };

        match next_char {
            CharType::Asterisk if matches!(token_stream.last(), Some(Token::Asterisk)) => {
                //trigger double asterisk
                token_stream.pop();
                token_stream.push(Token::DoubleAsterisk);
            }
            CharType::Asterisk => {
                //push asterisk normally
                token_stream.push(Token::Asterisk);
            }
            CharType::Space => {
                //push space normally
                token_stream.push(Token::Space);
            }
            CharType::Plaintext => {
                //do nothing, plaintext taken care of by previous match
            }
        };
        curr_section = Some(next_char);
        //also perform parsing here
    }

    //if plaintext is still open, close it, then add
    if reading_plaintext {
        token_stream.push(Token::Plaintext(start_idx, str.len()));
        //token_stream.push(Token::Plaintext(&str[start_idx..]));
    }

    token_stream
}
pub struct FormattedText<'a> {
    pub formatted: bool,
    pub substring: &'a str,
}
impl FormattedText<'_> {
    pub fn new(formatted: bool, substring: &str) -> FormattedText {
        FormattedText {
            formatted,
            substring,
        }
    }
    fn get_html(&self) -> String {
        match self.formatted {
            true => format!("<b>{}</b>", self.substring),
            false => self.substring.to_string(),
        }
    }
}
enum FormatSection {
    Text(String),
    Bold,
}
pub fn token_parser(tokens: &Vec<Token>, str: &str) -> String {
    let mut subsections: Vec<FormattedText> = Vec::new();
    let mut stack: Vec<Token> = Vec::new();
    let mut result: String = String::new();
    let mut parsing_plain_text: bool = false;
    //let mut curr_format_section: FormatSection
    //to extend format_section: 
    let mut start_idx: usize = 0;
    let mut end_idx: usize = 0;

    for next_token in tokens {
        //stack will store FormatSection will be stored in 
        //push items into the FormatSection as we parse
        //when the stack has the correct values, pop values, format them, push them back on 
        
        let mut section_stack:  Vec<FormatSection>  = Vec::new();
        match next_token {
            Token::Plaintext(_, _) => {
            // Token::Plaintext(x) => {
            //     section_stack.push(FormatSection::Text(String::from(x)));//isntead extract text from Plaintext
                section_stack.push(FormatSection::Text(String::new()));//isntead extract text from Plaintext
            },
            Token::Asterisk => {todo!()},
            Token::Space => todo!(),
            Token::DoubleAsterisk => {
                section_stack.push(FormatSection::Bold);//isntead extract text from Plaintext
            },
        }
        /*if curr_format_section is Plaintext && next_token is (Plaintext, Asterisk, Space)
                 continue expanding plaintext

        if curr_format_section is Bold && next_token is DoubleAsterisk
                 append '****' as plaintext, we are escaping the bold character
                 append to either current FormatSection::Text or create a new one
        if curr_format_section is Bold && next_token is (Plaintext, Asterisk, Space)

        */
        match (parsing_plain_text, next_token) {
            (true, Token::Plaintext(_, b)) => {
                //continue plaintext
                end_idx = *b;
            }
            (true, Token::Asterisk) => {
                //append asterisk as plaintext
                end_idx += 1;
            }
            (true, Token::Space) => {
                //append space as plaintext
                end_idx += 1;
            }
            (true, Token::DoubleAsterisk) => {
                let mut is_formatted_italics = false;
                if matches!(stack.last(), Some(Token::DoubleAsterisk)) {
                    is_formatted_italics = true;
                    stack.pop();
                }
                //end plaintext
                subsections.push(FormattedText::new(
                    is_formatted_italics,
                    &str[start_idx..end_idx],
                ))
            }
            (false, Token::Plaintext(_, _)) => {
                parsing_plain_text = true;
                todo!()},
            (false, Token::Asterisk) => todo!(),
            (false, Token::Space) => todo!(),
            (false, Token::DoubleAsterisk) => {}
        }
    }
    subsections
        .iter()
        .for_each(|subsection| result.push_str(&subsection.get_html()));
    result

    //initial scenario:
    //if prev_token, None: append curr_token
    //else, append curr_token

    //better version:
    // if prev_token is None, append curr. set prev_token to curr
    //if (prev,curr) is (Space, asterisk), append curr. set prev_token to token::asterisk
    //if (prev,curr) is (Plaintext, asterisk), append curr. set prev_token to asterisk.
    //if (prev,curr) is (Plaintext, asterisk), append curr. set prev_token to asterisk.
    //if (prev,curr) is (Plaintext, asterisk), append curr. set prev_token to asterisk.
    //if (prev,curr) is (Asterisk, asterisk), append curr. set prev_token to asterisk.
    //if (prev,curr) is (Asterisk, asterisk), append curr. set prev_token to asterisk.
}
#[cfg(test)]
mod test_tokenizer {
    use super::double_char_tokenizer;
    use super::Token;
    #[test]
    fn basic() {
        //string with space before pound sign should not be converted
        let input_str = "some";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 1);
        // assert!(matches!(actual_result[0], Token::Plaintext("some")));
        assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
    }
    #[test]
    fn single_asterisk() {
        //string with space before pound sign should not be converted
        let input_str = "som*";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 2);
        assert!(matches!(actual_result[0], Token::Plaintext(0, 3)));
        // assert!(matches!(actual_result[0], Token::Plaintext("som")));
        assert!(matches!(actual_result[1], Token::Asterisk));
    }
    #[test]
    fn double_asterisk() {
        //string with space before pound sign should not be converted
        let input_str = "some**";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 2);
        assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
        //assert!(matches!(actual_result[0], Token::Plaintext("some")));

        assert!(matches!(actual_result[1], Token::DoubleAsterisk));
    }
    #[test]
    fn mixed() {
        //string with space before pound sign should not be converted
        let input_str = "some *";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 3);
        assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
        // assert!(matches!(actual_result[0], Token::Plaintext("some")));
        assert!(matches!(actual_result[1], Token::Space));
        assert!(matches!(actual_result[2], Token::Asterisk));
    }
    #[test]
    fn mixed_more() {
        //string with space before pound sign should not be converted
        let input_str = "some * here";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 5);
        assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
        //assert!(matches!(actual_result[0], Token::Plaintext("some")));
        assert!(matches!(actual_result[1], Token::Space));
        assert!(matches!(actual_result[2], Token::Asterisk));
        assert!(matches!(actual_result[3], Token::Space));
        assert!(matches!(actual_result[4], Token::Plaintext(7, 11)));
        // assert!(matches!(actual_result[4], Token::Plaintext("here")));
    }
}

#[cfg(test)]
mod test_token_parser {
    use crate::single_char_pattern::single_char_parser::FormatText;

    use super::token_parser;
    use super::Token;
    #[test]
    fn one_token() {
        let f1: FormatText<'_> = FormatText::new(true, "*");
        //string with space before pound sign should not be converted
        let s: &str = "*";
        let tokens = vec![Token::Asterisk];
        let output: String = token_parser(tokens, s);
        let expected_output = String::from("*");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn two_tokens() {
        //string with space before pound sign should not be converted
        let tokens: Vec<Token> = vec![Token::Asterisk, Token::Space];
        let s: &str = "* ";
        let output: String = token_parser(tokens, s);
        let expected_output = String::from("");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn three_tokens() {
        //string with space before pound sign should not be converted
        let s: &str = "* p";
        let tokens: Vec<Token> = vec![Token::Asterisk, Token::Space, Token::Plaintext(2, 3)];
        let output: String = token_parser(tokens, s);
        let expected_output = String::from("* _");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn longer_plaintext() {
        //string with space before pound sign should not be converted
        let s: &str = "some* ";
        let tokens: Vec<Token> = vec![Token::Asterisk, Token::Space];
        let output: String = token_parser(tokens, s);
        let expected_output = String::from("some* ");
        assert_eq!(output, expected_output);
    }
    #[test]
    fn short_italics() {
        //string with space before pound sign should not be converted
        let s: &str = "*some*";
        let tokens: Vec<Token> = vec![Token::Asterisk, Token::Space];
        let output: String = token_parser(tokens, s);
        let expected_output = String::from("</i>some</i>");
        assert_eq!(output, expected_output);
    }
}

mod mod_parsing_md {}

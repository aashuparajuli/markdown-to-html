use crate::single_char_pattern::single_char_parser::FormatText;

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
    }

    //if plaintext is still open, close it, then add
    if reading_plaintext {
        token_stream.push(Token::Plaintext(start_idx, str.len()));
        //token_stream.push(Token::Plaintext(&str[start_idx..]));
    }

    token_stream
}

impl FormatText<'_> {
    fn to_html(&self) -> String {
        match self.formatted {
            true => format!("<b>{}</b>", self.substring),
            false => self.substring.to_string(),
        }
    }
}
pub fn token_parser(tokens: Vec<Token>) -> String{
    let mut stack: Vec<FormatText> = Vec::new();
    let mut result: String = String::new();
    let mut parsing_formatted_text: bool = false;
    let mut start_idx: usize = 0;
    let mut curr_char: Option<CharType> = None; 

    for next_token in tokens {
        // match (curr_char, next_token){

        // }
    }
    stack
        .iter()
        .for_each(|subsection| result.push_str(&subsection.to_html()));
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

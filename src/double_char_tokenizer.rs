
#[derive(Clone, Copy, Debug)]
pub enum Token {
    Plaintext,
    Asterisk,
    Space,
    DoubleAsterisk,//each character, except double asterisk gets it own character
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
    use Token;
    //make it generic over any type that implements
    let mut token_stream: Vec<Token> = Vec::new();
    let mut prev_char: Option<CharType> = None;

    let mut _start_idx: usize = 0;
    for (_i, c) in str.char_indices() {
        let curr_char = CharType::new(c);
        match curr_char {
            CharType::Asterisk if matches!(prev_char, Some(CharType::Asterisk))=> {
                //remove pop token
                token_stream.pop();
                token_stream.push(Token::DoubleAsterisk)}
            CharType::Asterisk => {token_stream.push(Token::Asterisk)},
            CharType::Plaintext => {token_stream.push(Token::Plaintext)},
            CharType::Space => {token_stream.push(Token::Space)},
        };
        prev_char = Some(curr_char.clone());
    }
    token_stream
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
        assert_eq!(actual_result.len(), 4);
        assert!(matches!(actual_result[0], Token::Plaintext));
        assert!(matches!(actual_result[1], Token::Plaintext));
        assert!(matches!(actual_result[2], Token::Plaintext));
        assert!(matches!(actual_result[3], Token::Plaintext));
    }
    #[test]
    fn single_asterisk() {
        //string with space before pound sign should not be converted
        let input_str = "som*";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 4);
        assert!(matches!(actual_result[0], Token::Plaintext));
        assert!(matches!(actual_result[1], Token::Plaintext));
        assert!(matches!(actual_result[2], Token::Plaintext));
        assert!(matches!(actual_result[3], Token::Asterisk));
    }
    #[test]
    fn double_asterisk() {
        //string with space before pound sign should not be converted
        let input_str = "some**";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 5);
        assert!(matches!(actual_result[0], Token::Plaintext));
        assert!(matches!(actual_result[1], Token::Plaintext));
        assert!(matches!(actual_result[2], Token::Plaintext));
        assert!(matches!(actual_result[3], Token::Plaintext));
        assert!(matches!(actual_result[4], Token::DoubleAsterisk));
    }
    #[test]
    fn mixed() {
        //string with space before pound sign should not be converted
        let input_str = "some *";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 6)
    }
}

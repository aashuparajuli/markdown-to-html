pub enum Token{
    Plaintext,
    Asterisk,
    DoubleAsterisk,
}
enum CharType{
    Asterisk,
    Plaintext,
    Space
}
impl CharType {
    pub fn new(c: char) -> CharType{
        match c {
            '*' => CharType::Asterisk,
            ' ' => CharType::Space,
            _=> CharType::Plaintext,
        }
    }
}
pub fn double_char_tokenizer(str: &str) -> Vec<Token> {
    //make it generic over any type that implements
    let mut result: String = String::new();
    let mut token_stream: Vec<Token> = Vec::new();
    let mut start_idx: usize = 0;

    for c in str.chars() {
        let curr_char = CharType::new(c);
        let curr_token = match curr_char {
            CharType::Asterisk => Token::Asterisk,
            CharType::Space => Token::Plaintext,
            CharType::Plaintext =>  Token::Plaintext,
        };
        token_stream.push(curr_token)
    }
    token_stream

}

#[cfg(test)]
mod test_tokenizer {
    use super::double_char_tokenizer;
    use super::Token;
    #[test]
    fn convert_italics() {
        //string with space before pound sign should not be converted
        let input_str = "some *";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 6)
    }
}
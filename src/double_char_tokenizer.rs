#[derive(Clone, Copy)]
pub enum Token{
    Plaintext,
    Asterisk,
    Space,
    DoubleAsterisk,
}
#[derive(Clone, Copy)]
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
    use Token;
    //make it generic over any type that implements
    let mut token_stream: Vec<Token> = Vec::new();
    let mut prevChar: Option<CharType> = None;
    for c in str.chars() {
        let curr_char = CharType::new(c);
        match (prevChar, curr_char) {
            (None, CharType::Asterisk) => {todo!("append Asterisk token")},
            (None, CharType::Plaintext) => todo!("append plaintext token"),
            (None, CharType::Space) => todo!("Append space token"),
            (Some(CharType::Space), CharType::Asterisk) => todo!("append asterisk token"),
            (Some(CharType::Asterisk), CharType::Asterisk) => todo!("remove previous asterisk token, append DoubleAsterisk token"),
            (Some(CharType::Plaintext), CharType::Asterisk) => todo!("append append '*' as plaintext token"),

            (Some(CharType::Asterisk), CharType::Plaintext) => todo!("append plaintext token"),
            (Some(CharType::Plaintext), CharType::Plaintext) => todo!("append plaintext token"),
            (Some(CharType::Space), CharType::Plaintext) => todo!("append asterisk token"),

            (Some(CharType::Asterisk), CharType::Space) => todo!("append space token"),//should escape the asterisk, but assume that will be solved in the lexer for now
            (Some(CharType::Space), CharType::Space) => todo!("append space token"),//should escape the asterisk, but assume that will be solved in the lexer for now
            (Some(CharType::Plaintext), CharType::Space) => todo!("append space token"),//should escape the asterisk, but assume that will be solved in the lexer for now

        };
        prevChar = Some(curr_char.clone());
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
#[derive(Clone, Copy, Debug)]
pub enum Token {
    Plaintext,
    Asterisk,
    Space,
    //DoubleAsterisk, //each character, except double asterisk gets it own character
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
//Currently separates the entire text into individual character tokens, when it should group
//runs of plaintext together
pub fn double_char_tokenizer(str: &str) -> Vec<Token> {
    use Token;
    //make it generic over any type that implements
    let mut token_stream: Vec<Token> = Vec::new();
    let mut curr_token: Option<Token> = None;

    let mut start_idx: usize = 0;
    for (i, c) in str.char_indices() {
        let next_char = CharType::new(c);

        //preconditions, dealing with altering previous elements
        //if prev char is not plaintext && curr char is plaintext => set start_idx
        //if prev char is plaintext && curr char is not plaintext => use start_idx and i to get substring
        //if prev char is plaintext && curr char is plaintext, do nothing, let i continue growing
        //do nothing

        //otherwise, just push to stack

        match (curr_token, next_char) {
            (None, _) => {
                //just push the token
                let next_token: Token = match next_char {
                    CharType::Asterisk => Token::Asterisk,
                    CharType::Plaintext => Token::Plaintext,
                    CharType::Space => Token::Space,
                };
                curr_token = Some(next_token.clone());
                token_stream.push(next_token)
            }
            (Some(Token::Plaintext), CharType::Plaintext) => {
                //push the next character
                token_stream.push(Token::Plaintext)
            }
            (Some(Token::Plaintext), CharType::Asterisk) => {
                //end plaintext, push it
                let _tuple = (start_idx, i);
                //TODO: push the current plaintext, then add the asterisk after it
                //push asterisk
                token_stream.push(Token::Asterisk);
            }
            (Some(Token::Plaintext), CharType::Space) => {
                let _tuple = (start_idx, i);
                //push space
                token_stream.push(Token::Space);
            }
            (_, CharType::Asterisk) => token_stream.push(Token::Asterisk),
            (_, CharType::Plaintext) => token_stream.push(Token::Plaintext),
            (_, CharType::Space) => token_stream.push(Token::Space),
        };
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
        assert_eq!(actual_result.len(), 6);
        assert!(matches!(actual_result[0], Token::Plaintext));
        assert!(matches!(actual_result[1], Token::Plaintext));
        assert!(matches!(actual_result[2], Token::Plaintext));
        assert!(matches!(actual_result[3], Token::Plaintext));
        //assert!(matches!(actual_result[4], Token::DoubleAsterisk));
    }
    #[test]
    fn mixed() {
        //string with space before pound sign should not be converted
        let input_str = "some *";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 6)
    }
}

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
pub fn double_char_tokenizer(str: &str) -> Vec<Token> {
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
        if !matches!(curr_section, Some(CharType::Plaintext))
            && matches!(next_char, CharType::Plaintext)
        {
            start_idx = i;
            reading_plaintext = true;
        } else if matches!(curr_section, Some(CharType::Plaintext))
            && !matches!(next_char, CharType::Plaintext)
        {
            //append plaintext
            token_stream.push(Token::Plaintext);
            reading_plaintext = false;
        } else if matches!(curr_section, Some(CharType::Plaintext))
            && !matches!(next_char, CharType::Plaintext)
        {
            //do nothing
        }

        if matches!(next_char, CharType::Asterisk) {
            token_stream.push(Token::Asterisk);
        } else if matches!(next_char, CharType::Space) {
            token_stream.push(Token::Space);
        }

        // match (curr_section, next_char) {
        //     (None, _) => {
        //         //just push the token
        //         let next_token = match next_char {
        //             CharType::Asterisk => Token::Asterisk,
        //             CharType::Plaintext => Token::Plaintext,
        //             CharType::Space => Token::Space,
        //         };
        //         // curr_token = Some(next_token.clone());
        //         token_stream.push(next_token)
        //     }
        //     (Some(CharType::Plaintext), CharType::Plaintext) => {
        //         //do nothing
        //     }
        //     (Some(CharType::Plaintext), CharType::Asterisk) => {
        //         //end plaintext, push it
        //         let _tuple = (start_idx, i);
        //         //push asterisk
        //         token_stream.push(Token::Asterisk);
        //     }
        //     (Some(CharType::Plaintext), CharType::Space) => {
        //         let _tuple = (start_idx, i);
        //         //push space
        //         token_stream.push(Token::Space);
        //     }
        //     (_, CharType::Asterisk) => token_stream.push(Token::Asterisk),
        //     (_, CharType::Plaintext) => token_stream.push(Token::Plaintext),
        //     (_, CharType::Space) => token_stream.push(Token::Space),
        // };
        curr_section = Some(next_char);
    }
    
    //if plaintext is still open, close it, then add
    if reading_plaintext{
        token_stream.push(Token::Plaintext);
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
        assert_eq!(actual_result.len(), 1);
        assert!(matches!(actual_result[0], Token::Plaintext));
    }
    #[test]
    fn single_asterisk() {
        //string with space before pound sign should not be converted
        let input_str = "som*";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 2);
        assert!(matches!(actual_result[0], Token::Plaintext));
        assert!(matches!(actual_result[1], Token::Asterisk));
    }
    #[test]
    fn double_asterisk() {
        //string with space before pound sign should not be converted
        let input_str = "some**";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 3);
        assert!(matches!(actual_result[0], Token::Plaintext));
        assert!(matches!(actual_result[1], Token::Asterisk));
        assert!(matches!(actual_result[2], Token::Asterisk));
        //assert!(matches!(actual_result[2], Token::DoubleAsterisk));
    }
    #[test]
    fn mixed() {
        //string with space before pound sign should not be converted
        let input_str = "some *";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str);
        assert_eq!(actual_result.len(), 3);
        assert!(matches!(actual_result[0], Token::Plaintext));
        assert!(matches!(actual_result[1], Token::Space));
        assert!(matches!(actual_result[2], Token::Asterisk));
    }
}

use super::tokens::Token;

use crate::inline_parsing::tag::HtmlTag;
//use super::BOLD_ASTERISK_TAG;
#[derive(Clone, Copy)]
enum CharType {
    Asterisk,
    Plaintext,
    Space,
}
impl CharType {
    pub fn new(c: char, tag: char) -> CharType {
        if c == tag {
            CharType::Asterisk
        } else if c == ' ' {
            CharType::Space
        } else {
            CharType::Plaintext
        }
    }
}
pub fn double_char_tokenizer<'a>(str: &'a str, tag: &'a HtmlTag) -> Vec<Token<'a>> {
    if str.is_empty() {
        return Vec::new();
    }
    //make it generic over any type that implements
    let mut token_stream: Vec<Token> = Vec::new();
    let mut curr_section: Option<CharType> = None;
    let mut start_idx: usize = usize::max_value();
    let mut reading_plaintext: bool = false;
    for (i, c) in str.char_indices() {
        let next_char: CharType = CharType::new(c, tag.matching_char);

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
                //token_stream.push(Token::Plaintext(start_idx, i));
                token_stream.push(Token::Plaintext(&str[start_idx..i]));
                reading_plaintext = false;
            }
            (_, _) => (),
        };

        match next_char {
            CharType::Asterisk
                if matches!(token_stream.last(), Some(Token::SingleFormatChar(_))) =>
            {
                //trigger double asterisk
                token_stream.pop();
                token_stream.push(Token::DoubleFormatChar(tag));
            }
            CharType::Asterisk => {
                //push asterisk normally
                token_stream.push(Token::SingleFormatChar(c));
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
        //token_stream.push(Token::Plaintext(start_idx, str.len()));
        token_stream.push(Token::Plaintext(&str[start_idx..]));
    }

    token_stream
}
#[cfg(test)]
mod test_tokenizer {
    use super::double_char_tokenizer;
    use super::HtmlTag;
    use super::Token;
    const BOLD_ASTERISK_TAG: HtmlTag = HtmlTag {
        opening_tag: "<b>",
        closing_tag: "</b>",
        matching_char: '*',
    };
    #[test]
    fn basic() {
        //string with space before pound sign should not be converted
        let input_str = "some";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
        assert_eq!(actual_result.len(), 1);
        assert!(matches!(actual_result[0], Token::Plaintext("some")));
        //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
    }
    #[test]
    fn single_asterisk() {
        //string with space before pound sign should not be converted
        let input_str = "som*";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
        assert_eq!(actual_result.len(), 2);
        //assert!(matches!(actual_result[0], Token::Plaintext(0, 3)));
        assert!(matches!(actual_result[0], Token::Plaintext("som")));
        assert!(matches!(actual_result[1], Token::SingleFormatChar('*')));
    }
    #[test]
    fn valid_two_words() {
        //string with space before pound sign should not be converted
        let input_str = "**so me**";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
        assert_eq!(actual_result.len(), 5);
        assert!(matches!(
            actual_result[0],
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG)
        ));
        assert!(matches!(actual_result[1], Token::Plaintext("so")));
        assert!(matches!(actual_result[2], Token::Space));
        assert!(matches!(actual_result[3], Token::Plaintext("me")));
        assert!(matches!(
            actual_result[4],
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG)
        ));

        //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
    }
    #[test]
    fn double_asterisk() {
        //string with space before pound sign should not be converted
        let input_str = "some**";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
        assert_eq!(actual_result.len(), 2);
        //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
        assert!(matches!(actual_result[0], Token::Plaintext("some")));
        assert!(matches!(
            actual_result[1],
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG)
        ));
        //assert!(matches!(actual_result[1], Token::DoubleAsterisk("<b>","</b>")));
    }
    #[test]
    fn invalid_double_spaces() {
        //string with space before pound sign should not be converted
        //string with space before pound sign should not be converted
        let input_str = "** some **";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
        assert_eq!(actual_result.len(), 5);
        //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
        assert!(matches!(
            actual_result[0],
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG)
        ));
        assert!(matches!(actual_result[1], Token::Space));
        assert!(matches!(actual_result[2], Token::Plaintext("some")));
        assert!(matches!(actual_result[3], Token::Space));
        assert!(matches!(
            actual_result[4],
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG)
        ));
    }
    #[test]
    fn valid_double_spaces() {
        //string with space before pound sign should not be converted
        //string with space before pound sign should not be converted
        let input_str = "**some** ";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
        assert_eq!(actual_result.len(), 4);
        //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
        assert!(matches!(
            actual_result[0],
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG)
        ));
        assert!(matches!(actual_result[1], Token::Plaintext("some")));
        assert!(matches!(
            actual_result[2],
            Token::DoubleFormatChar(&BOLD_ASTERISK_TAG)
        ));
        assert!(matches!(actual_result[3], Token::Space));
    }
    #[test]
    fn mixed() {
        //string with space before pound sign should not be converted
        let input_str = "some *";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
        assert_eq!(actual_result.len(), 3);
        //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
        assert!(matches!(actual_result[0], Token::Plaintext("some")));
        assert!(matches!(actual_result[1], Token::Space));
        assert!(matches!(actual_result[2], Token::SingleFormatChar('*')));
    }
    #[test]
    fn mixed_more() {
        //string with space before pound sign should not be converted
        let input_str = "some * here";
        let actual_result: Vec<Token> = double_char_tokenizer(input_str, &BOLD_ASTERISK_TAG);
        assert_eq!(actual_result.len(), 5);
        //assert!(matches!(actual_result[0], Token::Plaintext(0, 4)));
        assert!(matches!(actual_result[0], Token::Plaintext("some")));
        assert!(matches!(actual_result[1], Token::Space));
        assert!(matches!(actual_result[2], Token::SingleFormatChar('*')));
        assert!(matches!(actual_result[3], Token::Space));
        //assert!(matches!(actual_result[4], Token::Plaintext(7, 11)));
        assert!(matches!(actual_result[4], Token::Plaintext("here")));
    }
}

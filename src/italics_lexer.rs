mod structs {
    #[derive(Clone, Debug)]
    pub enum HtmlTag {
        ItalicsAsterisk,
        ItalicsUnderscore,
    }

    impl HtmlTag {
        pub fn get_text(&self) -> &str {
            match self {
                HtmlTag::ItalicsAsterisk => "*",
                HtmlTag::ItalicsUnderscore => "_",
            }
        }
        pub fn format(&self, s: &str) -> String {
            match self {
                HtmlTag::ItalicsAsterisk | HtmlTag::ItalicsUnderscore => format!("<i>{}</i>", s),
            }
        }
    }
}
mod tokenizer {
    use super::structs;
    #[derive(Clone, Debug)]
    pub enum TokenType {
        Tag(structs::HtmlTag),
        Space,
        Plaintext { start: usize, end: usize },
    }
    impl TokenType {
        fn get_plain_text(&self) -> &str {
            match self {
                TokenType::Tag(x) => x.get_text(),
                TokenType::Space => todo!(),
                TokenType::Plaintext { start, end } => todo!(),
            }
        }
    }
    pub fn italics_tokenizer(str: String) -> Vec<TokenType> {
        let mut tokens: Vec<TokenType> = Vec::new();
        let mut start_idx: usize = 0;

        for (curr_idx, c) in str.chars().enumerate() {
            match c {
                '*' => {
                    //push the currently parsed tokens
                    if curr_idx != start_idx {
                        tokens.push(TokenType::Plaintext {
                            start: start_idx,
                            end: curr_idx,
                        });
                        // println!("created 2 token in *:{curr_idx},{c}");
                    } else {
                        // println!("created 1 tokens in *:{curr_idx},{c}");
                    }
                    //push the italics token
                    tokens.push(TokenType::Tag(structs::HtmlTag::ItalicsAsterisk));

                    start_idx = curr_idx + 1;
                }
                '_' => {
                    //push the currently parsed tokens
                    if curr_idx != start_idx {
                        tokens.push(TokenType::Plaintext {
                            start: start_idx,
                            end: curr_idx,
                        });
                        // println!("created 2 token in *:{curr_idx},{c}");
                    } else {
                        // println!("created 1 tokens in *:{curr_idx},{c}");
                    }
                    //push the italics token
                    tokens.push(TokenType::Tag(structs::HtmlTag::ItalicsUnderscore));

                    start_idx = curr_idx + 1;
                }

                ' ' => {
                    //push the currently parsed plaintext, if there is any
                    if curr_idx != start_idx {
                        tokens.push(TokenType::Plaintext {
                            start: start_idx,
                            end: curr_idx,
                        });
                        // println!("created 2 token in *:{curr_idx},{c}");
                    } else {
                        // println!("created 1 tokens in space:{curr_idx},{c}");
                    }
                    //push the space token
                    tokens.push(TokenType::Space);
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
            tokens.push(TokenType::Plaintext {
                start: start_idx,
                end: str.len(),
            });
        }
        tokens
    }
    #[cfg(test)]
    mod test_tokenizer {
        use super::italics_tokenizer;
        use super::structs::HtmlTag;
        use super::TokenType;
        #[test]
        fn valid_tokens() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some *text*");
            let actual_result = italics_tokenizer(input_str);
            assert_eq!(5, actual_result.len());
            matches!(actual_result[0], TokenType::Plaintext { start: 0, end: 4 });
            matches!(actual_result[1], TokenType::Space);
            matches!(actual_result[2], TokenType::Tag(HtmlTag::ItalicsAsterisk));
            matches!(actual_result[3], TokenType::Plaintext { start: 5, end: 9 });
            matches!(actual_result[4], TokenType::Tag(HtmlTag::ItalicsAsterisk));
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
            assert!(matches!(
                actual_result[0],
                TokenType::Plaintext { start: _, end: _ }
            ));
            assert!(matches!(actual_result[1], TokenType::Space));
            assert!(matches!(
                actual_result[2],
                TokenType::Tag(HtmlTag::ItalicsAsterisk)
            ));
            assert!(matches!(
                actual_result[3],
                TokenType::Plaintext { start: _, end: _ }
            ));
            assert!(matches!(actual_result[4], TokenType::Space));
            assert!(matches!(
                actual_result[5],
                TokenType::Tag(HtmlTag::ItalicsAsterisk)
            ));
        }
        #[test]
        fn invalid_two_spaces() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some * text *");
            let actual_result = italics_tokenizer(input_str);
            assert_eq!(7, actual_result.len());
            assert!(matches!(
                actual_result[0],
                TokenType::Plaintext { start: _, end: _ }
            ));
            assert!(matches!(actual_result[1], TokenType::Space));
            assert!(matches!(
                actual_result[2],
                TokenType::Tag(HtmlTag::ItalicsAsterisk)
            ));
            assert!(matches!(actual_result[3], TokenType::Space));
            assert!(matches!(
                actual_result[4],
                TokenType::Plaintext { start: _, end: _ }
            ));
            assert!(matches!(actual_result[5], TokenType::Space));
            assert!(matches!(
                actual_result[6],
                TokenType::Tag(HtmlTag::ItalicsAsterisk)
            ));
        }
        #[test]
        fn invalid_two_asterisk() {
            //string with space before pound sign should not be converted
            let input_str = String::from("some **text");
            let actual_result = italics_tokenizer(input_str);
            assert_eq!(5, actual_result.len());
            assert!(matches!(
                actual_result[0],
                TokenType::Plaintext { start: _, end: _ }
            ));
            assert!(matches!(actual_result[1], TokenType::Space));
            assert!(matches!(
                actual_result[2],
                TokenType::Tag(HtmlTag::ItalicsAsterisk)
            ));
            assert!(matches!(
                actual_result[3],
                TokenType::Tag(HtmlTag::ItalicsAsterisk)
            ));
            assert!(matches!(
                actual_result[4],
                TokenType::Plaintext { start: _, end: _ }
            ));
        }
    }
}
mod parser {
    use super::structs::HtmlTag;
    use std::fmt::format;

    use super::tokenizer::TokenType;
    trait Stack {
        fn second_last(&self) -> Option<&Substring>;
        fn add_char(&mut self, substr: &str);
    }
    impl Stack for Vec<Substring> {
        fn second_last(&self) -> Option<&Substring> {
            if self.len() < 2 {
                None
            } else {
                Some(&self[self.len() - 2])
            }
        }
        fn add_char(&mut self, substr: &str) {
            match self.last_mut() {
                Some(Substring::Plaintext(x)) => x.push_str(substr),
                _ => self.push(Substring::Plaintext(substr.to_string())),
            }
        }
    }

    /*There are 2 types of formatted strings:
       tokens, that are simply the start and end indices from a string
       or raw-unformatted opening tags
       formatted strings, which are tokens that also contain html tags
    */

    enum Substring {
        //Each substring is either a tag(only italics for now) or plaintext
        Tag(HtmlTag),
        Plaintext(String),
    }
    impl Substring {
        fn get_text(&self) -> String {
            match self {
                Substring::Tag(HtmlTag::ItalicsAsterisk) => String::from("*"),
                Substring::Tag(HtmlTag::ItalicsUnderscore) => String::from("_"),
                Substring::Plaintext(x) => x.to_owned(),
            }
        }
    }

    pub fn parse_tokens(tokens: &Vec<TokenType>, full_string: &str) -> String {
        let mut result: String = String::new();
        let mut stack: Vec<Substring> = Vec::new();
        let mut previousToken: Option<TokenType> = None;
        for token in tokens {
            //use the current character and the top of the stack to make a decision
            /*

            if second is italics + top is plaintext + current is italics
                => format with <i> tags and place on top of stack
            else if top is italics + current is space
                => escape into plaintext
            else curr is italics
              => push italics
            else
                => push the token
            */
            //the only time formatting tags can be appending to strings is when a TokenType::Italics is added (or other tags in the future)
            //in all other cases, changes will happen, but no text will be re-formatted
            use HtmlTag::ItalicsAsterisk as Asterisk;
            use HtmlTag::ItalicsUnderscore as Underscore;
            match (token, stack.last()) {
                (TokenType::Tag(x), None) => {
                    //the italics is the first things on the stack
                    //push italics
                    stack.push(Substring::Tag(x.clone()));
                }
                (TokenType::Space, None) => {
                    //if stack is empty, push space as plaintext
                    stack.push(Substring::Plaintext(String::from(" ")))
                }
                (TokenType::Tag(x), Some(Substring::Plaintext(s)))
                    if matches!(stack.second_last(), Some(Substring::Tag(x))) =>
                {
                    //if curr is italics, top is plaintext && second is italics, format text, push it
                    let formatted_text = x.format(s);
                    stack.pop(); //pop plaintext
                    stack.pop(); //pop opening italics
                                 //push text with italics tags
                    stack.push(Substring::Plaintext(formatted_text))
                }
                (TokenType::Tag(x), Some(Substring::Tag(y))) if matches!(x, y) => {
                    //if two consecutive identical tags, convert both into plaintext: **
                    stack.pop();
                    //stack.push(Substring::Plaintext(String::from("**")));
                    stack.add_char(x.get_text());
                    stack.add_char(x.get_text());
                    //todo!("either append to current plaintext or create new plaintext at the top")
                }
                (TokenType::Tag(_), Some(Substring::Tag(y))) => {
                    //if two consecutive different tags: push the second one
                    stack.push(Substring::Tag(y.clone()));
                }
                (TokenType::Tag(x), Some(Substring::Plaintext(_))) => {
                    //if curr is tag, top is plaintext, then push tag}
                    stack.push(Substring::Tag(x.clone()));
                }
                (TokenType::Space, Some(Substring::Tag(x))) => {
                    //if curr is space and top is italics, escape the italics
                    // stack.pop();
                    // //stack.push(Substring::Plaintext(String::from("**")));
                    // stack.add_char(x.get_text());
                    // stack.add_char(x.get_text());

                    // stack.pop();
                    // stack.add_char(x.clone().get_text());
                    // stack.add_char(" ");
                }
                (TokenType::Space, Some(Substring::Plaintext(_))) => {
                    // if Some(Plaintext), append space
                    stack.add_char(" ");
                }
                (TokenType::Plaintext { start, end }, _) => {
                    //add this substring to the stack
                    stack.add_char(&full_string[*start..*end])
                }
            };
            previousToken = Some(token.clone());
        }
        stack
            .iter()
            .for_each(|subsection: &Substring| result.push_str(&subsection.get_text()));
        result
    }
}
pub fn parse_italics(input_str: String) -> String {
    //println!("{input_str}");
    let tokens = tokenizer::italics_tokenizer(input_str.clone());
    parser::parse_tokens(&tokens, &input_str)
}

#[cfg(test)]
mod italics_asterisk_tests {
    use super::parse_italics;
    #[test]
    fn valid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *text*");
        let expected_result = String::from("some <i>text</i>");
        let actual_result = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn plain_text() {
        //string with space before pound sign should not be converted
        let input_str = String::from("plain text");
        let expected_result = String::from("plain text");
        let actual_result = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn valid_one_space() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *text *");
        let expected_result = String::from("some <i>text </i>");
        let actual_result = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some * text *");
        let expected_result = String::from("some * text *");
        let actual_result: String = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn valid_no_space() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some*text*a");
        let expected_result = String::from("some<i>text</i>");
        let actual_result = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some **text");
        let expected_result = String::from("some **text");
        let actual_result: String = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn unusual() {
        //string with space before pound sign should not be converted
        let input_str = String::from("  s - s");
        let expected_result = String::from("  s - s");
        let actual_result: String = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }

    fn text_around() {
        //string with space before pound sign should not be converted
        let input_str = String::from("a*a a*a");
        let expected_result = String::from("a*a a*a");
        let actual_result: String = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
}
#[cfg(test)]
mod italics_underscore_tests {
    use super::parse_italics;
    #[test]
    fn valid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _text_");
        let expected_result = String::from("some <i>text</i>");
        let actual_result = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn plain_text() {
        //string with space before pound sign should not be converted
        let input_str = String::from("plain text");
        let expected_result = String::from("plain text");
        let actual_result = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn valid_one_space() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _text _");
        let expected_result = String::from("some <i>text </i>");
        let actual_result = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _ text _");
        let expected_result = String::from("some _ text _");
        let actual_result: String = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn convert_italics_invalid_2() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some **text");
        let expected_result = String::from("some **text");
        let actual_result: String = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    fn text_around() {
        //string with space before pound sign should not be converted
        let input_str = String::from("a_a a_a");
        let expected_result = String::from("a_a a_a");
        let actual_result: String = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
}

#[cfg(test)]
mod mixed_underscore_asterisk_tests {
    use super::parse_italics;
    #[test]
    fn mixed() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _text*");
        let expected_result = String::from("some _text*");
        let actual_result = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn inner_asterisk() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some _*text _");
        let expected_result = String::from("some <i>*text </i>");
        let actual_result = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
    #[test]
    fn inner_underscore() {
        //string with space before pound sign should not be converted
        let input_str = String::from("some *_text *");
        let expected_result = String::from("some <i>_text </i>");
        let actual_result = parse_italics(input_str);
        assert_eq!(actual_result, expected_result);
    }
}

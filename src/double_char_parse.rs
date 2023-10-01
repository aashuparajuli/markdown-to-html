mod wrappers;
mod double_char_tokenizer;
mod tokens;

pub use wrappers::bold::parse_bold_asterisk;
pub use wrappers::bold::parse_bold_underscore;

pub use wrappers::strikethrough::parse_strikethrough;


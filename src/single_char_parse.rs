mod wrapper;
mod token;
mod tokenizer;
mod generate_html;
pub mod single_char_parser;
pub use wrapper::inline_code::process_inline_code;
pub use wrapper::italics::process_asterisk as italics_asterisk;
pub use wrapper::italics::process_underscore as italics_underscore;
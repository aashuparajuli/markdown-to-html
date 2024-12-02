pub mod single_char_parser;
mod token;
mod tokenizer;
mod wrapper;
pub use wrapper::inline_code::process_inline_code;
pub use wrapper::italics::process_asterisk as italics_asterisk;
pub use wrapper::italics::process_underscore as italics_underscore;

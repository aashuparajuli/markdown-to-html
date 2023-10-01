mod single_char_wrappers;
pub mod single_char_parser;
pub use single_char_wrappers::inline_code::process_inline_code;
pub use single_char_wrappers::italics::process_asterisk as italics_asterisk;
pub use single_char_wrappers::italics::process_underscore as italics_underscore;
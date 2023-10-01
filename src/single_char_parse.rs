mod parsers;
pub mod single_char_pattern;
pub use parsers::inline_code::process_inline_code;
pub use parsers::italics::process_asterisk as italics_asterisk;
pub use parsers::italics::process_underscore as italics_underscore;
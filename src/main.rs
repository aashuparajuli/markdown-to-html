#![allow(dead_code, unused_imports)]

mod bold;
mod file_io;
use clap::Parser;
mod double_char_tokenizer;
mod full_line_parsing;
mod parsers;
mod parse_bold_underscore;
mod parse_markdown;
mod single_char_pattern;
mod strikethrough;
use file_io::FileAccess;
use std::path::PathBuf;
use std::time::{Duration, Instant};
#[derive(Parser)]
struct Cli {
    input_file_path: PathBuf,
    output_file_path: PathBuf,
}
fn main() {
    let start_reading_file = Instant::now();
    let args = Cli::parse();
    // let input_file_name = "./benchmarks/benchmark1/input.md";
    // let output_file_name = "./benchmarks/benchmark1/output.html";

    // let input_file_name = "./input/input.txt";
    let input_file_name = args.input_file_path;
    let output_file_name = args.output_file_path;

    let input_lines = file_io::get_file_lines(input_file_name); //get the lines from the file
    let reading_file_duration: Duration = start_reading_file.elapsed();

    let start_parsing_text = Instant::now();
    let mut file_access: file_io::FileAccess = FileAccess::open_file(output_file_name);
    parse_markdown::parse_all_lines(input_lines, &mut file_access); //process the lines
    let parsing_text_duration = start_parsing_text.elapsed();

    //let start_writing_file = Instant::now();
    //file_io::write_line_to_file_true(&output_lines, output_file_name);
    //let writing_file_duration: Duration = start_writing_file.elapsed();
    let total_time: Duration = start_reading_file.elapsed();
    println!("Time to read:{:?}", reading_file_duration,);
    println!("Time to parse:{:?}", parsing_text_duration,);
    //println!("Time to write:{:?}", writing_file_duration,);
    println!("Time to write:{:?}", 0);
    println!("total:{:?}", total_time);
}

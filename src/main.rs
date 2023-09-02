#![allow(dead_code, unused_imports)]
mod file_io;
mod parse_line_formatting;
mod parse_text_formatting;
mod stack;
use std::time::{Duration, Instant};

fn main() {
    let input_file_name = "./benchmarks/benchmark1/input.md";
    let output_file_name = "./benchmarks/benchmark1/output.html";
    let start_reading_file = Instant::now();
    let input_lines = file_io::get_file_lines(input_file_name); //get the lines from the file
    let reading_file_duration: Duration = start_reading_file.elapsed();

    let start_parsing_text = Instant::now();
    let output_lines: Vec<String> = parse_line_formatting::parse_all_lines(input_lines); //process the lines
    let parsing_text_duration = start_parsing_text.elapsed();

    let start_writing_file = Instant::now();
    file_io::write_line_to_file_true(&output_lines, output_file_name);
    let writing_file_duration = start_writing_file.elapsed();
    let total_time = start_reading_file.elapsed();
    println!("Time to read:{:?}", reading_file_duration,);
    println!("Time to parse:{:?}", parsing_text_duration,);
    println!("Time to write:{:?}", writing_file_duration,);
    println!("total:{:?}", total_time);
}

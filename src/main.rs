#![allow(dead_code, unused_imports, unused_variables, unused_mut)]

mod file_io;
mod parse_line_formatting;
mod parse_text_formatting;
mod stack;
use file_io::FileAccess;
use std::time::{Duration, Instant};
fn main() {
    let input_file_name = "./benchmarks/benchmark1/input.md";
    let output_file_name = "./benchmarks/benchmark1/output.html";

    // let input_file_name = "./input/input.txt";
    // let output_file_name = "./output/output.html";

    let start_reading_file = Instant::now();
    let input_lines = file_io::get_file_lines(input_file_name); //get the lines from the file
    let reading_file_duration: Duration = start_reading_file.elapsed();

    let start_parsing_text = Instant::now();
    let mut file_access: file_io::FileAccess = FileAccess::open_file(output_file_name);
    let output_lines: Vec<String> =
        parse_line_formatting::parse_all_lines(input_lines, &mut file_access); //process the lines
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

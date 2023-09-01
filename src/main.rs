#![allow(dead_code, unused_imports, unused_variables)]
mod file_io;
mod parse_line_formatting;
mod parse_text_formatting;
mod stack;
fn main() {
    let input_file_name = "./benchmarks/benchmark1/input.md";
    let output_file_name = "./benchmarks/benchmark1/output.html";
    // let input_file_name = "./input/input.md";
    // let output_file_name = "./output/output.html";
    let input_lines = file_io::get_file_lines(input_file_name); //get the lines from the file
    let output_lines: Vec<String> = parse_line_formatting::parse_all_lines(input_lines); //process the lines

    file_io::write_line_to_file_true(&output_lines, output_file_name);
}

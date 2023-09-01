#![allow(dead_code, unused_imports)]
mod parse_line_formatting;
mod parse_text_formatting;
mod stack;
pub mod file_io {
    /**
     * Functions to read/write lines from a file
     */
    use std::fs::File;
    use std::io::Write;
    use std::io::{self, BufRead};
    use std::path::Path;

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where
        P: AsRef<Path>,
    {
        let file = File::open(filename)?;
        Ok(io::BufReader::new(file).lines())
    }
    pub fn get_file_lines(filename: &str) -> Vec<String> {
        //let filename = "./hosts.txt";
        let mut file_lines: Vec<String> = Vec::new();
        if let Ok(lines) = read_lines(filename) {
            // Consumes the iterator, returns an (Optional) String
            for line in lines {
                let ip = line.unwrap();
                file_lines.push(ip);
            }
        }
        file_lines
    }

    pub fn write_line_to_file_true(str: &Vec<String>, filename: &str) {
        //let data = "Some data!";
        let mut f = File::create(filename).expect("Unable to create file");
        for s in str {
            f.write_all(s.as_bytes()).expect("Unable to write data");
        }
    }
    pub fn write_line_to_file(str: &str, file: &mut Vec<String>) {
        //write a line to a file
        //TODO: should write to file
        file.push(str.to_string());
    }
}

fn main() {
    let input_file_name = "./benchmarks/benchmark1/input.md";
    let output_file_name = "./benchmarks/benchmark1/output.html";
    // let input_file_name = "./input/input.md";
    // let output_file_name = "./output/output.html";
    let input_lines: Vec<String> = file_io::get_file_lines(input_file_name); //get the lines from the file
    let output_lines = parse_line_formatting::parse_all_lines(input_lines); //process the lines

    file_io::write_line_to_file_true(&output_lines, output_file_name);
}

use std::fs::read_to_string;
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
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
pub struct FileAccess {
    file: File,
}
impl FileAccess {
    pub fn open_file(filename: &str) -> FileAccess {
        let file = File::create(filename).expect("Unable to create file");
        FileAccess { file }
    }
    pub fn write_to_file(&mut self, str: &str) {
        //("writing to file: {}", str);
        self.file
            .write_all(str.as_bytes())
            .expect("Unable to write data");
    }
}

pub fn write_one_line_to_file(s: &str, filename: &str) {
    //let data = "Some data!";
    let mut f = File::create(filename).expect("Unable to create file");
    f.write_all(s.as_bytes()).expect("Unable to write data");
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

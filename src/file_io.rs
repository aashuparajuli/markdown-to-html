use std::fs::read_to_string;
/**
 * Functions to read/write lines from a file
 */
use std::fs::File;
use std::io::Write;

pub fn get_file_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap() // panic on possible file-reading errors
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}
/**
 * Trait that represents the general ability to write to a file
 * Implemented by FileWriter, for release, or by Vec<String> for testing
 */
pub trait FileWriter {
    fn write_line_to_file(&mut self, line: &str);
}
/**
 * Enables Vec<String> to be used as a proxy for file output during tests
 */
impl FileWriter for Vec<String> {
    fn write_line_to_file(&mut self, line: &str) {
        self.push(line.to_string());
    }
}
impl FileWriter for FileAccess {
    fn write_line_to_file(&mut self, s: &str) {
        self.file
            .write_all(s.as_bytes())
            .expect("Unable to write data");
    }
}
pub struct FileAccess {
    file: File,
}
impl FileAccess {
    pub fn open_file(filename: &str) -> FileAccess {
        let file = File::create(filename).expect("Unable to create file");
        FileAccess { file }
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

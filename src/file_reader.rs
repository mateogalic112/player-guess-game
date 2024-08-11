use std::fs::{read_to_string, File};
use std::io::ErrorKind;

pub fn get_file_content(filename: &str) -> String {
    let file_content = read_to_string(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(filename).expect("Failed to create file");
            return String::from("");
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });

    file_content
}

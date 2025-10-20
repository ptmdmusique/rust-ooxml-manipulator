use prompted::input;
use std::path::Path;

use crate::utils::error::print_error_with_panic;

pub struct FilePathInfo {
    pub file_name: String,
    pub file_path: String,
}

impl FilePathInfo {
    pub fn new(file_path: String) -> Self {
        // https://stackoverflow.com/questions/73845791/how-to-remove-path-and-get-the-filename-in-rust
        let path = Path::new(&file_path);

        if !path.is_file() {
            print_error_with_panic(&format!("The path is not a file {}", file_path));
        }

        let file_name = match path.file_name() {
            Some(file_name) => file_name.to_str().unwrap(),
            None => {
                print_error_with_panic(&format!("Path doesn't contain a file name {}", file_path))
            }
        };
        Self {
            file_name: file_name.to_string(),
            file_path,
        }
    }

    pub fn print_info(&self) {
        println!("Full path: {} --- Name: {}", self.file_path, self.file_name);
    }
}

pub fn get_file_path() -> FilePathInfo {
    let input_path = input!("Enter the path of the file: ");
    FilePathInfo::new(input_path)
}

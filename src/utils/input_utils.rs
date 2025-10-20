use crate::utils::error::print_error_with_panic;
use colored::Colorize;
use prompted::input;
use std::path::Path;

// Info about the file path
pub struct FilePathInfo {
    // Name of the file without extension
    pub file_name: String,
    pub file_extension: String,
    pub file_path: String,
}

impl FilePathInfo {
    pub fn new(file_path: String) -> Self {
        // https://stackoverflow.com/questions/73845791/how-to-remove-path-and-get-the-filename-in-rust
        let path = Path::new(&file_path);

        if !path.is_file() {
            print_error_with_panic(&format!("The path is not a file {}", file_path));
        }

        let full_file_name = match path.file_name() {
            Some(file_name) => file_name.to_str().unwrap(),
            None => {
                print_error_with_panic(&format!("Path doesn't contain a file name {}", file_path))
            }
        };

        let file_extension = match path.extension() {
            Some(extension) => extension.to_str().unwrap().to_string(),
            None => print_error_with_panic(&format!("File extension is not valid {}", file_path)),
        };

        let file_name = full_file_name.split('.').next().unwrap().to_string();

        Self {
            file_name,
            file_extension,
            file_path,
        }
    }

    pub fn print_info(&self) {
        println!("{}", "File information:".green());
        println!(
            "\tFull path: {}\n\tName: {}\n\tExtension: {}",
            self.file_path, self.file_name, self.file_extension
        );
    }
}

pub fn get_file_path() -> FilePathInfo {
    let input_path = input!("Enter the path of the file: ");
    FilePathInfo::new(input_path)
}

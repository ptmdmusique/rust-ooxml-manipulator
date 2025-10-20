use crate::utils::print_utils::print_error_with_panic;
use colored::Colorize;
use std::path::Path;

/// Info about the file path
pub struct FilePathInfo {
    /// Name of the file without extension
    pub file_name: String,
    pub file_extension: String,
    /// File path without the file name
    pub file_path: String,
    pub full_file_path: String,
}

impl FilePathInfo {
    pub fn new(full_file_path: String) -> Self {
        // https://stackoverflow.com/questions/73845791/how-to-remove-path-and-get-the-filename-in-rust
        let path = Path::new(&full_file_path);

        let full_file_name = match path.file_name() {
            Some(file_name) => file_name.to_str().unwrap(),
            None => print_error_with_panic(&format!(
                "Path doesn't contain a file name {}",
                full_file_path
            )),
        };

        let file_extension = match path.extension() {
            Some(extension) => extension.to_str().unwrap().to_string(),
            None => {
                print_error_with_panic(&format!("File extension is not valid {}", full_file_path))
            }
        };

        let file_name = full_file_name.split('.').next().unwrap().to_string();

        let file_path_without_file_name = path.parent().unwrap().to_str().unwrap().to_string();

        Self {
            file_name,
            file_extension,
            file_path: file_path_without_file_name,
            full_file_path,
        }
    }

    pub fn print_info(&self) {
        println!("{}", "File information:".green());
        println!(
            "\tFull path: {}\n\tName: {}\n\tExtension: {}",
            self.full_file_path, self.file_name, self.file_extension
        );
    }
}

// ---

pub struct ZipFolder {
    /// The root folder of the zip folder
    pub root_folder: String,
    /// The folder where the zip file will be extracted
    pub extracted_folder: String,
}

use crate::utils::print_utils::print_error_with_panic;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::Path;

// * ---
/// Info about the file path
pub struct FilePathInfo {
    /// Name of the file without extension
    pub file_name: String,
    pub file_extension: Option<String>,
    pub file_name_with_extension: String,
    /// File path without the file name
    pub file_path: String,
    pub full_file_path: String,
    pub file_size: u64,
}

impl FilePathInfo {
    pub fn new(full_file_path: String) -> Self {
        // https://stackoverflow.com/questions/73845791/how-to-remove-path-and-get-the-filename-in-rust
        let path = Path::new(&full_file_path);

        let file_name_with_extension = match path.file_name() {
            Some(file_name) => file_name.to_str().unwrap(),
            None => print_error_with_panic(&format!(
                "Path doesn't contain a file name {}",
                full_file_path
            )),
        };

        let file_extension = path
            .extension()
            .map(|extension| extension.to_string_lossy().to_string());
        let file_size = match path.metadata() {
            Ok(metadata) => metadata.len(),
            Err(e) => print_error_with_panic(&format!("Failed to get file size: {}", e)),
        };

        let file_name = match file_name_with_extension.split('.').next() {
            Some(file_name) => file_name.to_string(),
            None => print_error_with_panic(&format!(
                "File name doesn't have a extension: {}",
                full_file_path
            )),
        };

        let file_path_without_file_name = match path.parent() {
            Some(parent) => parent.to_str().unwrap().to_string(),
            None => print_error_with_panic(&format!(
                "File path doesn't have a parent: {}",
                full_file_path
            )),
        };

        Self {
            file_name,
            file_extension,
            file_name_with_extension: file_name_with_extension.to_string(),
            file_path: file_path_without_file_name,
            full_file_path,
            file_size,
        }
    }

    pub fn print_info(&self) {
        println!("{}", "File information:".green());

        let extension = self.file_extension.as_ref().unwrap();
        println!(
            "\tFull path: {}\n\tName: {}\n\tExtension: {}",
            self.full_file_path, self.file_name, extension
        );
    }
}

// * ---

pub struct ZipFolder {
    /// The root folder of the zip folder
    pub root_folder: String,
    /// The folder where the zip file will be extracted
    pub extracted_folder: String,
}

#[derive(Serialize, Deserialize)]
pub struct FileInfo {
    /// Name of the file with extension
    pub file_name_with_extension: String,
    /// Full file path
    pub full_file_path: String,
    /// File size in KB
    pub file_size_in_kb: f64,
}

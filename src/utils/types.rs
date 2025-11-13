use crate::utils::{
    files::{read_struct_from_json, write_struct_to_json},
    print_utils::print_error_with_panic,
};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const EXTRACTED_FOLDER_NAME: &str = "extracted";
pub const SUMMARY_FILE_NAME: &str = "summary.json";
/// The name of the analyzed custom XML file
pub const CUSTOM_XML_FILE_NAME: &str = "customXml.json";
/// The path to the preference file that store user's last used params
const PREFERENCE_FILE_PATH: &str = "preference.json";

/// The path to the fixture folder
#[cfg(test)]
pub const FIXTURE_FOLDER_PATH: &str = ".local";

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

/// The user preference that stores the last used feature and file path
///
/// ! Note that this will automatically save to file when the last used feature, file path or folder path is changed
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserPreference {
    pub last_used_feature: String,
    pub last_used_file_path: String,
    /// Folder containing the extracted files
    pub last_used_extracted_folder_path: String,
    /// The path to the output file that will be created
    pub last_used_output_file_path: String,
    /// The path to the root folder that contains the extracted and other files such as customXml.json and summary.json
    pub last_used_root_folder_path: String,
}

impl UserPreference {
    pub fn new() -> Self {
        match read_struct_from_json::<UserPreference>(PREFERENCE_FILE_PATH) {
            Ok(user_preference) => user_preference,
            Err(_) => {
                let user_preference = Self {
                    last_used_feature: "N/A".to_string(),
                    last_used_file_path: "N/A".to_string(),
                    last_used_extracted_folder_path: "N/A".to_string(),
                    last_used_output_file_path: "N/A".to_string(),
                    last_used_root_folder_path: "N/A".to_string(),
                };
                user_preference.save_to_file();
                user_preference
            }
        }
    }

    pub fn save_to_file(&self) {
        let _ = write_struct_to_json(self, PREFERENCE_FILE_PATH);
    }

    pub fn save_last_used_file_path(&mut self, file_path: String) {
        self.last_used_file_path = file_path;
        self.save_to_file()
    }

    pub fn save_last_used_folder_path(&mut self, folder_path: String) {
        self.last_used_extracted_folder_path = folder_path;
        self.save_to_file()
    }

    pub fn save_last_used_feature(&mut self, feature: String) {
        self.last_used_feature = feature;
        self.save_to_file()
    }

    pub fn save_last_used_output_file_path(&mut self, last_used_output_file_path: String) {
        self.last_used_output_file_path = last_used_output_file_path;
        self.save_to_file()
    }

    pub fn save_last_used_root_folder_path(&mut self, root_folder_path: String) {
        self.last_used_root_folder_path = root_folder_path;
        self.save_to_file()
    }
}


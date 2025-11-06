use prompted::input;

use crate::utils::types::{FilePathInfo, UserPreference};

/// Get the file path from the input or use preference
pub fn get_file_path_from_input(user_preference: &mut UserPreference) -> FilePathInfo {
    let last_input_path = user_preference.clone().last_used_file_path;

    let mut input_path = input!(
        "{}",
        format!(
            "Enter the path of the file (default: {}): ",
            last_input_path
        )
    );

    if input_path.is_empty() {
        input_path = last_input_path;
    } else if input_path != last_input_path {
        user_preference.save_last_used_file_path(input_path.clone());
    }

    FilePathInfo::new(input_path)
}

// * Rezip stuff
/// Get the folder path from the input for rezip
pub fn get_folder_path_from_input_for_rezip(user_preference: &mut UserPreference) -> String {
    let last_input_path = user_preference.clone().last_used_extracted_folder_path;

    let mut input_path = input!(
        "{}",
        format!(
            "Enter the path of folder that contains the extracted folder (default: {}): ",
            last_input_path
        )
    );

    if input_path.is_empty() {
        input_path = last_input_path;
    } else if input_path != last_input_path {
        user_preference.save_last_used_folder_path(input_path.clone());
    }

    input_path
}

/// Get the output file path from the input for rezip
pub fn get_output_file_path_from_input_for_rezip(user_preference: &mut UserPreference) -> String {
    let last_input_path = user_preference.clone().last_used_output_file_path;

    let mut input_path = input!(
        "{}",
        format!(
            "Enter the path of the output file that will be created (default: {}): ",
            last_input_path
        )
    );

    if input_path.is_empty() {
        input_path = last_input_path;
    } else if input_path != last_input_path {
        user_preference.save_last_used_output_file_path(input_path.clone());
    }

    input_path
}

// * Other

/// Get the folder path containing the extracted folder and other files
pub fn get_extracted_root_folder_path(user_preference: &mut UserPreference) -> String {
    let last_input_path = user_preference.clone().last_used_root_folder_path;

    let mut input_path = input!(
        "{}",
        format!(
            "Enter the path of folder that contains the extracted folder and other files (default: {}): ",
            last_input_path
        )
    );

    if input_path.is_empty() {
        input_path = last_input_path;
    } else if input_path != last_input_path {
        user_preference.save_last_used_root_folder_path(input_path.clone());
    }

    input_path
}

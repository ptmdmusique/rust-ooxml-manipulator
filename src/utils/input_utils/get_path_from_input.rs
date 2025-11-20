use colored::Colorize;
use prompted::input;

use crate::utils::types::{FilePathInfo, UserPreference};

/// Get the file path from the input or use preference
pub fn get_file_path_from_input(user_preference: &mut UserPreference) -> FilePathInfo {
    let last_input_path = user_preference.clone().last_used_file_path;

    let prompt_text = format!(
        "{} Enter file path {} [last used: {}]: ",
        "┌─".bright_cyan(),
        "─┐".bright_cyan(),
        last_input_path.bright_yellow()
    );
    let mut input_path = input!("{}", prompt_text);

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

    let prompt_text = format!(
        "{} Enter folder path (typically the extracted folder) {} [last used: {}]: ",
        "┌─".bright_cyan(),
        "─┐".bright_cyan(),
        last_input_path.bright_yellow()
    );
    let mut input_path = input!("{}", prompt_text);

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

    let prompt_text = format!(
        "{} Enter output file path {} [last used: {}]: ",
        "┌─".bright_cyan(),
        "─┐".bright_cyan(),
        last_input_path.bright_yellow()
    );
    let mut input_path = input!("{}", prompt_text);

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

    let prompt_text = format!(
        "{} Enter root folder path (contains extracted folder) {} [last used: {}]: ",
        "┌─".bright_cyan(),
        "─┐".bright_cyan(),
        last_input_path.bright_yellow()
    );
    let mut input_path = input!("{}", prompt_text);

    if input_path.is_empty() {
        input_path = last_input_path;
    } else if input_path != last_input_path {
        user_preference.save_last_used_root_folder_path(input_path.clone());
    }

    input_path
}

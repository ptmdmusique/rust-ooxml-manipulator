use crate::utils::types::FilePathInfo;
use prompted::input;

pub fn get_file_path_from_input() -> FilePathInfo {
    let input_path = input!("Enter the path of the file: ");
    FilePathInfo::new(input_path)
}

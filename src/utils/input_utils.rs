use crate::utils::{
    types::FilePathInfo,
    zip_utils::{extract_zip_wrapper, rezip_folder_wrapper},
};
use prompted::input;

pub fn get_file_path_from_input() -> FilePathInfo {
    let input_path = input!("Enter the path of the file: ");
    FilePathInfo::new(input_path)
}

pub fn get_fn_to_call() -> Result<fn(), &'static str> {
    println!("Here is the list of features:");
    println!("\t1. Extract");
    println!("\t2. Rezip");

    // TODO: add more features
    // println!("\t3. Summarize");

    let input_feature = input!("Enter the feature number you want to use: ");

    match input_feature.as_str() {
        "1" => Ok(extract_zip_wrapper as fn()),
        "2" => Ok(rezip_folder_wrapper as fn()),
        _ => Err("Invalid feature"),
    }
}

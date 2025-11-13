use std::path::Path;

use prompted::input;

use crate::utils::{
    files::get_output_folder,
    types::{FilePathInfo, ZipFolder},
    zip_utils::main::extract_zip,
};

pub fn ensure_ooxml_exist(file_path_info: &FilePathInfo) -> Result<(String, String), &'static str> {
    let ZipFolder {
        extracted_folder,
        root_folder,
        ..
    } = get_output_folder(file_path_info);

    let output_path = Path::new(&extracted_folder);

    if !output_path.exists() {
        let do_extract = input!(
            "The extracted folder {} does not exist. \n\tDo you want to extract it? (y/n - default: n): ",
            &extracted_folder
        );

        if do_extract.to_lowercase() == "y" {
            let extract_result = extract_zip(file_path_info);
            if extract_result.is_err() {
                return Err(extract_result.err().unwrap());
            }
        } else {
            return Err("The extracted folder does not exist");
        }
    }

    if !output_path.is_dir() {
        return Err("The extracted folder is not a directory");
    }

    Ok((extracted_folder, root_folder))
}

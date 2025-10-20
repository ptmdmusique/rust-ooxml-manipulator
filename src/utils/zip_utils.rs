use crate::utils::{
    print_utils::{print_error_with_panic, print_fn_progress},
    types::FilePathInfo,
    types::ZipFolder,
};
use colored::Colorize;
use prompted::input;
use std::{
    fs::{create_dir_all, remove_dir_all},
    path::Path,
};
use zip_extensions::{zip_create_from_directory, zip_extract};

/// Extract the zip file into a new folder
pub fn extract_zip(file_path_info: &FilePathInfo) {
    println!("\n");
    let fn_name = "Extract zip";
    print_fn_progress(fn_name, "Extracting zip...");

    let FilePathInfo { full_file_path, .. } = file_path_info;

    let ZipFolder {
        root_folder,
        extracted_folder,
    } = get_output_folder(file_path_info);

    // Check if the output folder already exists
    let output_path = Path::new(&root_folder);
    if output_path.exists() {
        println!(
            "{}",
            "The output folder already exists... skipping folder creation".yellow()
        );

        let override_input = input!("Override? (y/n - default: n): ");
        if override_input.to_lowercase() == "y" {
            match remove_dir_all(output_path) {
                Ok(_) => println!(
                    "{}",
                    "Output folder removed successfully, creating new one...".blue()
                ),
                Err(e) => print_error_with_panic(&format!(
                    "Failed to remove the output folder: {}",
                    e.to_string()
                )),
            }
        } else {
            println!("{}", "Operation cancelled".yellow());
            return;
        }
    }

    println!("Creating the output folder...");
    // Create the output folder
    let create_result = create_dir_all(&extracted_folder);
    if create_result.is_err() {
        print_error_with_panic(&format!(
            "Failed to create the output folder {}: {}",
            extracted_folder,
            create_result.err().unwrap().to_string()
        ));
    }

    // Extract the zip
    match zip_extract(
        &Path::new(full_file_path).to_path_buf(),
        &Path::new(&extracted_folder).to_path_buf(),
    ) {
        Ok(_) => println!("{}", "Zip extracted successfully".green()),
        Err(e) => print_error_with_panic(&format!("Failed to extract the zip: {}", e.to_string())),
    }

    print_fn_progress(
        fn_name,
        "Zip extracted successfully!\n".green().to_string().as_str(),
    );
}

/// Rezip an extracted folder into a Word file
pub fn rezip_folder(file_path_info: &FilePathInfo) {
    println!("\n");
    let fn_name = "Rezip folder";
    print_fn_progress(fn_name, "Rezipping folder...");

    let FilePathInfo { full_file_path, .. } = file_path_info;

    let ZipFolder {
        extracted_folder, ..
    } = get_output_folder(file_path_info);

    let folder_path = Path::new(&extracted_folder);
    if !folder_path.is_dir() {
        print_error_with_panic(&format!(
            "The folder path is not a directory: {}",
            folder_path.to_string_lossy()
        ));
    }

    println!(
        "Creating the zip file from {} to {}...",
        extracted_folder, full_file_path
    );

    match zip_create_from_directory(
        &Path::new(full_file_path).to_path_buf(),
        &folder_path.to_path_buf(),
    ) {
        Ok(_) => println!("{}", "Zip file created successfully".green()),
        Err(e) => print_error_with_panic(&format!(
            "Failed to create the zip file from folder {:?}: {}",
            folder_path,
            e.to_string()
        )),
    }

    print_fn_progress(
        fn_name,
        "Zip file created successfully!"
            .green()
            .to_string()
            .as_str(),
    );
}

fn get_output_folder(file_path_info: &FilePathInfo) -> ZipFolder {
    let FilePathInfo {
        file_name,
        file_path,
        ..
    } = file_path_info;

    let output_folder = format!("{}/{}", file_path, file_name);
    let zip_output_folder = format!("{}/{}", output_folder, "extracted");

    ZipFolder {
        root_folder: output_folder,
        extracted_folder: zip_output_folder,
    }
}

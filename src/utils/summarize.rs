use colored::Colorize;
use prompted::input;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::utils::files::get_file_size_in_kb_from_bytes;
use crate::utils::files::get_output_folder;
use crate::utils::files::is_image_extension;
use crate::utils::files::visit_dirs;
use crate::utils::files::write_struct_to_json;
use crate::utils::print_utils::print_error_with_panic;
use crate::utils::types::FilePathInfo;
use crate::utils::types::ZipFolder;
use crate::utils::zip_utils::extract_zip;
use crate::utils::{input_utils::get_file_path_from_input, print_utils::print_fn_progress};

#[derive(Serialize, Deserialize)]
struct FileInfo {
    file_name: String,
    file_path: String,
    file_size_in_kb: u64,
}

#[derive(Serialize, Deserialize)]
struct MediaInfo {
    file_count: u32,
    total_size_in_kb: u64,
    files: Vec<FileInfo>,
}

#[derive(Serialize, Deserialize)]
struct SummarizeResult {
    basic_info: FileInfo,
    file_count: u32,
    media_info: MediaInfo,
}

/// Summarize the structure of the Word file
pub fn summarize_wrapper() {
    println!("\n");
    let fn_name = "Summarize";
    print_fn_progress(fn_name, "Summarizing...");

    let file_path_info = get_file_path_from_input();
    file_path_info.print_info();
    let ZipFolder {
        extracted_folder, ..
    } = get_output_folder(&file_path_info);

    let summarize_result = summarize(&extracted_folder, &file_path_info);
    if summarize_result.is_err() {
        print_error_with_panic(&format!(
            "Failed to summarize: {}",
            summarize_result.err().unwrap()
        ));
    }

    let output_path = format!("{}/summary.json", extracted_folder);

    let write_result = write_struct_to_json(&summarize_result.unwrap(), &output_path);
    if write_result.is_err() {
        print_error_with_panic(&format!(
            "Failed to write the summarize result to the file: {}",
            write_result.err().unwrap()
        ));
    }

    println!("Summary file: {}", output_path);
    println!("{}", "Summarization completed successfully!".green());
}

/// Recursively traverse the extracted folder and count the number of files, images, custom XMLs, etc
fn summarize(
    extracted_folder: &String,
    file_path_info: &FilePathInfo,
) -> Result<SummarizeResult, &'static str> {
    let output_path = Path::new(&extracted_folder);

    // * First make sure the folder exists
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

    // * Continue with the summarization
    let mut file_count = 0;
    let mut media_info = MediaInfo {
        file_count: 0,
        total_size_in_kb: 0,
        files: Vec::new(),
    };

    let visit_result = visit_dirs(output_path, &mut |entry| {
        let path = entry.path();
        let FilePathInfo {
            file_name,
            file_path,
            file_size,
            file_extension,
            ..
        } = FilePathInfo::new(path.to_string_lossy().to_string());

        let file_size_in_kb = get_file_size_in_kb_from_bytes(file_size);
        file_count += 1;

        if file_extension.is_some() && is_image_extension(&file_extension.unwrap()) {
            media_info.file_count += 1;
            media_info.total_size_in_kb += file_size_in_kb;
            media_info.files.push(FileInfo {
                file_name,
                file_path,
                file_size_in_kb,
            });
        }
    });

    if visit_result.is_err() {
        return Err("Failed to visit the directory");
    }

    Ok(SummarizeResult {
        basic_info: FileInfo {
            file_name: file_path_info.file_name.clone(),
            file_path: file_path_info.file_path.clone(),
            file_size_in_kb: get_file_size_in_kb_from_bytes(file_path_info.file_size),
        },
        file_count,
        media_info,
    })
}

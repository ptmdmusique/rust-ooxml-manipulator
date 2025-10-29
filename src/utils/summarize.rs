use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::utils::ensure_ooxml_exist::ensure_ooxml_exist;
use crate::utils::files::get_file_size_in_kb_from_bytes;
use crate::utils::files::is_file_custom_xml;
use crate::utils::files::is_image_extension;
use crate::utils::files::visit_dirs;
use crate::utils::files::write_struct_to_json;
use crate::utils::print_utils::print_error_with_panic;
use crate::utils::types::FilePathInfo;
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
struct SummarizeData {
    basic_info: FileInfo,
    file_count: u32,
    media_info: MediaInfo,
    custom_xml_files: Vec<FileInfo>,
}

/// Summarize the structure of the Word file
pub fn summarize_wrapper() {
    println!("\n");
    let fn_name = "Summarize";
    print_fn_progress(fn_name, "Summarizing...");

    let file_path_info = get_file_path_from_input();
    file_path_info.print_info();

    let summarize_result = summarize(&file_path_info);
    if summarize_result.is_err() {
        print_error_with_panic(&format!(
            "Failed to summarize: {}",
            summarize_result.err().unwrap()
        ));
    }

    let (summarize_data, root_folder) = summarize_result.unwrap();

    let output_path = format!("{}/summary.json", root_folder);

    let write_result = write_struct_to_json(&summarize_data, &output_path);
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
fn summarize(file_path_info: &FilePathInfo) -> Result<(SummarizeData, String), &'static str> {
    let (extracted_folder, root_folder) = ensure_ooxml_exist(file_path_info)?;

    // * Continue with the summarization
    let mut file_count = 0;
    let mut media_info = MediaInfo {
        file_count: 0,
        total_size_in_kb: 0,
        files: Vec::new(),
    };
    let mut custom_xml_files: Vec<FileInfo> = Vec::new();

    let output_path = Path::new(&extracted_folder);
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

        if is_file_custom_xml(&file_name) {
            custom_xml_files.push(FileInfo {
                file_name,
                file_path,
                file_size_in_kb,
            });
        } else if file_extension.is_some() && is_image_extension(&file_extension.unwrap()) {
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

    Ok((
        SummarizeData {
            basic_info: FileInfo {
                file_name: file_path_info.file_name.clone(),
                file_path: file_path_info.file_path.clone(),
                file_size_in_kb: get_file_size_in_kb_from_bytes(file_path_info.file_size),
            },
            file_count,
            media_info,
            custom_xml_files,
        },
        root_folder,
    ))
}

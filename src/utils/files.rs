use crate::utils::types::FilePathInfo;
use crate::utils::types::ZipFolder;
use serde::Serialize;
use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;

/// Get the output folder for the zip file extraction including the root folder and the folder that the Word file will be extracted to
pub fn get_output_folder(file_path_info: &FilePathInfo) -> ZipFolder {
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

/// https://doc.rust-lang.org/nightly/std/fs/fn.read_dir.html#examples
pub fn visit_dirs(dir: &Path, file_callback: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, file_callback)?;
            } else {
                file_callback(&entry);
            }
        }
    }
    Ok(())
}

/// Check whether the given extension is an image extension
pub fn is_image_extension(extension: &str) -> bool {
    matches!(
        extension,
        "jpg" | "jpeg" | "png" | "gif" | "bmp" | "tiff" | "ico"
    )
}

pub fn write_struct_to_json<T: Serialize>(data: &T, file_path: &str) -> Result<(), std::io::Error> {
    let json = serde_json::to_string_pretty(data)?;
    fs::write(file_path, json)
}

pub fn get_file_size_in_kb_from_bytes(file_size: u64) -> u64 {
    file_size / 1024
}

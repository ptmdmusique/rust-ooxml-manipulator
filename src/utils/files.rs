use crate::utils::types::FilePathInfo;
use crate::utils::types::ZipFolder;
use fancy_regex::Regex;
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

pub fn get_file_size_in_kb_from_bytes(file_size: u64) -> f64 {
    let kb = file_size as f64 / 1024.0;
    (kb * 100.0).round() / 100.0
}

/// Check whether the given file name is a custom XML file
/// An example of a custom XML file is `item1.xml`
/// This will not return `true` for `itemProps1.xml`
pub fn is_file_custom_xml(file_name: &str) -> bool {
    let pattern = r"^item\d+\.xml$";
    let re = Regex::new(pattern).unwrap();

    re.is_match(file_name).unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_file_custom_xml() {
        // Valid custom XML files
        assert!(is_file_custom_xml("item1.xml"));
        assert!(is_file_custom_xml("item123.xml"));
        assert!(is_file_custom_xml("item0.xml"));
        assert!(is_file_custom_xml("item999.xml"));

        // Invalid custom XML files
        assert!(!is_file_custom_xml("item.xml")); // missing number
        assert!(!is_file_custom_xml("item1a.xml")); // contains letter
        assert!(!is_file_custom_xml("item1")); // missing .xml extension
        assert!(!is_file_custom_xml("item1.XML")); // wrong case
        assert!(!is_file_custom_xml("item1.txt")); // wrong extension
        assert!(!is_file_custom_xml("custom.xml")); // wrong prefix
        assert!(!is_file_custom_xml("itemProps1.xml")); // wrong prefix (as mentioned in doc)
        assert!(!is_file_custom_xml("")); // empty string
        assert!(!is_file_custom_xml("item.xml.bak")); // extra extension
    }

    #[test]
    fn test_get_file_size_in_kb_from_bytes() {
        // Test exact conversions
        assert_eq!(get_file_size_in_kb_from_bytes(0), 0.0);
        assert_eq!(get_file_size_in_kb_from_bytes(1024), 1.0);
        assert_eq!(get_file_size_in_kb_from_bytes(2048), 2.0);
        assert_eq!(get_file_size_in_kb_from_bytes(5120), 5.0);

        // Test with remainders (should round to 2 decimal places)
        assert_eq!(get_file_size_in_kb_from_bytes(1023), 1.0); // 0.999KB -> 1.0KB
        assert_eq!(get_file_size_in_kb_from_bytes(1536), 1.5); // 1.5KB -> 1.5KB
        assert_eq!(get_file_size_in_kb_from_bytes(2047), 2.0); // 1.999KB -> 2.0KB
        assert_eq!(get_file_size_in_kb_from_bytes(2049), 2.0); // 2.001KB -> 2.0KB

        // Test larger values
        assert_eq!(get_file_size_in_kb_from_bytes(1024 * 1024), 1024.0); // 1MB
        assert_eq!(get_file_size_in_kb_from_bytes(1024 * 1024 * 5), 5120.0); // 5MB

        // Test rounding to 2 decimal places
        assert_eq!(get_file_size_in_kb_from_bytes(1536), 1.5); // 1536 bytes = 1.5 KB
        assert_eq!(get_file_size_in_kb_from_bytes(2560), 2.5); // 2560 bytes = 2.5 KB
        assert_eq!(get_file_size_in_kb_from_bytes(1234), 1.21); // 1234 bytes = 1.205... KB -> 1.21 KB (rounded)
    }

    #[test]
    fn test_is_image_extension() {
        // Valid image extensions (case sensitive)
        assert!(is_image_extension("jpg"));
        assert!(is_image_extension("jpeg"));
        assert!(is_image_extension("png"));
        assert!(is_image_extension("gif"));
        assert!(is_image_extension("bmp"));
        assert!(is_image_extension("tiff"));
        assert!(is_image_extension("ico"));

        // Invalid extensions
        assert!(!is_image_extension("JPG")); // wrong case
        assert!(!is_image_extension("PNG")); // wrong case
        assert!(!is_image_extension("webp")); // not supported
        assert!(!is_image_extension("svg")); // not supported
        assert!(!is_image_extension("pdf")); // not an image
        assert!(!is_image_extension("txt")); // not an image
        assert!(!is_image_extension("")); // empty string
        assert!(!is_image_extension("jpegx")); // invalid variant
        assert!(!is_image_extension("png.")); // with dot
        assert!(!is_image_extension(".png")); // with leading dot
    }
}

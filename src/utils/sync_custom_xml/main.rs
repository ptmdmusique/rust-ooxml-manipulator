use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::utils::{
    analyze_custom_xml::main::{CustomXmlFile, CustomXmlInfo, parse_custom_xml_content_for_tag},
    files::read_struct_from_json,
    input_utils::get_path_from_input::get_extracted_root_folder_path,
    print_utils::{get_error_message, print_error_with_panic, print_fn_progress},
    types::{CUSTOM_XML_FILE_NAME, EXTRACTED_FOLDER_NAME, UserPreference},
};

/// Sync the customXml.json back to the customXml folder
pub fn sync_custom_xml_wrapper(user_preference: &mut UserPreference) {
    println!("\n");
    let fn_name = "Sync customXML";
    print_fn_progress(fn_name, "Syncing customXML...");

    let root_folder = get_extracted_root_folder_path(user_preference);
    println!("Root folder: {}", root_folder);

    let sync_result = sync_custom_xml(&root_folder);
    if let Err(e) = sync_result {
        print_error_with_panic(e);
    }

    println!("{}", "Syncing customXML completed successfully!".green());
}

/// Sync the customXml.json back to the customXml folder inside the extracted folder
///
/// ! Note that this will override the content of the customXml folder
///
/// This also doesn't support syncing the item props and rels files
pub fn sync_custom_xml(root_folder: &str) -> Result<(), &'static str> {
    // * Read the customXml.json file
    let custom_xml_json_path = format!("{}/{}", root_folder, CUSTOM_XML_FILE_NAME);
    if !Path::new(&custom_xml_json_path).exists() {
        return Err("customXml.json file not found in the root folder");
    }

    let custom_xml_files: Vec<CustomXmlFile> = match read_struct_from_json(&custom_xml_json_path) {
        Ok(files) => files,
        Err(_) => {
            return Err("Failed to read customXml.json");
        }
    };

    if custom_xml_files.is_empty() {
        return Err("No custom XML files found in customXml.json");
    }

    // * Get the customXml folder path inside extracted folder
    let mut custom_xml_folder = format!("{}/{}/customXml", root_folder, EXTRACTED_FOLDER_NAME);
    if !Path::new(&custom_xml_folder).exists() {
        // Creating a custom xml via Word Desktop/Online might create a folder named customXML instead of customXml
        // #microsuck
        custom_xml_folder = format!("{}/{}/customXML", root_folder, EXTRACTED_FOLDER_NAME);
    }

    // TODO: implement this feature later
    // We'll need to also add the item props and rels files
    let custom_xml_folder_path = Path::new(&custom_xml_folder);
    if !custom_xml_folder_path.exists() {
        return Err("customXml folder not found in the extracted folder");
    }

    // * Sync each custom XML file
    let mut synced_count = 0;
    let mut skipped_count = 0;
    for custom_xml_file in &custom_xml_files {
        let file_name = &custom_xml_file.file_info.file_name_with_extension;
        let file_path = format!("{}/{}", custom_xml_folder, file_name);

        let should_update = should_update_file(&file_path, &custom_xml_file.custom_xml_info);
        if should_update {
            // Reconstruct the XML content from the JSON data
            let xml_content = reconstruct_xml_from_json(custom_xml_file);

            // Write the XML content to the file
            match fs::write(&file_path, xml_content) {
                Ok(_) => {
                    synced_count += 1;
                    println!("Synced: {}", file_name);
                }
                Err(e) => {
                    println!(
                        "{}",
                        get_error_message(&format!("Failed to write {}: {}", file_name, e))
                    );
                }
            }
        } else {
            skipped_count += 1;
            println!("Skipped (unchanged): {}", file_name);
        }
    }

    println!(
        "Successfully synced {} out of {} custom XML files ({} skipped as unchanged)",
        synced_count,
        custom_xml_files.len(),
        skipped_count
    );
    Ok(())
}

/// Determine if a file should be updated by comparing the current file content with the expected CustomXmlInfo
///
/// This only return `true` if the content changed.
/// If the file is not found or the content can't be parsed, this will return `false` as well as we'll have to also check whether the rel files exist or not
/// TODO: implement this feature later
pub fn should_update_file(file_path: &str, expected_info: &CustomXmlInfo) -> bool {
    if !Path::new(file_path).exists() {
        println!("{}", format!("File not found: {}", file_path).yellow());
        return false;
    }

    match fs::read_to_string(file_path) {
        Ok(current_content) => {
            // Parse the current XML content
            match parse_custom_xml_content_for_tag(&current_content) {
                Ok(current_info) => {
                    // Compare with the expected version
                    current_info != *expected_info
                }
                Err(error) => {
                    // If parsing fails, we should update
                    println!(
                        "{}",
                        get_error_message(&format!("Failed to parse {}: {}", file_path, error))
                    );
                    false
                }
            }
        }
        Err(error) => {
            // If reading fails, we should update
            println!(
                "{}",
                get_error_message(&format!("Failed to read {}: {}", file_path, error))
            );
            false
        }
    }
}

/// Reconstruct the XML content from the CustomXmlFile struct
fn reconstruct_xml_from_json(custom_xml_file: &CustomXmlFile) -> String {
    let custom_xml_info = &custom_xml_file.custom_xml_info;
    let tag = &custom_xml_info.tag;
    let attributes = &custom_xml_info.attributes;
    let json_content = &custom_xml_info.json_content;

    // * Build the opening tag
    let mut xml = String::new();
    xml.push('<');
    xml.push_str(tag);

    // * Add attributes if they exist
    if let Some(attrs) = attributes.as_ref()
        && let Some(attrs_obj) = attrs.as_object()
    {
        for (key, value) in attrs_obj {
            if let Some(value_str) = value.as_str() {
                let attribute_string = format!(" {}=\"{}\"", key, value_str);
                xml.push_str(attribute_string.as_str());
            }
        }
    }

    xml.push('>');

    // * Add the JSON content (no pretty printing)
    let json_content_str = serde_json::to_string(json_content).unwrap();
    xml.push_str(&json_content_str);

    // * Add the closing tag
    xml.push_str("</");
    xml.push_str(tag);
    xml.push('>');

    xml
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::analyze_custom_xml::main::CustomXmlInfo;
    use crate::utils::types::FileInfo;
    use serde_json::json;

    #[test]
    fn test_reconstruct_xml_from_json_basic() {
        let custom_xml_file = CustomXmlFile {
            file_info: FileInfo {
                file_name_with_extension: "item1.xml".to_string(),
                full_file_path: "test/item1.xml".to_string(),
                file_size_in_kb: 1.0,
            },
            custom_xml_info: CustomXmlInfo {
                tag: "rootTag".to_string(),
                attributes: Some(json!({
                    "attr1": "value1",
                    "attr2": "value2"
                })),
                json_content: json!({"key": "value"}),
            },
        };

        let result = reconstruct_xml_from_json(&custom_xml_file);

        assert!(result.starts_with("<rootTag"));
        assert!(result.contains("attr1=\"value1\""));
        assert!(result.contains("attr2=\"value2\""));
        assert!(result.contains("{\"key\":\"value\"}"));
        assert!(result.ends_with("</rootTag>"));
    }

    #[test]
    fn test_reconstruct_xml_from_json_no_attributes() {
        let custom_xml_file = CustomXmlFile {
            file_info: FileInfo {
                file_name_with_extension: "item1.xml".to_string(),
                full_file_path: "test/item1.xml".to_string(),
                file_size_in_kb: 1.0,
            },
            custom_xml_info: CustomXmlInfo {
                tag: "simpleTag".to_string(),
                attributes: None,
                json_content: json!({"data": 123}),
            },
        };

        let result = reconstruct_xml_from_json(&custom_xml_file);

        assert_eq!(result, "<simpleTag>{\"data\":123}</simpleTag>");
    }

    #[test]
    fn test_reconstruct_xml_from_json_empty_json_content() {
        let custom_xml_file = CustomXmlFile {
            file_info: FileInfo {
                file_name_with_extension: "item1.xml".to_string(),
                full_file_path: "test/item1.xml".to_string(),
                file_size_in_kb: 1.0,
            },
            custom_xml_info: CustomXmlInfo {
                tag: "emptyTag".to_string(),
                attributes: Some(json!({"id": "123"})),
                json_content: json!({}),
            },
        };

        let result = reconstruct_xml_from_json(&custom_xml_file);

        assert!(result.starts_with("<emptyTag"));
        assert!(result.contains("id=\"123\""));
        assert!(result.contains("{}"));
        assert!(result.ends_with("</emptyTag>"));
    }

    #[test]
    fn test_reconstruct_xml_from_json_complex_json() {
        let custom_xml_file = CustomXmlFile {
            file_info: FileInfo {
                file_name_with_extension: "item1.xml".to_string(),
                full_file_path: "test/item1.xml".to_string(),
                file_size_in_kb: 1.0,
            },
            custom_xml_info: CustomXmlInfo {
                tag: "dataTag".to_string(),
                attributes: Some(json!({"version": "1.0"})),
                json_content: json!({
                    "nested": {
                        "key": "value",
                        "number": 42
                    },
                    "array": [1, 2, 3]
                }),
            },
        };

        let result = reconstruct_xml_from_json(&custom_xml_file);

        assert!(result.starts_with("<dataTag"));
        assert!(result.contains("version=\"1.0\""));
        assert!(result.contains("nested"));
        assert!(result.contains("array"));
        assert!(result.ends_with("</dataTag>"));
    }

    #[test]
    fn test_reconstruct_xml_from_json_single_attribute() {
        let custom_xml_file = CustomXmlFile {
            file_info: FileInfo {
                file_name_with_extension: "item1.xml".to_string(),
                full_file_path: "test/item1.xml".to_string(),
                file_size_in_kb: 1.0,
            },
            custom_xml_info: CustomXmlInfo {
                tag: "singleAttr".to_string(),
                attributes: Some(json!({"name": "test"})),
                json_content: json!("simple string"),
            },
        };

        let result = reconstruct_xml_from_json(&custom_xml_file);

        assert!(result.starts_with("<singleAttr"));
        assert!(result.contains("name=\"test\""));
        assert!(result.contains("\"simple string\""));
        assert!(result.ends_with("</singleAttr>"));
    }

    #[test]
    fn test_should_update_file_when_file_does_not_exist() {
        let expected_info = CustomXmlInfo {
            tag: "testTag".to_string(),
            attributes: None,
            json_content: json!({"key": "value"}),
        };

        // Use a non-existent file path
        let file_path = "/tmp/nonexistent_file_12345.xml";
        let result = should_update_file(file_path, &expected_info);

        assert!(!result, "Should return false when file doesn't exist");
    }

    #[test]
    fn test_should_update_file_when_content_matches() {
        use std::fs;
        use tempfile::NamedTempFile;

        let expected_info = CustomXmlInfo {
            tag: "testTag".to_string(),
            attributes: Some(json!({"attr1": "value1"})),
            json_content: json!({"key": "value"}),
        };

        // Create a temporary file with matching content
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Write XML content that matches the expected_info
        let xml_content = r#"<testTag attr1="value1">{"key":"value"}</testTag>"#;
        fs::write(file_path, xml_content).unwrap();

        let result = should_update_file(file_path, &expected_info);

        assert!(!result, "Should return false when content matches");
    }

    #[test]
    fn test_should_update_file_when_content_differs() {
        use std::fs;
        use tempfile::NamedTempFile;

        let expected_info = CustomXmlInfo {
            tag: "testTag".to_string(),
            attributes: Some(json!({"attr1": "value1"})),
            json_content: json!({"key": "different_value"}),
        };

        // Create a temporary file with different content
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Write XML content that differs from expected_info
        let xml_content = r#"<testTag attr1="value1">{"key":"value"}</testTag>"#;
        fs::write(file_path, xml_content).unwrap();

        let result = should_update_file(file_path, &expected_info);

        assert!(result, "Should return true when content differs");
    }

    #[test]
    fn test_should_update_file_when_tag_differs() {
        use std::fs;
        use tempfile::NamedTempFile;

        let expected_info = CustomXmlInfo {
            tag: "testTag".to_string(),
            attributes: None,
            json_content: json!({"key": "value"}),
        };

        // Create a temporary file with different tag
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Write XML content with different tag
        let xml_content = r#"<differentTag>{"key":"value"}</differentTag>"#;
        fs::write(file_path, xml_content).unwrap();

        let result = should_update_file(file_path, &expected_info);

        assert!(result, "Should return true when tag differs");
    }

    #[test]
    fn test_should_update_file_when_attributes_differ() {
        use std::fs;
        use tempfile::NamedTempFile;

        let expected_info = CustomXmlInfo {
            tag: "testTag".to_string(),
            attributes: Some(json!({"attr1": "value1"})),
            json_content: json!({"key": "value"}),
        };

        // Create a temporary file with different attributes
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Write XML content with different attributes
        let xml_content = r#"<testTag attr1="different_value">{"key":"value"}</testTag>"#;
        fs::write(file_path, xml_content).unwrap();

        let result = should_update_file(file_path, &expected_info);

        assert!(result, "Should return true when attributes differ");
    }

    #[test]
    fn test_should_update_file_when_file_is_invalid_xml() {
        use std::fs;
        use tempfile::NamedTempFile;

        let expected_info = CustomXmlInfo {
            tag: "testTag".to_string(),
            attributes: None,
            json_content: json!({"key": "value"}),
        };

        // Create a temporary file with invalid XML
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Write invalid XML content
        let xml_content = "This is not valid XML content";
        fs::write(file_path, xml_content).unwrap();

        let result = should_update_file(file_path, &expected_info);

        assert!(
            !result,
            "Should return false when file contains invalid XML"
        );
    }

    #[test]
    fn test_should_update_file_when_attributes_missing_in_file() {
        use std::fs;
        use tempfile::NamedTempFile;

        let expected_info = CustomXmlInfo {
            tag: "testTag".to_string(),
            attributes: Some(json!({"attr1": "value1"})),
            json_content: json!({"key": "value"}),
        };

        // Create a temporary file without attributes
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Write XML content without attributes
        let xml_content = r#"<testTag>{"key":"value"}</testTag>"#;
        fs::write(file_path, xml_content).unwrap();

        let result = should_update_file(file_path, &expected_info);

        assert!(
            result,
            "Should return true when attributes are missing in file"
        );
    }

    #[test]
    fn test_should_update_file_when_attributes_missing_in_expected() {
        use std::fs;
        use tempfile::NamedTempFile;

        let expected_info = CustomXmlInfo {
            tag: "testTag".to_string(),
            attributes: None,
            json_content: json!({"key": "value"}),
        };

        // Create a temporary file with attributes
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path().to_str().unwrap();

        // Write XML content with attributes
        let xml_content = r#"<testTag attr1="value1">{"key":"value"}</testTag>"#;
        fs::write(file_path, xml_content).unwrap();

        let result = should_update_file(file_path, &expected_info);

        assert!(
            result,
            "Should return true when attributes are present in file but not expected"
        );
    }
}

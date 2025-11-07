use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::utils::{
    analyze_custom_xml::CustomXmlFile,
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
    for custom_xml_file in &custom_xml_files {
        let file_name = &custom_xml_file.file_info.file_name_with_extension;
        let file_path = format!("{}/{}", custom_xml_folder, file_name);

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
    }

    println!(
        "Successfully synced {} out of {} custom XML files",
        synced_count,
        custom_xml_files.len()
    );
    Ok(())
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
    use crate::utils::analyze_custom_xml::CustomXmlInfo;
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
}

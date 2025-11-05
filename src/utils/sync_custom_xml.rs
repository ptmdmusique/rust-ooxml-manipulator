use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::utils::{
    analyze_custom_xml::CustomXmlFile,
    files::read_struct_from_json,
    input_utils::get_path_from_input::get_folder_path_from_input_for_sync,
    print_utils::{get_error_message, print_error_with_panic, print_fn_progress},
    types::{CUSTOM_XML_FILE_NAME, EXTRACTED_FOLDER_NAME, UserPreference},
};

/// Sync the customXML.json back to the customXml folder
pub fn sync_custom_xml_wrapper(user_preference: &mut UserPreference) {
    println!("\n");
    let fn_name = "Sync customXML";
    print_fn_progress(fn_name, "Syncing customXML...");

    let root_folder = get_folder_path_from_input_for_sync(user_preference);
    println!("Root folder: {}", root_folder);

    let sync_result = sync_custom_xml(&root_folder);
    if let Err(e) = sync_result {
        print_error_with_panic(e);
    }

    println!("{}", "Syncing customXML completed successfully!".green());
}

/// Sync the customXML.json back to the customXml folder inside the extracted folder
/// This will also create the customXml folder if it doesn't exist
/// ! Note that this will override the content of the customXml folder
fn sync_custom_xml(root_folder: &str) -> Result<(), &'static str> {
    // * Read the customXML.json file
    let custom_xml_json_path = format!("{}/{}", root_folder, CUSTOM_XML_FILE_NAME);
    if !Path::new(&custom_xml_json_path).exists() {
        return Err("customXML.json file not found in the root folder");
    }

    let custom_xml_files: Vec<CustomXmlFile> = match read_struct_from_json(&custom_xml_json_path) {
        Ok(files) => files,
        Err(_) => {
            return Err("Failed to read customXML.json");
        }
    };

    if custom_xml_files.is_empty() {
        println!("No custom XML files found in customXML.json");
        return Ok(());
    }

    // * Get the customXml folder path inside extracted folder
    let mut custom_xml_folder = format!("{}/{}/customXml", root_folder, EXTRACTED_FOLDER_NAME);
    if !Path::new(&custom_xml_folder).exists() {
        // Creating a custom xml via Word Desktop/Online might create a folder named customXML instead of customXml
        // #microsuck
        custom_xml_folder = format!("{}/{}/customXML", root_folder, EXTRACTED_FOLDER_NAME);
    }

    let custom_xml_folder_path = Path::new(&custom_xml_folder);
    if !custom_xml_folder_path.exists() {
        // Create the customXml folder if it doesn't exist
        fs::create_dir_all(custom_xml_folder_path)
            .map_err(|_| "Failed to create customXml folder")?;
    }

    // * Sync each custom XML file
    let mut synced_count = 0;
    for custom_xml_file in &custom_xml_files {
        let file_name = &custom_xml_file.file_info.file_name_with_extension;
        let file_path = format!("{}/{}", custom_xml_folder, file_name);

        // Reconstruct the XML content from the JSON data
        let xml_content = reconstruct_xml_from_json(&custom_xml_file);

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
    if let Some(attrs) = attributes.as_ref() {
        if let Some(attrs_obj) = attrs.as_object() {
            for (key, value) in attrs_obj {
                if let Some(value_str) = value.as_str() {
                    let attribute_string = format!(" {}=\"{}\"", key, value_str);
                    xml.push_str(attribute_string.as_str());
                }
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

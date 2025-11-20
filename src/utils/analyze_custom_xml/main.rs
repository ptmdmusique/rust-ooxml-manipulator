use colored::Colorize;
use fancy_regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, fs::read_to_string, path::Path};

use crate::utils::{
    ensure_ooxml_exist::ensure_ooxml_exist,
    files::{get_file_size_in_kb_from_bytes, is_file_custom_xml, visit_dirs, write_struct_to_json},
    input_utils::get_path_from_input::get_file_path_from_input,
    print_utils::{get_error_message, print_error_with_panic, print_fn_progress},
    types::{CUSTOM_XML_FILE_NAME, FileInfo, FilePathInfo, UserPreference},
};

#[derive(Serialize, Deserialize)]
pub struct CustomXmlFile {
    pub file_info: FileInfo,
    pub custom_xml_info: CustomXmlInfo,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustomXmlInfo {
    pub tag: String,
    pub attributes: Option<serde_json::Value>,
    pub json_content: serde_json::Value,
}

/// Analyze the custom XMLs in the extracted folder
/// Note that this will only support custom XMLs in this format
/// <someTag attribute1="value1" attribute2="value2">
///   { jsonKey1:"value1" jsonKey2:"value2" }
/// </someTag>
/// And the result will be a json object like { "someTag": { "attribute1": "value1", "attribute2": "value2" } }
///
/// This doesn't support nested tags or multiple tags in the same file.
pub fn analyze_custom_xml_wrapper(user_preference: &mut UserPreference) {
    println!("\n");
    let fn_name = "Analyze customXML";
    print_fn_progress(fn_name, "Analyzing customXML...");

    let file_path_info = get_file_path_from_input(user_preference);
    file_path_info.print_info();

    let analyze_result = analyze_custom_xml(&file_path_info);
    if let Err(e) = analyze_result {
        print_error_with_panic(e);
    }

    let (custom_xml_infos, root_folder) = analyze_result.unwrap();

    let output_path = format!("{}/{}", root_folder, CUSTOM_XML_FILE_NAME);
    let write_result = write_struct_to_json(&custom_xml_infos, &output_path);
    if write_result.is_err() {
        print_error_with_panic(&format!(
            "Failed to write the custom XML info to the file: {}",
            write_result.err().unwrap()
        ));
    }

    println!("Custom XML info file written at path: {}", output_path);
    print_fn_progress(
        fn_name,
        "Analyzing customXML completed successfully!"
            .green()
            .to_string()
            .as_str(),
    );
}

fn analyze_custom_xml(
    file_path_info: &FilePathInfo,
) -> Result<(Vec<CustomXmlFile>, String), &'static str> {
    let (extracted_folder, root_folder) = ensure_ooxml_exist(file_path_info)?;

    let mut custom_xml_files: Vec<CustomXmlFile> = Vec::new();

    let mut parsed_file_count = 0;
    let mut total_file_count = 0;

    // Visit the extracted folder and read the custom XML files
    let output_path = Path::new(&extracted_folder);
    let visit_result = visit_dirs(output_path, &mut |entry| {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();

        let FilePathInfo {
            file_name_with_extension,
            file_size,
            full_file_path,
            ..
        } = FilePathInfo::new(path.to_string_lossy().to_string());

        if is_file_custom_xml(&file_name) {
            total_file_count += 1;

            // Read the content of the custom XML file as a string
            let content = read_to_string(&path);
            if content.is_err() {
                let message = get_error_message(
                    format!(
                        "Failed to read the custom XML file: {}",
                        path.to_string_lossy()
                    )
                    .as_str(),
                );

                println!("{}", message)
            }

            /*
                The content will be in the format of
                <someTag attribute1="value1" attribute2="value2">
                    { jsonKey1:"value1" jsonKey2:"value2" }
                </someTag>
            */
            let content = content.unwrap();
            match parse_custom_xml_content_for_tag(&content) {
                Ok(custom_xml_info) => {
                    custom_xml_files.push(CustomXmlFile {
                        file_info: FileInfo {
                            file_name_with_extension: file_name_with_extension.clone(),
                            full_file_path: full_file_path.clone(),
                            file_size_in_kb: get_file_size_in_kb_from_bytes(file_size),
                        },
                        custom_xml_info,
                    });

                    parsed_file_count += 1;
                    println!(
                        "{}",
                        format!("Parsed custom XML content for file: {}", file_name).green()
                    );
                }
                Err(_) => {
                    println!(
                        "{}",
                        format!("Unsupported custom XML content for file: {}", file_name).yellow()
                    );
                }
            }
        }
    });

    if visit_result.is_err() {
        return Err("Failed to visit the directory");
    } else {
        println!(
            "\n{}\n",
            format!(
                "Parsed {} custom XML files out of {} total files",
                parsed_file_count, total_file_count
            )
            .green()
        );
    }

    Ok((custom_xml_files, root_folder))
}

/// Parse the custom XML content for a tag
/// An example of a custom XML content is:
/// <someTag attribute1="value1" attribute2="value2">
///   { jsonKey1:"value1" jsonKey2:"value2" }
/// </someTag>
/// And the result will be a json object like { "someTag": { "attribute1": "value1", "attribute2": "value2" } }
pub fn parse_custom_xml_content_for_tag(html_content: &str) -> Result<CustomXmlInfo, &'static str> {
    let re = Regex::new(r#"(?s)<(\w+)([^>]*)>(.*?)</\1>"#).unwrap();

    if let Ok(Some(caps)) = re.captures(html_content) {
        let tag = caps.get(1).unwrap().as_str();

        let attributes_as_string = caps.get(2).unwrap().as_str();
        let attributes = parse_attributes(attributes_as_string)?;

        let json_content_as_string = caps.get(3).unwrap().as_str();

        match serde_json::from_str(json_content_as_string) {
            Ok(json_content) => {
                return Ok(CustomXmlInfo {
                    tag: tag.to_string(),
                    attributes,
                    json_content,
                });
            }
            Err(_) => return Err("Unsupported custom XML content"),
        }
    }

    Err("Failed to find the custom XML content")
}

/// Parse the attribute string into a serde_json::Value
/// An example of an attribute string is "attribute1=\"value1\" attribute2=\"value2\""
/// And the result will be a json object like { "attribute1": "value1", "attribute2": "value2" }
fn parse_attributes(attributes: &str) -> Result<Option<serde_json::Value>, &'static str> {
    if attributes.is_empty() {
        return Ok(None);
    }

    let attr_re = Regex::new(r#"(\w+)="([^"]*)""#).unwrap();

    // Collect attributes into a HashMap<String, String>
    let mut attrs = HashMap::new();
    for cap in attr_re.captures_iter(attributes) {
        let cap_result = cap.unwrap();
        attrs.insert(cap_result[1].to_string(), cap_result[2].to_string());
    }

    Ok(Some(json!(attrs)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_attributes_basic() {
        let attrs = "attribute1=\"value1\" attribute2=\"value2\"";
        let result = parse_attributes(attrs).expect("attributes should parse");

        assert_eq!(
            result,
            Some(json!({
                "attribute1": "value1",
                "attribute2": "value2"
            }))
        );
    }

    #[test]
    fn test_parse_attributes_empty_string() {
        let attrs = "";
        let result = parse_attributes(attrs).expect("empty attributes should parse");
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_attributes_ignores_non_matching_segments() {
        let attrs = "attribute1=\"value1\" junk attribute2=\"value2\"";
        let result = parse_attributes(attrs).expect("attributes should parse with junk");
        assert_eq!(
            result,
            Some(json!({
                "attribute1": "value1",
                "attribute2": "value2"
            }))
        );
    }

    #[test]
    fn test_parse_custom_xml_content_for_tag_ok() {
        let content = r#"
            <someTag attribute1="value1" attribute2="value2">{"jsonKey1":"value1","jsonKey2":"value2"}</someTag>
            "#;
        let result = parse_custom_xml_content_for_tag(content);

        let expected_result = CustomXmlInfo {
            tag: "someTag".to_string(),
            attributes: Some(json!({
                "attribute1": "value1",
                "attribute2": "value2"
            })),
            json_content: json!({ "jsonKey1": "value1", "jsonKey2": "value2" }),
        };

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_result);
    }

    #[test]
    fn test_parse_custom_xml_content_for_tag_err_on_mismatch() {
        let content = r#"<someTag>{\"jsonKey1\":\"value1\"}"#; // missing closing tag
        let result = parse_custom_xml_content_for_tag(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_custom_xml_content_for_tag_err_on_unsupported_content() {
        let content = r#"<someTag><moreTag></moreTag></someTag>"#; // invalid JSON
        let result = parse_custom_xml_content_for_tag(content);
        assert!(result.is_err());
    }
}

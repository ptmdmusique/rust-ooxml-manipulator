use crate::utils::{
    input_utils::get_path_from_input::get_extracted_root_folder_path,
    print_utils::{print_error_with_panic, print_fn_progress},
    sync_custom_xml::main::sync_custom_xml,
    types::{CUSTOM_XML_FILE_NAME, EXTRACTED_FOLDER_NAME, UserPreference},
    zip_utils::main::rezip_folder,
};
use colored::Colorize;
use notify::{Event, EventKind, RecursiveMode, Watcher};
use prompted::input;
use std::{collections::HashMap, sync::mpsc, time::Duration};
use std::{
    path::{Path, PathBuf},
    time::Instant,
};

/// Watch for file changes in the root folder
pub fn watch_folder_wrapper(user_preference: &mut UserPreference) {
    println!("\n");
    let fn_name = "Watch folder";
    print_fn_progress(fn_name, "Starting file watcher...");

    let root_folder = get_extracted_root_folder_path(user_preference);
    if !Path::new(&root_folder).exists() {
        print_error_with_panic(&format!("Root folder does not exist: {}", root_folder));
    }

    println!("Watching root folder: {}", root_folder);

    let watch_result = watch_folder(&root_folder);
    if let Err(e) = watch_result {
        print_error_with_panic(&format!("File watcher error: {}", e));
    }
}

/// Watch for file changes and handle them accordingly
fn watch_folder(root_folder: &str) -> Result<(), &'static str> {
    // * Set up the paths
    let root_path = PathBuf::from(root_folder);
    if !root_path.is_dir() {
        return Err("Root folder is not a directory");
    }

    let extracted_folder_path = root_path.join(EXTRACTED_FOLDER_NAME);
    if !extracted_folder_path.is_dir() {
        return Err("Missing extracted folder");
    }

    let custom_xml_json_path = root_path.join(CUSTOM_XML_FILE_NAME);
    let has_custom_xml_file = custom_xml_json_path.is_file();
    if !has_custom_xml_file {
        println!(
            "\n{}",
            format!(
                "customXml file {} not found, ignoring...",
                CUSTOM_XML_FILE_NAME
            )
            .yellow()
        );
    }

    println!("\n{}", "File watcher started. Press Ctrl+C to stop.".blue());
    println!("{}", "Watching for changes in:".yellow());
    if has_custom_xml_file {
        println!("\t- {} (will trigger resync prompt)", CUSTOM_XML_FILE_NAME);
    }
    println!(
        "\t- {} folder (will trigger rezip prompt)",
        EXTRACTED_FOLDER_NAME
    );
    println!();

    let file_name = format!("{}.docx", root_path.file_name().unwrap().to_string_lossy());
    let output_file_path = root_path
        .parent()
        .unwrap()
        .join(file_name)
        .to_string_lossy()
        .to_string();

    // * Set up the watcher
    // https://docs.rs/notify/latest/notify/index.html#examples
    let (tx, rx) = mpsc::channel();
    // Create a watcher with recommended backend to make sure it works on all platforms
    let mut watcher =
        notify::recommended_watcher(tx).map_err(|_| "Failed to create file watcher")?;
    // Watch the root folder recursively
    watcher
        .watch(Path::new(root_folder), RecursiveMode::Recursive)
        .map_err(|_| "Failed to start watching the root folder")?;

    // ! There is a bug where the same event is fired multiple times - unsure why (yet!)
    // This hashmap keep track of the last time the user confirmed the action
    // There might be multiple changes to the same file while we debounce
    //  but that's ok because our re-save function will automatically take the latest change
    let mut debounce_hashmap: HashMap<PathBuf, Instant> = HashMap::new();

    // * Handle events
    // This will run forever until the program is stopped
    for res in rx {
        if res.is_err() {
            println!("{}", format!("Watcher error: {}", res.err().unwrap()).red());
            continue;
        }

        let mut did_execute_action = false;

        let Event { kind, paths, .. } = res.unwrap();
        match kind {
            EventKind::Modify(_) => {
                for path in paths {
                    let last_modified = debounce_hashmap.get(&path);
                    if let Some(last_modified) = last_modified
                        && last_modified.elapsed() < Duration::from_millis(200)
                    {
                        continue;
                    }

                    println!(
                        "{}",
                        format!("{} file data modified (kind: {:?}):", path.display(), kind)
                            .bright_blue()
                    );

                    did_execute_action = true;

                    handle_file_change(
                        &path,
                        &root_path,
                        &extracted_folder_path,
                        &custom_xml_json_path,
                        &output_file_path,
                    );

                    debounce_hashmap.insert(path, Instant::now());
                }
            }
            EventKind::Create(_) => {
                println!("{}", format!("{} files created:", paths.len()).yellow());
                for path in paths {
                    println!("\t- {}", path.display());
                }
            }
            EventKind::Remove(_) => {
                println!("{}", format!("{} files removed:", paths.len()).yellow());
                for path in paths {
                    println!("\t- {}", path.display());
                }
            }
            _ => {
                // ! Other event types are ignored
                println!("{}", format!("Unsupported event type: {:?}", kind).yellow());
            }
        }

        if did_execute_action {
            println!("\n{}\n", "Watching for changes...".blue());
        }
    }

    Ok(())
}

/// Handle a file change event
fn handle_file_change(
    changed_path: &Path,
    root_path: &Path,
    extracted_folder_path: &Path,
    custom_xml_json_path: &Path,
    output_file_path: &String,
) {
    // * Normalize paths for comparison
    let normalized_changed = normalized_path(changed_path);
    let normalized_custom_xml = normalized_path(custom_xml_json_path);
    let normalized_extracted = normalized_path(extracted_folder_path);

    // * Check if it's the customXml.json file
    if normalized_changed == normalized_custom_xml {
        println!("{}", format!("{} changed!", CUSTOM_XML_FILE_NAME).yellow());
        let response = input!("Do you want to resync? (y/n - default: y): ");
        if response.to_lowercase() != "n" {
            match sync_custom_xml(root_path.to_str().unwrap()) {
                Ok(_) => {
                    println!("{}", "Resync completed successfully!".green());
                }
                Err(e) => {
                    println!("{}", format!("Resync failed: {}", e).red());
                }
            }
        } else {
            println!("{}", "Resync cancelled.".yellow());
        }
        return;
    }

    // * Check if it's a file in the extracted folder
    if normalized_changed.starts_with(&normalized_extracted) {
        println!(
            "{}",
            format!(
                "File in {} folder changed: {}",
                EXTRACTED_FOLDER_NAME,
                changed_path.display()
            )
            .yellow()
        );

        let response = input!("Do you want to rezip? (y/n - default: y): ");
        if response.to_lowercase() != "n" {
            let extracted_folder_path_str = extracted_folder_path.to_string_lossy().to_string();
            println!(
                "Rezipping from {} to {}...",
                extracted_folder_path_str, output_file_path
            );

            match rezip_folder(&extracted_folder_path_str, output_file_path) {
                Ok(_) => {
                    println!("{}", "Rezip completed successfully!".green());
                }
                Err(e) => {
                    println!("{}", format!("Rezip failed: {}", e).red());
                }
            }
        } else {
            println!("{}", "Rezip cancelled.".yellow());
        }
        return;
    }

    // File change is not supported
    println!(
        "{}",
        format!(
            "File change detected but not supported: {}",
            changed_path.display()
        )
        .yellow()
    );
}

/// Normalize a path for comparison (handle Windows/Unix path differences)
/// This is used to make sure the changed path is the same as the last used path
fn normalized_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use crate::utils::types::FIXTURE_FOLDER_PATH;

    use super::*;
    use std::env::temp_dir;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_normalized_path_with_existing_file() {
        // Create a temporary file
        let mut temp_file = temp_dir();
        temp_file.push(FIXTURE_FOLDER_PATH);
        temp_file.push("test_normalize.txt");

        let mut file = File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(b"test").expect("Failed to write");
        drop(file);

        // Test normalization of existing file
        let normalized = normalized_path(&temp_file);

        // Should return a canonicalized path (absolute)
        assert!(normalized.is_absolute());

        // Clean up
        let _ = std::fs::remove_file(&temp_file);
    }

    #[test]
    fn test_normalized_path_with_nonexistent_file() {
        // Test with a non-existent path
        let mut nonexistent_path = temp_dir();
        nonexistent_path.push(FIXTURE_FOLDER_PATH);
        nonexistent_path.push("nonexistent/path/file.txt");
        let normalized = normalized_path(Path::new(&nonexistent_path));

        // Should return the original path as PathBuf when canonicalize fails
        assert_eq!(normalized, nonexistent_path.to_path_buf());
    }

    #[test]
    fn test_normalized_path_with_relative_path() {
        // Create a temporary file in current directory for relative path test
        let mut temp_file = temp_dir();
        temp_file.push(FIXTURE_FOLDER_PATH);
        temp_file.push("test_normalize_relative.txt");
        let mut file = File::create(&temp_file).expect("Failed to create temp file");
        file.write_all(b"test").expect("Failed to write");
        drop(file);

        // Test with relative path
        let relative_path = Path::new(&temp_file);
        let normalized = normalized_path(relative_path);

        // Should return an absolute path when file exists
        assert!(normalized.is_absolute());

        // Clean up
        let _ = std::fs::remove_file(&temp_file);
    }

    #[test]
    fn test_normalized_path_with_directory() {
        // Create a temporary directory
        let mut temp_dir = temp_dir();
        temp_dir.push(FIXTURE_FOLDER_PATH);
        temp_dir.push("test_normalize_dir");
        std::fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

        // Test normalization of directory
        let normalized = normalized_path(&temp_dir);

        // Should return a canonicalized path (absolute)
        assert!(normalized.is_absolute());
        assert!(normalized.is_dir());

        // Clean up
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}

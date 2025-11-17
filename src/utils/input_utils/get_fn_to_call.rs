use crate::utils::{
    analyze_custom_xml::main::analyze_custom_xml_wrapper,
    file_watcher::main::watch_folder_wrapper,
    summarize::main::summarize_wrapper,
    sync_custom_xml::main::sync_custom_xml_wrapper,
    types::UserPreference,
    zip_utils::main::{extract_zip_wrapper, rezip_folder_wrapper},
};
use colored::Colorize;
use prompted::input;

pub fn get_fn_to_call() -> Result<(), &'static str> {
    let mut user_preference = UserPreference::new();

    // Beautiful feature menu with sections
    println!();
    println!(
        "{}",
        "╔═══════════════════════════════════════════════════════╗".bright_cyan()
    );
    println!(
        "{}",
        "║          Available Features - Select an Option        ║".bright_cyan()
    );
    println!(
        "{}",
        "╚═══════════════════════════════════════════════════════╝".bright_cyan()
    );
    println!();

    // File Operations Section
    println!("{}", "📁 File Operations".bright_yellow().bold());
    println!(
        "{}",
        "─────────────────────────────────────────────────────────".bright_yellow()
    );
    println!(
        "  {}  {}",
        "1.".bright_cyan().bold(),
        "Extract".bright_white().bold()
    );
    println!(
        "      {} Extract a Word file into its OOXML representation",
        "→".bright_blue()
    );
    println!();
    println!(
        "  {}  {}",
        "2.".bright_cyan().bold(),
        "Rezip".bright_white().bold()
    );
    println!(
        "      {} Re-zip an extracted folder back into a Word file",
        "→".bright_blue()
    );
    println!();

    // Analysis Section
    println!("{}", "🔍 Analysis & Inspection".bright_magenta().bold());
    println!(
        "{}",
        "─────────────────────────────────────────────────────────".bright_magenta()
    );
    println!(
        "  {}  {}",
        "3.".bright_cyan().bold(),
        "Summarize".bright_white().bold()
    );
    println!(
        "      {} Analyze and summarize file structure (images, customXML, etc.)",
        "→".bright_blue()
    );
    println!();
    println!(
        "  {}  {}",
        "4.".bright_cyan().bold(),
        "Analyze customXML".bright_white().bold()
    );
    println!(
        "      {} Analyze custom XML files embedded in the Word document",
        "→".bright_blue()
    );
    println!();

    // Advanced Section
    println!("{}", "⚙️  Advanced Features".bright_green().bold());
    println!(
        "{}",
        "─────────────────────────────────────────────────────────".bright_green()
    );
    println!(
        "  {}  {}",
        "5.".bright_cyan().bold(),
        "Sync customXML".bright_white().bold()
    );
    println!(
        "      {} Update custom XML files in the extracted folder",
        "→".bright_blue()
    );
    println!();
    println!(
        "  {}  {}",
        "6.".bright_cyan().bold(),
        "Watch folder".bright_white().bold()
    );
    println!(
        "      {} Monitor folder for file changes and auto-sync",
        "→".bright_blue()
    );
    println!();

    let prompt_text = format!(
        "{} Select feature (1-6) {} [last used: {}]: ",
        "┌─".bright_cyan(),
        "─┐".bright_cyan(),
        user_preference.last_used_feature
    );
    let mut input_feature = input!("{}", prompt_text);

    if input_feature.is_empty() {
        input_feature = user_preference.last_used_feature.clone();
    } else if input_feature != user_preference.last_used_feature {
        user_preference.save_last_used_feature(input_feature.clone());
    }

    match input_feature.as_str() {
        "1" => extract_zip_wrapper(&mut user_preference),
        "2" => rezip_folder_wrapper(&mut user_preference),
        "3" => summarize_wrapper(&mut user_preference),
        "4" => analyze_custom_xml_wrapper(&mut user_preference),
        "5" => sync_custom_xml_wrapper(&mut user_preference),
        "6" => watch_folder_wrapper(&mut user_preference),
        _ => return Err("Invalid feature"),
    }

    Ok(())
}

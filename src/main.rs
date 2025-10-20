mod utils;

use crate::utils::zip_utils::{extract_zip, rezip_folder};
use colored::Colorize;
use utils::input_utils::get_file_path_from_input;

fn main() {
    println!("\n{}\n", "--- Welcome to Word utils ---".blue());

    // TODO: show the options

    let file_path_info = get_file_path_from_input();
    file_path_info.print_info();

    // extract_zip(&file_path_info);
    rezip_folder(&file_path_info);
}

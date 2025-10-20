mod utils;

use colored::Colorize;
use utils::input_utils::get_file_path;

fn main() {
    println!("\n{}\n", "--- Welcome to Word utils ---".blue());

    // TODO: show the options

    let file_path_info = get_file_path();
    file_path_info.print_info();
}

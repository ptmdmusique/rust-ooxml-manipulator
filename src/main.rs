mod utils;

use utils::input_utils::get_file_path;

fn main() {
    let file_path = get_file_path();
    println!("File path: {}", file_path);
}

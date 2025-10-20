mod utils;

use utils::input_utils::get_file_path;

fn main() {
    let file_path_info = get_file_path();
    file_path_info.print_info();
}

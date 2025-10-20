mod utils;

use crate::utils::{input_utils::get_fn_to_call, print_utils::print_error_with_panic};
use colored::Colorize;

fn main() {
    println!("\n{}\n", "--- Welcome to Word utils ---".blue());

    match get_fn_to_call() {
        Ok(fn_to_call) => fn_to_call(),
        Err(e) => print_error_with_panic(e),
    }
}

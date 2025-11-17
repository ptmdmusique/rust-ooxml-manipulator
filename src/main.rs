mod utils;

use crate::utils::{
    input_utils::get_fn_to_call::get_fn_to_call, print_utils::print_error_with_panic,
};
use colored::Colorize;

fn main() {
    // Beautiful welcome banner
    println!();
    let border = "╔═══════════════════════════════════════════════════════╗".bright_cyan();
    let empty_line = "║                                                       ║".bright_cyan();
    let welcome_text = "✨  Welcome to Word Utils  ✨".bright_white().bold();
    // Manual centering: 51 chars total width, text is ~29 chars, so ~11 spaces on each side
    let welcome_line = format!("║{:^53}║", welcome_text).bright_cyan();
    let bottom_border = "╚═══════════════════════════════════════════════════════╝".bright_cyan();

    println!("{}", border);
    println!("{}", empty_line);
    println!("{}", welcome_line);
    println!("{}", empty_line);
    println!("{}", bottom_border);
    println!();

    match get_fn_to_call() {
        Ok(_) => (),
        Err(e) => print_error_with_panic(e),
    }
}

use colored::Colorize;

pub fn print_error_with_panic(message: &str) -> ! {
    panic!("{} {} {}", "!!! ERROR".red(), message, "!!!".red());
}
